use errno::{set_errno, Errno};
use libc::c_int;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn reboot(cmd: c_int) -> c_int {
    libc!(libc::reboot(cmd));

    let arg = match cmd {
        libc::LINUX_REBOOT_CMD_CAD_OFF => rustix::system::RebootCommand::CadOff,
        libc::LINUX_REBOOT_CMD_CAD_ON => rustix::system::RebootCommand::CadOn,
        libc::LINUX_REBOOT_CMD_HALT => rustix::system::RebootCommand::Halt,
        libc::LINUX_REBOOT_CMD_KEXEC => rustix::system::RebootCommand::Kexec,
        libc::LINUX_REBOOT_CMD_POWER_OFF => rustix::system::RebootCommand::PowerOff,
        libc::LINUX_REBOOT_CMD_RESTART => rustix::system::RebootCommand::Restart,
        libc::LINUX_REBOOT_CMD_SW_SUSPEND => rustix::system::RebootCommand::SwSuspend,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    match convert_res(rustix::system::reboot(arg)) {
        Some(()) => 0,
        None => -1,
    }
}
