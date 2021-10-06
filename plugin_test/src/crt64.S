.section .init, "ax"
.global _start

_start:
    /* Jump to the absolute address defined by the linker script. */
    // for 64bit
.option push
.option norelax // to prevent an unsupported R_RISCV_ALIGN relocation from being generated
1:
    auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  3
1:
    .dword _abs_start
.option pop

_abs_start:
    .cfi_startproc
    .cfi_undefined ra

    csrw mie, 0
    csrw mip, 0

    li  x1, 0
    li  x2, 0
    li  x3, 0
    li  x4, 0
    li  x5, 0
    li  x6, 0
    li  x7, 0
    li  x8, 0
    li  x9, 0
    li  x10,0
    li  x11,0
    li  x12,0
    li  x13,0
    li  x14,0
    li  x15,0
    li  x16,0
    li  x17,0
    li  x18,0
    li  x19,0
    li  x20,0
    li  x21,0
    li  x22,0
    li  x23,0
    li  x24,0
    li  x25,0
    li  x26,0
    li  x27,0
    li  x28,0
    li  x29,0
    li  x30,0
    li  x31,0

    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    // Check hart id
    //csrr a2, mhartid
    //lui t0, %hi(_max_hart_id)
    //add t0, t0, %lo(_max_hart_id)
    //bgtu a2, t0, abort

    // Allocate stacks
    la sp, _stack_start
    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)
//#ifdef __riscv_mul
    // mul t0, a2, t0 // needed in multicore
//#else
//    beqz a2, 2f  // Jump if single-hart
//    mv t1, a2
//    mv t2, t0
//1:
//    add t0, t0, t2
//    addi t1, t1, -1
//    bnez t1, 1b
//2:
//#endif
    sub sp, sp, t0

    // Set frame pointer
    add s0, sp, zero

    jal zero, _start_rust

    .cfi_endproc

/*
    Trap entry point (_start_trap)

    Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
    restores caller saved registers and then returns.
*/
.section .trap, "ax"
.global _start_trap
/* Make it .weak so PAC/HAL can provide their own if needed. */
.weak _start_trap

_start_trap:
    addi sp, sp, -16*8

    sd ra, 0*8(sp)
    sd t0, 1*8(sp)
    sd t1, 2*8(sp)
    sd t2, 3*8(sp)
    sd t3, 4*8(sp)
    sd t4, 5*8(sp)
    sd t5, 6*8(sp)
    sd t6, 7*8(sp)
    sd a0, 8*8(sp)
    sd a1, 9*8(sp)
    sd a2, 10*8(sp)
    sd a3, 11*8(sp)
    sd a4, 12*8(sp)
    sd a5, 13*8(sp)
    sd a6, 14*8(sp)
    sd a7, 15*8(sp)

    add a0, sp, zero
    jal ra, _start_trap_rust

    ld ra, 0*8(sp)
    ld t0, 1*8(sp)
    ld t1, 2*8(sp)
    ld t2, 3*8(sp)
    ld t3, 4*8(sp)
    ld t4, 5*8(sp)
    ld t5, 6*8(sp)
    ld t6, 7*8(sp)
    ld a0, 8*8(sp)
    ld a1, 9*8(sp)
    ld a2, 10*8(sp)
    ld a3, 11*8(sp)
    ld a4, 12*8(sp)
    ld a5, 13*8(sp)
    ld a6, 14*8(sp)
    ld a7, 15*8(sp)

    addi sp, sp, 16*8
    mret

.section .text
.global default_setup_interrupts

default_setup_interrupts:
    // Set trap handler
    la t0, _start_trap
    csrw mtvec, t0
    ret

/* Make sure there is an abort when linking */
/* .globl abort
abort:
    j abort*/

.section ".tohost","aw",@progbits
.align 6
.globl tohost
tohost: .dword 0
.align 6
.globl fromhost
fromhost: .dword 0

