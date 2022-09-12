/*
 * Author: Dylan Turner
 * Description: Handle kernel interrupts
 */

use core::{
    intrinsics::unreachable,
    arch::asm,
    mem::size_of
};
use crate::{
    io::{
        inb, outb
    }, terminal::{
        print_str, ForegroundColor, BackgroundColor, print_u64
    }
};

#[no_mangle]
pub static mut IDT: [IdtGate64; 256] = [
    IdtGate64 {
        offset_low: 0,
        selector: 0x10,
        ist: 0,
        types_attr: 0x8E,
        offset_mid: 0,
        offset_high: 0,
        zero: 0
    }; 256
];

// Make sure for IDT stuff that structs are layed out w/out optimization
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdtGate64 {
    offset_low: u16, // 2b
    selector: u16, // 2b
    ist: u8, // 1b
    types_attr: u8, // 1b
    offset_mid: u16, // 2b
    offset_high: u32, // 4b
    zero: u32 // 4b
} // Sum = 16 bytes

#[derive(Clone, Copy)]
#[repr(C, packed(2))]
pub struct IdtDescriptor {
    limit: u16,
    base: u64
}

#[no_mangle]
pub extern "C" fn isr1_handler() {
    print_u64(inb(0x60) as u64, ForegroundColor::White, BackgroundColor::Black);
    print_str("\n", ForegroundColor::White, BackgroundColor::Black);

    outb(0x20, 0x20);
    outb(0xA0, 0x20);
}

fn isr1() -> ! {
    unsafe {
        asm!(
            "mov rdi, rsp",
            "sub rsp, 8", // Align stack pointer
            "call isr1_handler",
            "add rsp, 8", // Undo stack alignment
            "pop rax",
            "iretq"
        );
        unreachable();
    }
}

pub fn idt_init() {
    let isr_ptr = isr1 as *const () as u64;
    for table in 0..256 {
        unsafe {
            IDT[table].offset_low = (isr_ptr & 0x000000000000FFFF) as u16;
            IDT[table].offset_mid = ((isr_ptr & 0x00000000FFFF0000) >> 16) as u16;
            IDT[table].offset_high = ((isr_ptr & 0xFFFFFFFF00000000) >> 32) as u32;

            // Probably not be needed
            IDT[table].zero = 0;
            IDT[table].ist = 0;
            IDT[table].selector = 0x08;
            IDT[table].types_attr = 0x8E;
        }
    }

    outb(0x21, 0xFD);
    outb(0xA1, 0xFF);

    unsafe {
        let ptr = IdtDescriptor {
            base: &IDT as *const _ as u64,
            limit: (size_of::<[IdtGate64; 256]>() - 1) as u16
        };
        asm!(
            "lidt [{}]",
            "sti",
            in(reg) &ptr, options(readonly, nostack, preserves_flags)
        );
    }
}

