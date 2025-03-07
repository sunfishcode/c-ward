use core::arch::naked_asm;
use core::mem::size_of;
use libc::{c_int, c_void};
use rustix::runtime::{How, KernelSigSet};

#[allow(non_camel_case_types)]
type jmp_buf = *mut c_void;
#[allow(non_camel_case_types)]
type sigjmp_buf = *mut c_void;

#[no_mangle]
#[cfg_attr(
    any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    ),
    naked
)]
unsafe extern "C" fn setjmp(env: jmp_buf) -> c_int {
    //libc!(libc::setjmp(env));

    #[cfg(target_arch = "aarch64")]
    {
        naked_asm!(
            // Save all the callee-saved registers, the incoming stack pointer
            // value, and the incoming return address into the `jmp_buf`.
            "stp x19, x20, [x0,#0]",
            "stp x21, x22, [x0,#16]",
            "stp x23, x24, [x0,#32]",
            "stp x25, x26, [x0,#48]",
            "stp x27, x28, [x0,#64]",
            "stp x29, x30, [x0,#80]",
            "stp d8, d9, [x0,#96]",
            "stp d10, d11, [x0,#112]",
            "stp d12, d13, [x0,#128]",
            "stp d14, d15, [x0,#144]",
            "mov x1, sp",
            "str x1, [x0,#160]",
            // Return 0.
            "mov x0, #0",
            // Return to the caller normally.
            "ret"
        )
    }

    #[cfg(all(target_arch = "riscv64", target_feature = "soft-float"))]
    {
        naked_asm!(
            // Save all the callee-saved registers, the incoming stack pointer
            // value, and the incoming return address into the `jmp_buf`.
            "sd s0, 0(a0)",
            "sd s1, 8(a0)",
            "sd s2, 16(a0)",
            "sd s3, 24(a0)",
            "sd s4, 32(a0)",
            "sd s5, 40(a0)",
            "sd s6, 48(a0)",
            "sd s7, 56(a0)",
            "sd s8, 64(a0)",
            "sd s9, 72(a0)",
            "sd s10, 80(a0)",
            "sd s11, 88(a0)",
            "sd sp, 96(a0)",
            "sd ra, 104(a0)",
            // Soft-float mode; don't save the floating-point registers.

            // Return 0.
            "li a0, 0",
            // Return to the caller normally.
            "ret"
        )
    }

    #[cfg(all(target_arch = "riscv64", not(target_feature = "soft-float")))]
    {
        naked_asm!(
            // arch option manipulation needed due to LLVM/Rust bug, see rust-lang/rust#80608
            ".option push",
            ".option arch, +d",
            // Save all the callee-saved registers, the incoming stack pointer
            // value, and the incoming return address into the `jmp_buf`.
            "sd s0, 0(a0)",
            "sd s1, 8(a0)",
            "sd s2, 16(a0)",
            "sd s3, 24(a0)",
            "sd s4, 32(a0)",
            "sd s5, 40(a0)",
            "sd s6, 48(a0)",
            "sd s7, 56(a0)",
            "sd s8, 64(a0)",
            "sd s9, 72(a0)",
            "sd s10, 80(a0)",
            "sd s11, 88(a0)",
            "sd sp, 96(a0)",
            "sd ra, 104(a0)",
            // Hard-float mode; save the floating-point registers.
            "fsd fs0, 112(a0)",
            "fsd fs1, 120(a0)",
            "fsd fs2, 128(a0)",
            "fsd fs3, 136(a0)",
            "fsd fs4, 144(a0)",
            "fsd fs5, 152(a0)",
            "fsd fs6, 160(a0)",
            "fsd fs7, 168(a0)",
            "fsd fs8, 176(a0)",
            "fsd fs9, 184(a0)",
            "fsd fs10, 192(a0)",
            "fsd fs11, 200(a0)",
            // Return 0.
            "li a0, 0",
            // Return to the caller normally.
            "ret",
            // arch option manipulation needed due to LLVM/Rust bug, see rust-lang/rust#80608
            ".option pop"
        )
    }

    #[cfg(target_arch = "x86_64")]
    {
        naked_asm!(
            // Load the incoming return address.
            "mov rsi, [rsp]",
            // Save all the callee-saved registers, the incoming stack pointer
            // value, and the incoming return address into the `jmp_buf`.
            "mov [rdi], rbx",
            "mov [rdi+8], rbp",
            "mov [rdi+16], r12",
            "mov [rdi+24], r13",
            "mov [rdi+32], r14",
            "mov [rdi+40], r15",
            "mov [rdi+48], rsp",
            "mov [rdi+56], rsi",
            // Return 0.
            "xor eax, eax",
            // Return to the caller normally.
            "ret"
        )
    }

    #[cfg(target_arch = "x86")]
    {
        naked_asm!(
            // Load the `jmp_buf` address.
            "mov eax, [esp+4]",
            // Load the incoming return address.
            "mov ecx, [esp]",
            // Save all the callee-saved registers, the incoming stack pointer
            // value, and the incoming return address into the `jmp_buf`.
            "mov [eax], ebx",
            "mov [eax+4], esi",
            "mov [eax+8], edi",
            "mov [eax+12], ebp",
            "mov [eax+16], esp",
            "mov [eax+20], ecx",
            // Return 0.
            "xor eax, eax",
            // Return to the caller normally.
            "ret"
        )
    }

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    )))]
    {
        // We don't support `longjmp` yet, so just do the first-time return of 0.
        0
    }
}

