mod inet;

use alloc::vec;
use core::cmp::min;
use core::convert::TryInto;
use core::ffi::c_void;
#[cfg(not(target_os = "wasi"))]
use core::mem::size_of;
use core::num::NonZeroU32;
use core::ptr::copy_nonoverlapping;
use core::slice;
use errno::{set_errno, Errno};
use libc::{c_int, c_uint, ssize_t};
use rustix::fd::{BorrowedFd, IntoRawFd};
use rustix::io;
use rustix::net::{
    AddressFamily, Ipv4Addr, Ipv6Addr, Protocol, RecvAncillaryBuffer, RecvFlags, RecvMsgReturn,
    SendAncillaryBuffer, SendFlags, Shutdown, SocketAddrAny, SocketAddrStorage, SocketFlags,
    SocketType,
};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn accept(
    fd: c_int,
    addr: *mut SocketAddrStorage,
    len: *mut libc::socklen_t,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::accept(fd, addr.cast(), len));

    match convert_res(rustix::net::acceptfrom(BorrowedFd::borrow_raw(fd))) {
        Some((accepted_fd, from)) => {
            if !addr.is_null() {
                encode_addr(from, addr, len);
            }
            accepted_fd.into_raw_fd()
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn accept4(
    fd: c_int,
    addr: *mut SocketAddrStorage,
    len: *mut libc::socklen_t,
    flags: c_int,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::accept4(fd, addr.cast(), len, flags));

    let flags = SocketFlags::from_bits(flags as _).unwrap();
    match convert_res(rustix::net::acceptfrom_with(
        BorrowedFd::borrow_raw(fd),
        flags,
    )) {
        Some((accepted_fd, from)) => {
            if !addr.is_null() {
                encode_addr(from, addr, len);
            }
            accepted_fd.into_raw_fd()
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn bind(
    sockfd: c_int,
    addr: *const SocketAddrStorage,
    len: libc::socklen_t,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::bind(sockfd, addr.cast(), len));

    let addr = match convert_res(decode_addr(addr, len)) {
        Some(addr) => addr,
        None => return -1,
    };
    match convert_res(match addr {
        SocketAddrAny::V4(v4) => rustix::net::bind_v4(BorrowedFd::borrow_raw(sockfd), &v4),
        SocketAddrAny::V6(v6) => rustix::net::bind_v6(BorrowedFd::borrow_raw(sockfd), &v6),
        SocketAddrAny::Unix(unix) => rustix::net::bind_unix(BorrowedFd::borrow_raw(sockfd), &unix),
        _ => panic!("unrecognized SocketAddrAny kind"),
    }) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn connect(
    sockfd: c_int,
    addr: *const SocketAddrStorage,
    len: libc::socklen_t,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::connect(sockfd, addr.cast(), len));

    let addr = match convert_res(decode_addr(addr, len)) {
        Some(addr) => addr,
        None => return -1,
    };
    match convert_res(match addr {
        SocketAddrAny::V4(v4) => rustix::net::connect_v4(BorrowedFd::borrow_raw(sockfd), &v4),
        SocketAddrAny::V6(v6) => rustix::net::connect_v6(BorrowedFd::borrow_raw(sockfd), &v6),
        SocketAddrAny::Unix(unix) => {
            rustix::net::connect_unix(BorrowedFd::borrow_raw(sockfd), &unix)
        }
        _ => panic!("unrecognized SocketAddrAny kind"),
    }) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn getpeername(
    fd: c_int,
    addr: *mut SocketAddrStorage,
    len: *mut libc::socklen_t,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::getpeername(fd, addr.cast(), len));

    match convert_res(rustix::net::getpeername(BorrowedFd::borrow_raw(fd))) {
        Some(from) => {
            encode_addr(from, addr, len);
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn getsockname(
    fd: c_int,
    addr: *mut SocketAddrStorage,
    len: *mut libc::socklen_t,
) -> c_int {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::getsockname(fd, addr.cast(), len));

    match convert_res(rustix::net::getsockname(BorrowedFd::borrow_raw(fd))) {
        Some(from) => {
            let encoded_len = from.write(addr);
            *len = encoded_len.try_into().unwrap();
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn getsockopt(
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *mut c_void,
    optlen: *mut libc::socklen_t,
) -> c_int {
    use core::time::Duration;
    use rustix::net::sockopt::{self, Timeout};

    unsafe fn write_bool(
        value: rustix::io::Result<bool>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        Ok(write(value? as c_uint, optval.cast::<c_uint>(), optlen))
    }

    unsafe fn write_i32(
        value: rustix::io::Result<i32>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        Ok(write(value?, optval.cast::<i32>(), optlen))
    }

    unsafe fn write_u64(
        value: rustix::io::Result<u64>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        Ok(write(value?, optval.cast::<u64>(), optlen))
    }

    unsafe fn write_linger(
        linger: rustix::io::Result<Option<Duration>>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        let linger = linger?;
        let linger = libc::linger {
            l_onoff: linger.is_some() as c_int,
            l_linger: linger.unwrap_or_default().as_secs() as c_int,
        };
        Ok(write(linger, optval.cast::<libc::linger>(), optlen))
    }

    unsafe fn write_ucred(
        cred: rustix::io::Result<rustix::net::UCred>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        let cred = cred?;
        let cred = libc::ucred {
            pid: cred.pid.as_raw_nonzero().get(),
            gid: cred.gid.as_raw(),
            uid: cred.uid.as_raw(),
        };
        Ok(write(cred, optval.cast::<libc::ucred>(), optlen))
    }

    unsafe fn write_timeval(
        value: rustix::io::Result<Option<Duration>>,
        optval: *mut c_void,
        optlen: *mut libc::socklen_t,
    ) -> rustix::io::Result<()> {
        let timeval = match value? {
            None => libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            Some(duration) => libc::timeval {
                tv_sec: duration
                    .as_secs()
                    .try_into()
                    .map_err(|_| rustix::io::Errno::OVERFLOW)?,
                tv_usec: duration.subsec_micros() as _,
            },
        };
        Ok(write(timeval, optval.cast::<libc::timeval>(), optlen))
    }

    unsafe fn write<T>(value: T, optval: *mut T, optlen: *mut libc::socklen_t) {
        *optlen = size_of::<T>().try_into().unwrap();
        optval.write(value)
    }

    libc!(libc::getsockopt(fd, level, optname, optval, optlen));

    let fd = BorrowedFd::borrow_raw(fd);
    let result = match level {
        libc::SOL_SOCKET => match optname {
            libc::SO_REUSEADDR => write_bool(sockopt::get_socket_reuseaddr(fd), optval, optlen),
            libc::SO_BROADCAST => write_bool(sockopt::get_socket_broadcast(fd), optval, optlen),
            libc::SO_LINGER => write_linger(sockopt::get_socket_linger(fd), optval, optlen),
            libc::SO_PASSCRED => write_bool(sockopt::get_socket_passcred(fd), optval, optlen),
            libc::SO_SNDTIMEO => write_timeval(
                sockopt::get_socket_timeout(fd, Timeout::Send),
                optval,
                optlen,
            ),
            libc::SO_RCVTIMEO => write_timeval(
                sockopt::get_socket_timeout(fd, Timeout::Recv),
                optval,
                optlen,
            ),
            libc::SO_ERROR => sockopt::get_socket_error(fd).map(|err| {
                write::<i32>(
                    match err {
                        Ok(()) => 0,
                        Err(errno) => errno.raw_os_error(),
                    },
                    optval.cast::<i32>(),
                    optlen,
                )
            }),
            libc::SO_KEEPALIVE => write_bool(sockopt::get_socket_keepalive(fd), optval, optlen),
            libc::SO_TYPE => write_i32(
                sockopt::get_socket_type(fd).map(|ty| ty.as_raw() as i32),
                optval,
                optlen,
            ),
            libc::SO_SNDBUF => write_i32(
                sockopt::get_socket_send_buffer_size(fd).map(|size| size as i32),
                optval,
                optlen,
            ),
            libc::SO_RCVBUF => write_i32(
                sockopt::get_socket_recv_buffer_size(fd).map(|size| size as i32),
                optval,
                optlen,
            ),
            libc::SO_OOBINLINE => write_bool(sockopt::get_socket_oobinline(fd), optval, optlen),
            libc::SO_DOMAIN => write_i32(
                sockopt::get_socket_domain(fd).map(|domain| domain.as_raw().into()),
                optval,
                optlen,
            ),
            libc::SO_ACCEPTCONN => write_bool(sockopt::get_socket_acceptconn(fd), optval, optlen),
            libc::SO_REUSEPORT => write_bool(sockopt::get_socket_reuseport(fd), optval, optlen),
            libc::SO_PROTOCOL => write_i32(
                sockopt::get_socket_protocol(fd).map(|protocol| match protocol {
                    None => 0,
                    Some(protocol) => protocol.as_raw().get() as i32,
                }),
                optval,
                optlen,
            ),
            libc::SO_COOKIE => write_u64(sockopt::get_socket_cookie(fd), optval, optlen),
            libc::SO_INCOMING_CPU => write_i32(
                sockopt::get_socket_incoming_cpu(fd).map(|cpu| cpu as i32),
                optval,
                optlen,
            ),
            libc::SO_PEERCRED => write_ucred(sockopt::get_socket_peercred(fd), optval, optlen),
            _ => unimplemented!("unimplemented getsockopt SOL_SOCKET optname {:?}", optname),
        },
        libc::IPPROTO_IP => match optname {
            libc::IP_TTL => write_i32(
                sockopt::get_ip_ttl(fd).map(|ttl| ttl as i32),
                optval,
                optlen,
            ),
            libc::IP_MULTICAST_LOOP => {
                write_bool(sockopt::get_ip_multicast_loop(fd), optval, optlen)
            }
            libc::IP_MULTICAST_TTL => write_i32(
                sockopt::get_ip_multicast_ttl(fd).map(|ttl| ttl as i32),
                optval,
                optlen,
            ),
            libc::IP_TOS => write_i32(sockopt::get_ip_tos(fd).map(Into::into), optval, optlen),
            libc::IP_RECVTOS => write_bool(sockopt::get_ip_recvtos(fd), optval, optlen),
            libc::IP_FREEBIND => write_bool(sockopt::get_ip_freebind(fd), optval, optlen),
            libc::SO_ORIGINAL_DST => match sockopt::get_ip_original_dst(fd) {
                Ok(addr) => {
                    assert!(*optlen >= size_of::<SocketAddrStorage>().try_into().unwrap());
                    let len = SocketAddrAny::V4(addr).write(optval.cast());
                    *optlen = len.try_into().unwrap();
                    Ok(())
                }
                Err(err) => Err(err),
            },
            _ => unimplemented!("unimplemented getsockopt IPPROTO_IP optname {:?}", optname),
        },
        libc::IPPROTO_IPV6 => match optname {
            libc::IPV6_MULTICAST_LOOP => {
                write_bool(sockopt::get_ipv6_multicast_loop(fd), optval, optlen)
            }
            libc::IPV6_V6ONLY => write_bool(sockopt::get_ipv6_v6only(fd), optval, optlen),
            libc::IPV6_UNICAST_HOPS => write_i32(
                sockopt::get_ipv6_unicast_hops(fd).map(Into::into),
                optval,
                optlen,
            ),
            libc::IPV6_RECVTCLASS => write_bool(sockopt::get_ipv6_recvtclass(fd), optval, optlen),
            libc::IPV6_FREEBIND => write_bool(sockopt::get_ipv6_freebind(fd), optval, optlen),
            libc::IP6T_SO_ORIGINAL_DST => match sockopt::get_ipv6_original_dst(fd) {
                Ok(addr) => {
                    assert!(*optlen >= size_of::<SocketAddrStorage>().try_into().unwrap());
                    let len = SocketAddrAny::V6(addr).write(optval.cast());
                    *optlen = len.try_into().unwrap();
                    Ok(())
                }
                Err(err) => Err(err),
            },
            libc::IPV6_TCLASS => write_i32(
                sockopt::get_ipv6_tclass(fd).map(|value| value as i32),
                optval,
                optlen,
            ),
            _ => unimplemented!(
                "unimplemented getsockopt IPPROTO_IPV6 optname {:?}",
                optname
            ),
        },
        libc::IPPROTO_TCP => match optname {
            libc::TCP_NODELAY => write_bool(sockopt::get_tcp_nodelay(fd), optval, optlen),
            libc::TCP_KEEPIDLE => write_i32(
                sockopt::get_tcp_keepidle(fd)
                    .map(|duration| min(duration.as_secs(), i32::MAX as u64) as i32),
                optval,
                optlen,
            ),
            libc::TCP_USER_TIMEOUT => write_i32(
                sockopt::get_tcp_user_timeout(fd).map(|value| value as i32),
                optval,
                optlen,
            ),
            libc::TCP_KEEPINTVL => write_i32(
                sockopt::get_tcp_keepintvl(fd)
                    .map(|duration| min(duration.as_secs(), i32::MAX as u64) as i32),
                optval,
                optlen,
            ),
            libc::TCP_KEEPCNT => write_i32(
                sockopt::get_tcp_keepcnt(fd).map(|value| value as i32),
                optval,
                optlen,
            ),
            libc::TCP_QUICKACK => write_bool(sockopt::get_tcp_quickack(fd), optval, optlen),
            libc::TCP_CONGESTION => sockopt::get_tcp_congestion(fd).map(|name| {
                let len = core::cmp::min(*optlen as usize, name.len());
                core::ptr::copy_nonoverlapping(name.as_ptr(), optval.cast::<u8>(), len);
                *optlen = len as libc::socklen_t;
            }),
            libc::TCP_THIN_LINEAR_TIMEOUTS => {
                write_bool(sockopt::get_tcp_thin_linear_timeouts(fd), optval, optlen)
            }
            libc::TCP_CORK => write_bool(sockopt::get_tcp_cork(fd), optval, optlen),
            _ => unimplemented!("unimplemented getsockopt IPPROTO_TCP optname {:?}", optname),
        },
        _ => unimplemented!(
            "unimplemented getsockopt level {:?} optname {:?}",
            level,
            optname
        ),
    };
    match convert_res(result) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn setsockopt(
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: libc::socklen_t,
) -> c_int {
    use core::time::Duration;
    use rustix::net::sockopt::{self, Timeout};

    unsafe fn read_bool(optval: *const c_void, optlen: libc::socklen_t) -> bool {
        read(optval.cast::<c_int>(), optlen) != 0
    }

    unsafe fn read_u32(optval: *const c_void, optlen: libc::socklen_t) -> u32 {
        read(optval.cast::<u32>(), optlen)
    }

    unsafe fn read_i32(optval: *const c_void, optlen: libc::socklen_t) -> i32 {
        read(optval.cast::<i32>(), optlen)
    }

    unsafe fn read_linger(optval: *const c_void, optlen: libc::socklen_t) -> Option<Duration> {
        let linger = read(optval.cast::<libc::linger>(), optlen);
        (linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64))
    }

    unsafe fn read_timeval(optval: *const c_void, optlen: libc::socklen_t) -> Option<Duration> {
        let timeval = read(optval.cast::<libc::timeval>(), optlen);
        if timeval.tv_sec == 0 && timeval.tv_usec == 0 {
            None
        } else {
            Some(
                Duration::from_secs(timeval.tv_sec.try_into().unwrap())
                    + Duration::from_micros(timeval.tv_usec as _),
            )
        }
    }

    unsafe fn read_ip_mreq(optval: *const c_void, optlen: libc::socklen_t) -> libc::ip_mreq {
        read(optval.cast::<libc::ip_mreq>(), optlen)
    }

    unsafe fn read_ip_mreqn(optval: *const c_void, optlen: libc::socklen_t) -> libc::ip_mreqn {
        read(optval.cast::<libc::ip_mreqn>(), optlen)
    }

    unsafe fn read_ip_mreq_source(
        optval: *const c_void,
        optlen: libc::socklen_t,
    ) -> libc::ip_mreq_source {
        read(optval.cast::<libc::ip_mreq_source>(), optlen)
    }

    unsafe fn read_ipv6_multiaddr(optval: *const c_void, optlen: libc::socklen_t) -> Ipv6Addr {
        Ipv6Addr::from(
            read(optval.cast::<libc::ipv6_mreq>(), optlen)
                .ipv6mr_multiaddr
                .s6_addr,
        )
    }

    unsafe fn read_ipv6_interface(optval: *const c_void, optlen: libc::socklen_t) -> u32 {
        read(optval.cast::<libc::ipv6_mreq>(), optlen).ipv6mr_interface
    }

    unsafe fn read<T>(optval: *const T, optlen: libc::socklen_t) -> T {
        assert_eq!(optlen, size_of::<T>().try_into().unwrap());
        optval.read()
    }

    libc!(libc::setsockopt(fd, level, optname, optval, optlen));

    let fd = BorrowedFd::borrow_raw(fd);
    let result = match level {
        libc::SOL_SOCKET => match optname {
            libc::SO_REUSEADDR => sockopt::set_socket_reuseaddr(fd, read_bool(optval, optlen)),
            libc::SO_BROADCAST => sockopt::set_socket_broadcast(fd, read_bool(optval, optlen)),
            libc::SO_LINGER => sockopt::set_socket_linger(fd, read_linger(optval, optlen)),
            libc::SO_PASSCRED => sockopt::set_socket_passcred(fd, read_bool(optval, optlen)),
            libc::SO_SNDTIMEO => {
                sockopt::set_socket_timeout(fd, Timeout::Send, read_timeval(optval, optlen))
            }
            libc::SO_RCVTIMEO => {
                sockopt::set_socket_timeout(fd, Timeout::Recv, read_timeval(optval, optlen))
            }
            libc::SO_KEEPALIVE => sockopt::set_socket_keepalive(fd, read_bool(optval, optlen)),
            libc::SO_SNDBUF => {
                let size = read_i32(optval, optlen);
                if size < 0 {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
                sockopt::set_socket_send_buffer_size(fd, size as usize)
            }
            libc::SO_RCVBUF => {
                let size = read_i32(optval, optlen);
                if size < 0 {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
                sockopt::set_socket_recv_buffer_size(fd, size as usize)
            }
            libc::SO_OOBINLINE => sockopt::set_socket_oobinline(fd, read_bool(optval, optlen)),
            libc::SO_REUSEPORT => sockopt::set_socket_reuseport(fd, read_bool(optval, optlen)),
            libc::SO_INCOMING_CPU => sockopt::set_socket_incoming_cpu(fd, read_u32(optval, optlen)),
            _ => unimplemented!("unimplemented setsockopt SOL_SOCKET optname {:?}", optname),
        },
        libc::IPPROTO_IP => match optname {
            libc::IP_TTL => sockopt::set_ip_ttl(fd, read_i32(optval, optlen) as u32),
            libc::IP_MULTICAST_LOOP => {
                sockopt::set_ip_multicast_loop(fd, read_bool(optval, optlen))
            }
            libc::IP_MULTICAST_TTL => {
                sockopt::set_ip_multicast_ttl(fd, read_i32(optval, optlen) as u32)
            }
            libc::IP_ADD_MEMBERSHIP => {
                if optlen as usize == size_of::<libc::ip_mreq>() {
                    let mreq = read_ip_mreq(optval, optlen);
                    let multiaddr = Ipv4Addr::from(mreq.imr_multiaddr.s_addr.to_ne_bytes());
                    let interface = Ipv4Addr::from(mreq.imr_interface.s_addr.to_ne_bytes());
                    sockopt::set_ip_add_membership(fd, &multiaddr, &interface)
                } else {
                    let mreqn = read_ip_mreqn(optval, optlen);
                    let multiaddr = Ipv4Addr::from(mreqn.imr_multiaddr.s_addr.to_ne_bytes());
                    let address = Ipv4Addr::from(mreqn.imr_address.s_addr.to_ne_bytes());
                    let ifindex = mreqn.imr_ifindex;
                    sockopt::set_ip_add_membership_with_ifindex(fd, &multiaddr, &address, ifindex)
                }
            }
            libc::IP_DROP_MEMBERSHIP => {
                if optlen as usize == size_of::<libc::ip_mreq>() {
                    let mreq = read_ip_mreq(optval, optlen);
                    let multiaddr = Ipv4Addr::from(mreq.imr_multiaddr.s_addr.to_ne_bytes());
                    let interface = Ipv4Addr::from(mreq.imr_interface.s_addr.to_ne_bytes());
                    sockopt::set_ip_drop_membership(fd, &multiaddr, &interface)
                } else {
                    let mreqn = read_ip_mreqn(optval, optlen);
                    let multiaddr = Ipv4Addr::from(mreqn.imr_multiaddr.s_addr.to_ne_bytes());
                    let address = Ipv4Addr::from(mreqn.imr_address.s_addr.to_ne_bytes());
                    let ifindex = mreqn.imr_ifindex;
                    sockopt::set_ip_drop_membership_with_ifindex(fd, &multiaddr, &address, ifindex)
                }
            }
            libc::IP_ADD_SOURCE_MEMBERSHIP => {
                let mreq = read_ip_mreq_source(optval, optlen);
                let multiaddr = Ipv4Addr::from(mreq.imr_multiaddr.s_addr.to_ne_bytes());
                let interface = Ipv4Addr::from(mreq.imr_interface.s_addr.to_ne_bytes());
                let sourceaddr = Ipv4Addr::from(mreq.imr_sourceaddr.s_addr.to_ne_bytes());
                sockopt::set_ip_add_source_membership(fd, &multiaddr, &interface, &sourceaddr)
            }
            libc::IP_DROP_SOURCE_MEMBERSHIP => {
                let mreq = read_ip_mreq_source(optval, optlen);
                let multiaddr = Ipv4Addr::from(mreq.imr_multiaddr.s_addr.to_ne_bytes());
                let interface = Ipv4Addr::from(mreq.imr_interface.s_addr.to_ne_bytes());
                let sourceaddr = Ipv4Addr::from(mreq.imr_sourceaddr.s_addr.to_ne_bytes());
                sockopt::set_ip_drop_source_membership(fd, &multiaddr, &interface, &sourceaddr)
            }
            libc::IP_TOS => {
                let value = read_i32(optval, optlen);
                let value = match value.try_into() {
                    Ok(value) => value,
                    Err(_) => {
                        set_errno(Errno(libc::EINVAL));
                        return -1;
                    }
                };
                sockopt::set_ip_tos(fd, value)
            }
            libc::IP_RECVTOS => sockopt::set_ip_recvtos(fd, read_bool(optval, optlen)),
            libc::IP_FREEBIND => sockopt::set_ip_freebind(fd, read_bool(optval, optlen)),
            _ => unimplemented!("unimplemented setsockopt IPPROTO_IP optname {:?}", optname),
        },
        libc::IPPROTO_IPV6 => match optname {
            libc::IPV6_MULTICAST_LOOP => {
                sockopt::set_ipv6_multicast_loop(fd, read_bool(optval, optlen))
            }
            libc::IPV6_ADD_MEMBERSHIP => sockopt::set_ipv6_add_membership(
                fd,
                &read_ipv6_multiaddr(optval, optlen),
                read_ipv6_interface(optval, optlen),
            ),
            libc::IPV6_DROP_MEMBERSHIP => sockopt::set_ipv6_drop_membership(
                fd,
                &read_ipv6_multiaddr(optval, optlen),
                read_ipv6_interface(optval, optlen),
            ),
            libc::IPV6_V6ONLY => sockopt::set_ipv6_v6only(fd, read_bool(optval, optlen)),
            libc::IPV6_UNICAST_HOPS => {
                let hops = read_i32(optval, optlen);
                let hops = if hops == -1 {
                    None
                } else {
                    match hops.try_into() {
                        Ok(hops) => Some(hops),
                        Err(_) => {
                            set_errno(Errno(libc::EINVAL));
                            return -1;
                        }
                    }
                };
                sockopt::set_ipv6_unicast_hops(fd, hops)
            }
            libc::IPV6_RECVTCLASS => sockopt::set_ipv6_recvtclass(fd, read_bool(optval, optlen)),
            libc::IPV6_FREEBIND => sockopt::set_ipv6_freebind(fd, read_bool(optval, optlen)),
            libc::IPV6_TCLASS => sockopt::set_ipv6_tclass(fd, read_u32(optval, optlen)),
            _ => unimplemented!(
                "unimplemented setsockopt IPPROTO_IPV6 optname {:?}",
                optname
            ),
        },
        libc::IPPROTO_TCP => match optname {
            libc::TCP_NODELAY => sockopt::set_tcp_nodelay(fd, read_bool(optval, optlen)),
            libc::TCP_KEEPIDLE => {
                let secs = read_i32(optval, optlen);
                if secs < 0 {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
                sockopt::set_tcp_keepidle(fd, Duration::new(secs as u64, 0))
            }
            libc::TCP_USER_TIMEOUT => sockopt::set_tcp_user_timeout(fd, read_u32(optval, optlen)),
            libc::TCP_KEEPINTVL => {
                let secs = read_i32(optval, optlen);
                if secs < 0 {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
                sockopt::set_tcp_keepintvl(fd, Duration::new(secs as u64, 0))
            }
            libc::TCP_KEEPCNT => {
                let value = read_i32(optval, optlen);
                if value < 0 {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
                sockopt::set_tcp_keepcnt(fd, value as u32)
            }
            libc::TCP_QUICKACK => sockopt::set_tcp_quickack(fd, read_bool(optval, optlen)),
            libc::TCP_CONGESTION => {
                let name = core::str::from_utf8(core::slice::from_raw_parts(
                    optval.cast::<u8>(),
                    optlen as usize,
                ))
                .unwrap();
                sockopt::set_tcp_congestion(fd, name)
            }
            libc::TCP_THIN_LINEAR_TIMEOUTS => {
                sockopt::set_tcp_thin_linear_timeouts(fd, read_bool(optval, optlen))
            }
            libc::TCP_CORK => sockopt::set_tcp_cork(fd, read_bool(optval, optlen)),
            _ => unimplemented!("unimplemented setsockopt IPPROTO_TCP optname {:?}", optname),
        },
        _ => unimplemented!(
            "unimplemented setsockopt level {:?} optname {:?}",
            level,
            optname
        ),
    };
    match convert_res(result) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn listen(fd: c_int, backlog: c_int) -> c_int {
    libc!(libc::listen(fd, backlog));

    match convert_res(rustix::net::listen(BorrowedFd::borrow_raw(fd), backlog)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn recv(fd: c_int, ptr: *mut c_void, len: usize, flags: c_int) -> isize {
    libc!(libc::recv(fd, ptr, len, flags));

    let flags = RecvFlags::from_bits(flags as _).unwrap();

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    let mut tmp = vec![0u8; len];
    match convert_res(rustix::net::recv(
        BorrowedFd::borrow_raw(fd),
        &mut tmp,
        flags,
    )) {
        Some(nread) => {
            copy_nonoverlapping(tmp.as_ptr(), ptr.cast::<u8>(), len);
            nread as isize
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn recvfrom(
    fd: c_int,
    buf: *mut c_void,
    len: usize,
    flags: c_int,
    from: *mut SocketAddrStorage,
    from_len: *mut libc::socklen_t,
) -> isize {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::recvfrom(fd, buf, len, flags, from.cast(), from_len));

    let flags = RecvFlags::from_bits(flags as _).unwrap();

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    let mut tmp = vec![0u8; len];
    match convert_res(rustix::net::recvfrom(
        BorrowedFd::borrow_raw(fd),
        &mut tmp,
        flags,
    )) {
        Some((nread, addr)) => {
            copy_nonoverlapping(tmp.as_ptr(), buf.cast::<u8>(), len);
            if let Some(addr) = addr {
                let encoded_len = addr.write(from);
                *from_len = encoded_len.try_into().unwrap();
            }
            nread as isize
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn recvmsg(sockfd: c_int, msg: *mut libc::msghdr, flags: c_int) -> ssize_t {
    libc!(libc::recvmsg(sockfd, msg, flags));

    let msg = &mut *msg;

    let fd = BorrowedFd::borrow_raw(sockfd);
    let flags = RecvFlags::from_bits(flags as _).unwrap();
    let mut ancillaries = RecvAncillaryBuffer::default();

    match convert_res(rustix::net::recvmsg(
        fd,
        slice::from_raw_parts_mut(msg.msg_iov.cast(), msg.msg_iovlen as usize),
        &mut ancillaries,
        flags,
    )) {
        Some(RecvMsgReturn {
            bytes,
            flags,
            address,
        }) => {
            for _ancillary in ancillaries.drain() {
                todo!("recvmsg ancillary messages");
            }
            msg.msg_flags = flags.bits() as _;
            if !msg.msg_name.is_null() {
                if let Some(address) = address {
                    msg.msg_namelen = address.write(msg.msg_name.cast()) as _;
                } else {
                    msg.msg_namelen = 0;
                }
            }
            bytes as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize {
    libc!(libc::send(fd, buf, len, flags));

    let flags = SendFlags::from_bits(flags as _).unwrap();
    match convert_res(rustix::net::send(
        BorrowedFd::borrow_raw(fd),
        slice::from_raw_parts(buf.cast::<u8>(), len),
        flags,
    )) {
        Some(nwritten) => nwritten as isize,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sendto(
    fd: c_int,
    buf: *const c_void,
    len: usize,
    flags: c_int,
    to: *const SocketAddrStorage,
    to_len: libc::socklen_t,
) -> isize {
    // We don't use `checked_cast` here because libc uses `sockaddr` which
    // just represents the header of the struct, not the full storage.
    libc!(libc::sendto(fd, buf, len, flags, to.cast(), to_len));

    let flags = SendFlags::from_bits(flags as _).unwrap();
    let addr = match convert_res(decode_addr(to, to_len)) {
        Some(addr) => addr,
        None => return -1,
    };
    match convert_res(match addr {
        SocketAddrAny::V4(v4) => rustix::net::sendto_v4(
            BorrowedFd::borrow_raw(fd),
            slice::from_raw_parts(buf.cast::<u8>(), len),
            flags,
            &v4,
        ),
        SocketAddrAny::V6(v6) => rustix::net::sendto_v6(
            BorrowedFd::borrow_raw(fd),
            slice::from_raw_parts(buf.cast::<u8>(), len),
            flags,
            &v6,
        ),
        SocketAddrAny::Unix(unix) => rustix::net::sendto_unix(
            BorrowedFd::borrow_raw(fd),
            slice::from_raw_parts(buf.cast::<u8>(), len),
            flags,
            &unix,
        ),
        _ => panic!("unrecognized SocketAddrAny kind"),
    }) {
        Some(nwritten) => nwritten as isize,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sendmsg(sockfd: c_int, msg: *const libc::msghdr, flags: c_int) -> ssize_t {
    libc!(libc::sendmsg(sockfd, msg, flags));

    let msg = &*msg;

    let fd = BorrowedFd::borrow_raw(sockfd);
    let flags = SendFlags::from_bits(flags as _).unwrap();
    let mut addr = None;

    if !msg.msg_name.is_null() {
        addr = match convert_res(decode_addr(
            msg.msg_name.cast::<SocketAddrStorage>(),
            msg.msg_namelen,
        )) {
            Some(addr) => Some(addr),
            None => return -1,
        }
    }
    if msg.msg_controllen != 0 {
        todo!("sendmsg ancillary messages");
    }

    match convert_res(rustix::net::sendmsg_any(
        fd,
        addr.as_ref(),
        slice::from_raw_parts_mut(msg.msg_iov.cast(), msg.msg_iovlen as usize),
        &mut SendAncillaryBuffer::default(),
        flags,
    )) {
        Some(num) => num.try_into().unwrap(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn shutdown(fd: c_int, how: c_int) -> c_int {
    libc!(libc::shutdown(fd, how));

    let how = match how {
        libc::SHUT_RD => Shutdown::Read,
        libc::SHUT_WR => Shutdown::Write,
        libc::SHUT_RDWR => Shutdown::ReadWrite,
        _ => panic!("unrecognized shutdown kind {}", how),
    };
    match convert_res(rustix::net::shutdown(BorrowedFd::borrow_raw(fd), how)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    libc!(libc::socket(domain, type_, protocol));

    let domain = AddressFamily::from_raw(domain as _);
    let flags = SocketFlags::from_bits_truncate(type_ as _);
    let type_ = SocketType::from_raw(type_ as u32 & !SocketFlags::all().bits());
    let protocol = if let Some(protocol) = NonZeroU32::new(protocol as u32) {
        Some(Protocol::from_raw(protocol))
    } else {
        None
    };
    match convert_res(rustix::net::socket_with(domain, type_, flags, protocol)) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn socketpair(
    domain: c_int,
    type_: c_int,
    protocol: c_int,
    sv: *mut [c_int; 2],
) -> c_int {
    libc!(libc::socketpair(
        domain,
        type_,
        protocol,
        (*sv).as_mut_ptr()
    ));

    let domain = AddressFamily::from_raw(domain as _);
    let flags = SocketFlags::from_bits_truncate(type_ as _);
    let type_ = SocketType::from_raw(type_ as u32 & !SocketFlags::all().bits());
    let protocol = if let Some(protocol) = NonZeroU32::new(protocol as u32) {
        Some(Protocol::from_raw(protocol))
    } else {
        None
    };
    match convert_res(rustix::net::socketpair(domain, type_, flags, protocol)) {
        Some((fd0, fd1)) => {
            (*sv) = [fd0.into_raw_fd(), fd1.into_raw_fd()];
            0
        }
        None => -1,
    }
}

unsafe fn encode_addr(
    from: Option<SocketAddrAny>,
    addr: *mut SocketAddrStorage,
    len: *mut libc::socklen_t,
) {
    if let Some(from) = from {
        let encoded_len = from.write(addr);
        *len = encoded_len.try_into().unwrap();
    } else {
        (*addr)
            .__storage
            .__bindgen_anon_1
            .__bindgen_anon_1
            .ss_family = libc::AF_UNSPEC as _;
        *len = size_of::<libc::sa_family_t>() as _;
    }
}

unsafe fn decode_addr(
    addr: *const SocketAddrStorage,
    mut len: libc::socklen_t,
) -> io::Result<SocketAddrAny> {
    // There's unfortunately code out there which forgets to add 1 to the
    // `len` for the NUL terminator. Detect this and fix it.
    if addr.cast::<libc::sockaddr>().read_unaligned().sa_family == libc::AF_UNIX as _ {
        let sun_path = &(*addr.cast::<libc::sockaddr_un>()).sun_path;
        if sun_path[len as usize - offsetof_sun_path()] == 0 {
            len += 1;
        }
    }

    SocketAddrAny::read(addr, len.try_into().unwrap())
}

#[inline]
fn offsetof_sun_path() -> usize {
    let z = libc::sockaddr_un {
        sun_family: 0_u16,
        sun_path: [0; 108],
    };
    (core::ptr::addr_of!(z.sun_path).addr()) - (core::ptr::addr_of!(z).addr())
}
