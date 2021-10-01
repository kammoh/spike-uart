#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

extern crate panic_halt;
extern crate riscv_rt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    unsafe {
        // asm!("li t0, 0x10000000");
        // asm!("lw t1, 0(t0)");
        // asm!("sw t1, 0(t0)");
        // Write the value 1 to tohost, telling Spike to quit with an exit code of 0.
        // asm!("li t0, 1");
        // asm!("la t1, tohost");
        // asm!("sw t0, 0(t1)");

        // Spin until Spike terminates the run.
        // asm!("j 1b");
    }
    loop {}
}


// global_asm!(
//     ".align 3",
//     ".global tohost",
//     "tohost:",
//     ".dword 0",
//     ".align 3",
//     ".global fromhost",
//     "fromhost:",
//     ".dword 0"
// );