core::arch::global_asm!(".globl _setjmp", ".set _setjmp, setjmp");

#[no_mangle]
#[cfg_attr(
    any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    ),
    naked
)]
unsafe extern "C" fn longjmp(env: jmp_buf, val: c_int) -> ! {
    //libc!(libc::longjmp(env, val));

    #[cfg(target_arch = "aarch64")]
    {
        naked_asm!(
            // Restore the callee-saved registers and the stack pointer.
            "ldp x19, x20, [x0,#0]",
            "ldp x21, x22, [x0,#16]",
            "ldp x23, x24, [x0,#32]",
            "ldp x25, x26, [x0,#48]",
            "ldp x27, x28, [x0,#64]",
            "ldp x29, x30, [x0,#80]",
            "ldp d8, d9, [x0,#96]",
            "ldp d10, d11, [x0,#112]",
            "ldp d12, d13, [x0,#128]",
            "ldp d14, d15, [x0,#144]",
            "ldr x2, [x0,#160]",
            "mov sp, x2",
            // Return `val == 0 ? 1 : val`.
            "cmp w1, 0",
            "csinc w0, w1, wzr, ne",
            // Jump to the `setjmp`'s return address.
            "br x30"
        )
    }

    #[cfg(all(target_arch = "riscv64", target_feature = "soft-float"))]
    {
        naked_asm!(
            // Restore the callee-saved registers and the stack pointer.
            "ld s0, 0(a0)",
            "ld s1, 8(a0)",
            "ld s2, 16(a0)",
            "ld s3, 24(a0)",
            "ld s4, 32(a0)",
            "ld s5, 40(a0)",
            "ld s6, 48(a0)",
            "ld s7, 56(a0)",
            "ld s8, 64(a0)",
            "ld s9, 72(a0)",
            "ld s10, 80(a0)",
            "ld s11, 88(a0)",
            "ld sp, 96(a0)",
            "ld ra, 104(a0)",
            // Soft-float mode; don't restore the floating-point registers.

            // Return `val == 0 ? 1 : val`.
            "seqz a0, a1",
            "add a0, a0, a1",
            // Jump to the `setjmp`'s return address.
            "ret"
        );
    }

    #[cfg(all(target_arch = "riscv64", not(target_feature = "soft-float")))]
    {
        naked_asm!(
            // arch option manipulation needed due to LLVM/Rust bug, see rust-lang/rust#80608
            ".option push",
            ".option arch, +d",
            // Restore the callee-saved registers and the stack pointer.
            "ld s0, 0(a0)",
            "ld s1, 8(a0)",
            "ld s2, 16(a0)",
            "ld s3, 24(a0)",
            "ld s4, 32(a0)",
            "ld s5, 40(a0)",
            "ld s6, 48(a0)",
            "ld s7, 56(a0)",
            "ld s8, 64(a0)",
            "ld s9, 72(a0)",
            "ld s10, 80(a0)",
            "ld s11, 88(a0)",
            "ld sp, 96(a0)",
            "ld ra, 104(a0)",
            // Hard-float mode; restore the floating-point registers.
            "fld fs0, 112(a0)",
            "fld fs1, 120(a0)",
            "fld fs2, 128(a0)",
            "fld fs3, 136(a0)",
            "fld fs4, 144(a0)",
            "fld fs5, 152(a0)",
            "fld fs6, 160(a0)",
            "fld fs7, 168(a0)",
            "fld fs8, 176(a0)",
            "fld fs9, 184(a0)",
            "fld fs10, 192(a0)",
            "fld fs11, 200(a0)",
            // Return `val == 0 ? 1 : val`.
            "seqz a0, a1",
            "add a0, a0, a1",
            // Jump to the `setjmp`'s return address.
            "ret",
            // arch option manipulation needed due to LLVM/Rust bug, see rust-lang/rust#80608
            ".option pop"
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        naked_asm!(
            // Restore the callee-saved registers and the stack pointer.
            "mov rbx, [rdi]",
            "mov rbp, [rdi+8]",
            "mov r12, [rdi+16]",
            "mov r13, [rdi+24]",
            "mov r14, [rdi+32]",
            "mov r15, [rdi+40]",
            "mov rsp, [rdi+48]",
            // "Pop" the `setjmp` return address from the stack pointer.
            "add rsp, 8",
            // Return `val == 0 ? 1 : val`.
            "xor eax, eax",
            "cmp esi, 1",
            "adc eax, esi",
            // Jump to the `setjmp`'s return address.
            "jmp [rdi+56]"
        )
    }

    #[cfg(target_arch = "x86")]
    {
        naked_asm!(
            // Load the `jmp_buf` address.
            "mov ecx, [esp+4]",
            // Load `val`.
            "mov eax, [esp+8]",
            // Restore the callee-saved registers and the stack pointer.
            "mov ebx, [ecx]",
            "mov esi, [ecx+4]",
            "mov edi, [ecx+8]",
            "mov ebp, [ecx+12]",
            "mov esp, [ecx+16]",
            // "Pop" the `setjmp` return address from the stack pointer.
            "add esp, 4",
            // Return `val == 0 ? 1 : val`.
            "cmp eax, 1",
            "adc eax, 0",
            // Jump to the `setjmp`'s return address.
            "jmp [ecx+20]"
        );
    }

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    )))]
    {
        todo!("longjmp")
    }
}

