global_asm!(include_str!("crt64.S"));

pub mod c_externs {
    extern "C" {
        pub static mut tohost: usize;
    }
}

const SIGABRT: i32 = 6;

fn tohost_exit(code: i32) -> ! {
    unsafe { c_externs::tohost = (code << 1) as usize | 1 };
    loop {}
}

pub fn exit(code: i32) -> ! {
    tohost_exit(code);
}

#[export_name = "abort"]
pub extern "C" fn abort() {
    exit(128 + SIGABRT);
}

const UART_BASE: usize = 0x1000_0000;

pub fn putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(UART_BASE as *mut u8, c);
    }
}

pub fn printstr(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}

/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust() {
    #[rustfmt::skip]
    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        // fn main() -> !;
        fn main();

        // This symbol will be provided by the user via `#[pre_init]`
        // fn __pre_init();

        fn _setup_interrupts();

        // fn _mp_hook() -> bool;
    }

    // if _mp_hook() {
    //     __pre_init();

    //     r0::zero_bss(&mut _sbss, &mut _ebss);
    //     r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    // }

    // TODO: Enable FPU when available

    _setup_interrupts();

    main();
    exit(0)
}

/// Registers saved in trap handler
#[allow(missing_docs)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultExceptionHandler(trap_frame: &TrapFrame) -> ! {
    tohost_exit(666);
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultInterruptHandler() {
    tohost_exit(1337);
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

/* Interrupts */
// #[doc(hidden)]
// pub enum Interrupt {
//     UserSoft,
//     SupervisorSoft,
//     MachineSoft,
//     UserTimer,
//     SupervisorTimer,
//     MachineTimer,
//     UserExternal,
//     SupervisorExternal,
//     MachineExternal,
// }

// pub use self::Interrupt as interrupt;

// extern "C" {
//     fn UserSoft();
//     fn SupervisorSoft();
//     fn MachineSoft();
//     fn UserTimer();
//     fn SupervisorTimer();
//     fn MachineTimer();
//     fn UserExternal();
//     fn SupervisorExternal();
//     fn MachineExternal();
// }

// #[doc(hidden)]
// pub union Vector {
//     handler: unsafe extern "C" fn(),
//     reserved: usize,
// }

// #[doc(hidden)]
// #[no_mangle]
// pub static __INTERRUPTS: [Vector; 12] = [
//     Vector { handler: UserSoft },
//     Vector { handler: SupervisorSoft },
//     Vector { reserved: 0 },
//     Vector { handler: MachineSoft },
//     Vector { handler: UserTimer },
//     Vector { handler: SupervisorTimer },
//     Vector { reserved: 0 },
//     Vector { handler: MachineTimer },
//     Vector { handler: UserExternal },
//     Vector { handler: SupervisorExternal },
//     Vector { reserved: 0 },
//     Vector { handler: MachineExternal },
// ];

/// Trap entry point rust (_start_trap_rust)
///
/// `mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and ExceptionHandler
/// or one of the core interrupt handlers is called.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust(trap_frame: *const TrapFrame) {
    // extern "C" {
    //     fn ExceptionHandler(trap_frame: &TrapFrame);
    //     fn DefaultHandler();
    // }

    // unsafe {
    //     let cause = mcause::read();
    //     if cause.is_exception() {
    //         ExceptionHandler(&*trap_frame)
    //     } else {
    //         let code = cause.code();
    //         if code < __INTERRUPTS.len() {
    //             let h = &__INTERRUPTS[code];
    //             if h.reserved == 0 {
    //                 DefaultHandler();
    //             } else {
    //                 (h.handler)();
    //             }
    //         } else {
    //             DefaultHandler();
    //         }
    //     }
    // }
    loop {}
}
