/*
 * Author: Dylan Turner
 * Description: Handle io operations to the CPU
 */

use core::arch::asm;

const PIC1_CMD: u16 = 0x0020;
const PIC1_DATA: u16 = 0x0021;
const PIC2_CMD: u16 = 0x00A0;
const PIC2_DATA: u16 = 0x00A1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

pub fn remap_pic() {
    // Save masks
    let a1 = inb(PIC1_DATA);
    let a2 = inb(PIC2_DATA);

    outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4); // Tell master to start initialization
    outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
    outb(PIC1_DATA, 0);
    outb(PIC2_DATA, 8);
    outb(PIC1_DATA, 4);
    outb(PIC2_DATA, 2);
    outb(PIC1_DATA, ICW4_8086);
    outb(PIC2_DATA, ICW4_8086);

    // Restore saved masks
    outb(PIC1_DATA, a1);
    outb(PIC2_DATA, a2);
}

pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val
        );
    }
}

pub fn inb(port: u16) -> u8 {
    let mut val: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") val
        );
    }
    val
}
