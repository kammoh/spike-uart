#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use macros::entry;

extern crate macros;

extern crate panic_halt;

mod rt;

use rt::*;

#[entry]
fn main() -> ! {
    printstr("Hello, world!\n");
    exit(0);
}

// const UART_BASE: *mut u8 = 0x1000_0000  as *mut u8;

// fn putchar(c: u8) {
//     unsafe {
//         // (UART_BASE as *mut u8).offset(0).volatile.write(c).volatile();
//         core::ptr::write_volatile(UART_BASE, c);
//     }
//     // unsafe{

//     //     asm!("li t1, 0x61");
//     //     asm!("li t0, 0x10000000");
//     //     // // asm!("lw t1, 0(t0)");
//     //     asm!("sb t1, 0(t0)");
//     // }
// }

// #[no_mangle]
// fn ff() {
//     // unsafe {
//     //     asm!("li t0, 0x10000000");
//     //     asm!("li t1, 0x61");
//     //     asm!("sb t1, 0(t0)");
//     // }
// }

// #[no_mangle]
// fn _start() -> ! {

//     // putchar(0x63);
//     // unsafe{

//     //     asm!("li t1, 0x61");
//     //     asm!("li t0, 0x10000000");
//     //     // // asm!("lw t1, 0(t0)");
//     //     asm!("sb t1, 0(t0)");
//     // }
//     // putchar(0x62);
//     // putchar(0x63);
//     ff();

//     unsafe {
//         asm!("li t0, 0x10000000");
//         asm!("li t1, 0x61");
//         asm!("sb t1, 0(t0)");

//         asm!("li t1, 0x3");
//         asm!("sb t1, 0(t0)");
//         // end simulation
//         asm!("li t1, 1");
//         asm!("la t0, tohost");
//         asm!("sw t1, 0(t0)");
//     }
//     loop {}
// }

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
