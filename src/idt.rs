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
        inb, outb, remap_pic
    }, terminal::{
        Color, print_char, print_str, backspace
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

// Keyboard scan codes; TODO: Move to user space
const SCANCODE_TABLE: [char; 88] = [
    '\0', // Nothing
    '\0', // Escape - 0x01
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
    '\0', // Backspace - 0x0E
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    '\0', // Left control - 0x1D
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',  ';', '\'', '`',
    '\0', // Left shift - 0x2A
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', '.', '/',
    '\0', // Right shift - 0x36
    '*',
    '\0', // Left alt - 0x38
    ' ',
    '\0', // Caps lock - 0x3A
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', // F1 (0x3B) -> F10 (0x44)
    '\0', // Num lock - 0x45
    '\0', // Scroll lock - 0x46

    // Keypad numerals. Default to num lock on
    '7', '8', '9', '-',
    '4', '5', '6', '+',
    '1', '2', '3', '0', '.',
    
    '\0', '\0', '\0', // 0x54,55,56 - Nothing!
    '\0', '\0' // 0x57, 0x58 - F11, F12
    
    // Everything else up to 0x8X (releases) is nothing
    // Then later there's more special funcs
];

// TODO: Move to user space
fn keyboard_handler(code: u8, c: char, is_rel: bool) {
    if c != '\0' && !is_rel {
        print_char(c, Color::Black, Color::White);    
    } else if !is_rel {
        match code {
            0x01 => print_str("<ESC>", Color::Blue, Color::White),
            0x0E => backspace(),
            0x1D => print_str("<LCTRL>", Color::Blue, Color::White),
            0x2A => print_str("<LSHIFT>", Color::Blue, Color::White),
            0x36 => print_str("<RSHIFT>", Color::Blue, Color::White),
            0x38 => print_str("<LALT>", Color::Blue, Color::White),
            0x3A => print_str("<CPSLK>", Color::Blue, Color::White),
            0x3B => print_str("<F1>", Color::Blue, Color::White),
            0x3C => print_str("<F2>", Color::Blue, Color::White),
            0x3D => print_str("<F3>", Color::Blue, Color::White),
            0x3E => print_str("<F4>", Color::Blue, Color::White),
            0x3F => print_str("<F5>", Color::Blue, Color::White),
            0x40 => print_str("<F6>", Color::Blue, Color::White),
            0x41 => print_str("<F7>", Color::Blue, Color::White),
            0x42 => print_str("<F8>", Color::Blue, Color::White),
            0x43 => print_str("<F9>", Color::Blue, Color::White),
            0x44 => print_str("<F10>", Color::Blue, Color::White),
            0x45 => print_str("<NMLK>", Color::Blue, Color::White),
            0x46 => print_str("<SCRLK>", Color::Blue, Color::White),
            0x57 => print_str("<F11>", Color::Blue, Color::White),
            0x48 => print_str("<F12>", Color::Blue, Color::White),
            _ => {} // Not handled
        }
    }
}

// Keyboard interrupt handler
#[no_mangle]
pub extern "C" fn isr1_handler() {
    let code = inb(0x60);
    let mut c = '\0';
    if (code as usize) < SCANCODE_TABLE.len() {
        c = SCANCODE_TABLE[code as usize];
    } /*else if (code & 0x80) > 0 && ((code & (!0x80)) as usize) < SCANCODE_TABLE.len() {
        c = SCANCODE_TABLE[(code & (!0x80)) as usize];
    }*/
    let rel = false;//(code | 0x80) > 0;
    keyboard_handler(code, c, rel); // TODO: call user land code somehow

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

    // Enable just keyboard. All others cause crash
    unsafe {
        IDT[1].offset_low = (isr_ptr & 0x000000000000FFFF) as u16;
        IDT[1].offset_mid = ((isr_ptr & 0x00000000FFFF0000) >> 16) as u16;
        IDT[1].offset_high = ((isr_ptr & 0xFFFFFFFF00000000) >> 32) as u32;

        // Probably not be needed
        IDT[1].zero = 0;
        IDT[1].ist = 0;
        IDT[1].selector = 0x08;
        IDT[1].types_attr = 0x8E;
    }

    remap_pic();

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

