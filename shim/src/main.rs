#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]
#![deny(clippy::all)]

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "mov rax, 0x0000000A6D696873", // "shim\n"
        "push rax",

        "mov rax, {sys_write}",
        "mov rdi, {fd}",
        "mov rsi, rsp",
        "mov rdx, 5",
        "syscall",

        "mov rax, {sys_exit}",
        "mov rdi, {status}",
        "syscall",

        sys_write = const 1,
        sys_exit = const 60,
        status = const 0,
        fd = const 1,
        options(noreturn)
    )
}