core::arch::global_asm!(".globl _longjmp", ".set _longjmp, longjmp");

#[no_mangle]
#[cfg_attr(
    any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    ),
    naked
)]
unsafe extern "C" fn sigsetjmp(_env: sigjmp_buf, _savesigs: c_int) -> c_int {
    //libc!(libc::sigsetjmp(env, savesigs));

    // Call `__sigsetjmp_save`, and then tail-call `setjmp` so that it sees
    // the original return value.

    #[cfg(target_arch = "aarch64")]
    {
        naked_asm!(
            "stp x29, x30, [sp, -16]!",
            "mov x29, sp",
            "bl {__sigsetjmp_save}",
            "ldp x29, x30, [sp], 16",
            "b {setjmp}",
            __sigsetjmp_save = sym __sigsetjmp_save,
            setjmp = sym setjmp
        )
    }

    #[cfg(target_arch = "riscv64")]
    {
        naked_asm!(
            "addi sp, sp, -16",
            "sd ra, 8(sp)",
            "call {__sigsetjmp_save}",
            "ld ra, 8(sp)",
            "addi sp, sp, 16",
            "tail {setjmp}",
            __sigsetjmp_save = sym __sigsetjmp_save,
            setjmp = sym setjmp
        )
    }

    #[cfg(target_arch = "x86_64")]
    {
        naked_asm!(
            "push rbp",
            "mov rbp, rsp",
            "call {__sigsetjmp_save}",
            "mov rdi, rax",
            "pop rbp",
            "jmp {setjmp}",
            __sigsetjmp_save = sym __sigsetjmp_save,
            setjmp = sym setjmp
        )
    }

    #[cfg(target_arch = "x86")]
    {
        naked_asm!(
            "sub esp, 20",
            "push [esp+28]",
            "push [esp+28]",
            "call {__sigsetjmp_save}",
            "mov [esp+32], eax",
            "add esp, 28",
            "jmp {setjmp}",
            __sigsetjmp_save = sym __sigsetjmp_save,
            setjmp = sym setjmp
        )
    }

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "x86_64",
        target_arch = "x86"
    )))]
    {
        // As in `setjmp`, just do the first-time return.
        0
    }
}

core::arch::global_asm!(".globl __sigsetjmp", ".set __sigsetjmp, sigsetjmp");

// The offset from the start of `jmp_buf` to memory that `setjmp` does not
// store to, so `sigsetjmp` can store to it.
#[cfg(target_arch = "aarch64")]
const SIG_OFFSET: usize = 168;
#[cfg(target_arch = "riscv64")]
const SIG_OFFSET: usize = 208;
#[cfg(target_arch = "x86_64")]
const SIG_OFFSET: usize = 64;
#[cfg(target_arch = "x86")]
const SIG_OFFSET: usize = 24;

#[no_mangle]
unsafe extern "C" fn __sigsetjmp_save(env: sigjmp_buf, savesigs: c_int) -> sigjmp_buf {
    // Save `savesigs` so that `siglongjmp` can test it too.
    env.byte_add(SIG_OFFSET).cast::<c_int>().write(savesigs);

    if savesigs != 0 {
        // Save the signal set at the offset after `savesigs`.
        let set = &mut *env
            .byte_add(SIG_OFFSET + size_of::<usize>())
            .cast::<KernelSigSet>();
        *set = rustix::runtime::kernel_sigprocmask(How::SETMASK, None).unwrap();
    }

    // Return the `env` value so that the assembly code doesn't have to save
    // and restore it manually.
    env
}

#[no_mangle]
unsafe extern "C" fn siglongjmp(env: sigjmp_buf, val: c_int) -> ! {
    //libc!(libc::siglongjmp(env, val));

    // Load the saved `savesigs` value.
    let savesigs = env.byte_add(SIG_OFFSET).cast::<c_int>().read();

    if savesigs != 0 {
        // Restore the signal set from the offset after `savesigs`.
        let set = &*env
            .byte_add(SIG_OFFSET + size_of::<usize>())
            .cast::<KernelSigSet>();
        rustix::runtime::kernel_sigprocmask(How::SETMASK, Some(set)).ok();
    }

    // Call `longjmp` to do the actual jump.
    longjmp(env.cast(), val)
}
