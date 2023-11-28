//! `malloc`/`free`/etc. functions.
//!
//! TODO: Use `alloc_zeroed` and `realloc` instead of doing the work
//! ourselves, which might let allocators be more efficient.

#[cfg(not(target_arch = "riscv64"))]
use core::mem::align_of;
use core::mem::size_of;
use core::ptr::{copy_nonoverlapping, null_mut, write_bytes};
use errno::{set_errno, Errno};
use libc::{c_int, c_void, size_t};

// Decide which underlying allocator to use.

#[cfg(not(any(
    feature = "malloc-via-rust-global-alloc",
    feature = "malloc-via-crates"
)))]
compile_error!("One of the malloc implementation features must be enabled.");

#[cfg(feature = "malloc-via-rust-global-alloc")]
use alloc::alloc::{alloc as the_alloc, dealloc as the_dealloc};

#[cfg(feature = "malloc-via-crates")]
unsafe fn the_alloc(layout: alloc::alloc::Layout) -> *mut u8 {
    core::alloc::GlobalAlloc::alloc(&rustix_dlmalloc::GlobalDlmalloc, layout)
}
#[cfg(feature = "malloc-via-crates")]
unsafe fn the_dealloc(ptr: *mut u8, layout: alloc::alloc::Layout) {
    core::alloc::GlobalAlloc::dealloc(&rustix_dlmalloc::GlobalDlmalloc, ptr, layout)
}

/// Rust's `alloc` API requires the user to pass in the old size and alignment
/// for resizing and deallocation, while C's `malloc` API doesn't, so we store
/// the size and alignment next to the allocation memory.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Tag {
    size: usize,
    align: usize,
}

/// Allocate for a given layout, with a tag prepended to the allocation, to
/// keep track of said layout.
///
/// Return null if the allocation failed.
fn tagged_alloc(type_layout: alloc::alloc::Layout) -> *mut u8 {
    if type_layout.size() == 0 {
        return null_mut();
    }

    let tag_layout = alloc::alloc::Layout::new::<Tag>();
    let tag = Tag {
        size: type_layout.size(),
        align: type_layout.align(),
    };
    if let Ok((total_layout, offset)) = tag_layout.extend(type_layout) {
        let total_ptr = unsafe { the_alloc(total_layout) };
        if total_ptr.is_null() {
            return total_ptr;
        }

        let tag_offset = offset - tag_layout.size();
        unsafe {
            total_ptr.wrapping_add(tag_offset).cast::<Tag>().write(tag);
            total_ptr.wrapping_add(offset).cast()
        }
    } else {
        null_mut()
    }
}

/// Get the layout out of a tagged allocation.
///
/// # Safety
///
/// The given pointer must be a non-null pointer that was returned from
/// `tagged_alloc`.
unsafe fn get_layout(ptr: *mut u8) -> alloc::alloc::Layout {
    let tag = ptr.wrapping_sub(size_of::<Tag>()).cast::<Tag>().read();
    alloc::alloc::Layout::from_size_align_unchecked(tag.size, tag.align)
}

/// get the layout out of a tagged allocation
///
/// # Safety
///
/// The given pointer must be a non-null pointer that was returned from
/// `tagged_alloc`.
unsafe fn tagged_dealloc(ptr: *mut u8) {
    let tag_layout = alloc::alloc::Layout::new::<Tag>();
    let type_layout = get_layout(ptr);
    if let Ok((total_layout, offset)) = tag_layout.extend(type_layout) {
        let total_ptr = ptr.wrapping_sub(offset);
        the_dealloc(total_ptr, total_layout);
    }
}

#[linkage = "weak"]
#[no_mangle]
unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    libc!(libc::malloc(size));

    // If we're asked to allocate zero bytes, actually allocate 1 byte, so
    // that we can return a non-NULL pointer. Technically the `malloc`
    // spec says we can return NULL in this case, but popular code in the
    // wild interprets NULL as an allocation failure.
    let size = if size == 0 { size + 1 } else { size };

    // TODO: Add `max_align_t` for riscv64 to upstream libc.
    #[cfg(target_arch = "riscv64")]
    let layout = alloc::alloc::Layout::from_size_align(size, 16);
    #[cfg(not(target_arch = "riscv64"))]
    let layout = alloc::alloc::Layout::from_size_align(size, align_of::<libc::max_align_t>());

    let layout = match layout {
        Ok(layout) => layout,
        Err(_) => {
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }
    };

    let ret = tagged_alloc(layout);
    if ret.is_null() {
        set_errno(Errno(libc::ENOMEM));
    }
    ret.cast()
}

