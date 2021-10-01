#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

extern crate panic_halt;

#[no_mangle]
fn _start() -> ! {
    unsafe {
        asm!("li t0, 0x10000000");
        asm!("lw t1, 0(t0)");
        asm!("sw t1, 0(t0)");
        // end simulation
        asm!("li t0, 1");
        asm!("la t1, tohost");
        asm!("sw t0, 0(t1)");
    }
    loop {}
}

global_asm!(
    ".align 3",
    ".global tohost",
    "tohost:",
    ".dword 0",
    ".align 3",
    ".global fromhost",
    "fromhost:",
    ".dword 0"
);