#[linkage = "weak"]
#[no_mangle]
unsafe extern "C" fn realloc(old: *mut c_void, size: usize) -> *mut c_void {
    libc!(libc::realloc(old, size));

    if old.is_null() {
        malloc(size)
    } else {
        let old_layout = get_layout(old.cast());
        if old_layout.size() >= size {
            return old;
        }

        let new = malloc(size);

        if !new.is_null() {
            copy_nonoverlapping(
                old.cast::<u8>(),
                new.cast::<u8>(),
                core::cmp::min(size, old_layout.size()),
            );
        }
        tagged_dealloc(old.cast());
        new
    }
}

#[no_mangle]
unsafe extern "C" fn reallocarray(old: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void {
    libc!(libc::reallocarray(old, nmemb, size));

    let product = match nmemb.checked_mul(size) {
        Some(product) => product,
        None => {
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }
    };

    realloc(old, product)
}

#[linkage = "weak"]
#[no_mangle]
unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    libc!(libc::calloc(nmemb, size));

    let product = match nmemb.checked_mul(size) {
        Some(product) => product,
        None => {
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }
    };

    let ptr = malloc(product);
    write_bytes(ptr, 0, product);
    ptr
}

#[no_mangle]
unsafe extern "C" fn posix_memalign(
    memptr: *mut *mut c_void,
    alignment: usize,
    size: usize,
) -> c_int {
    libc!(libc::posix_memalign(memptr, alignment, size));

    if !alignment.is_power_of_two() || alignment < core::mem::size_of::<*const c_void>() {
        return libc::EINVAL;
    }

    let layout = alloc::alloc::Layout::from_size_align(size, alignment);
    let layout = match layout {
        Ok(layout) => layout,
        Err(_) => return libc::ENOMEM,
    };

    let ptr = tagged_alloc(layout);
    if ptr.is_null() {
        return libc::ENOMEM;
    }

    *memptr = ptr.cast();
    0
}

#[deprecated]
#[no_mangle]
unsafe extern "C" fn memalign(alignment: usize, size: usize) -> *mut c_void {
    libc!(libc::memalign(alignment, size));

    let layout = alloc::alloc::Layout::from_size_align(size, alignment);
    let layout = match layout {
        Ok(layout) => layout,
        Err(_) => {
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }
    };

    let ptr = tagged_alloc(layout);
    if ptr.is_null() {
        set_errno(Errno(libc::ENOMEM));
        return null_mut();
    }

    ptr.cast()
}

#[linkage = "weak"]
#[no_mangle]
unsafe extern "C" fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void {
    //libc!(libc::aligned_alloc(alignment, size));

    if !alignment.is_power_of_two() || size % alignment != 0 {
        set_errno(Errno(libc::EINVAL));
        return null_mut();
    }

    let layout = alloc::alloc::Layout::from_size_align(size, alignment).unwrap();
    let ptr = tagged_alloc(layout);
    if ptr.is_null() {
        set_errno(Errno(libc::ENOMEM));
        return null_mut();
    }

    ptr.cast()
}

#[deprecated]
#[allow(deprecated)]
#[no_mangle]
unsafe extern "C" fn valloc(size: size_t) -> *mut c_void {
    //libc!(libc::valloc(size));

    memalign(rustix::param::page_size(), size)
}

#[linkage = "weak"]
#[no_mangle]
unsafe extern "C" fn free(ptr: *mut c_void) {
    libc!(libc::free(ptr));

    if ptr.is_null() {
        return;
    }

    tagged_dealloc(ptr.cast());
}

#[no_mangle]
unsafe extern "C" fn malloc_usable_size(ptr: *mut c_void) -> size_t {
    libc!(libc::malloc_usable_size(ptr));

    if ptr.is_null() {
        return 0;
    }

    get_layout(ptr.cast()).size()
}
