/*
 * Author: Dylan Turner
 * Description: Handle text-mode printing. Used by all system modules
 */

use crate::io::outb;

const VGA_MEMORY: u64 = 0xB8000;
const VGA_WIDTH: u8 = 80;
const VGA_HEIGHT: u8 = 25;

static mut CURSOR_X: u8 = 0;
static mut CURSOR_Y: u8 = 0;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ForegroundColor {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    White = 0x0F
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BackgroundColor {
    Black = 0x00,
    Blue = 0x10,
    Green = 0x20,
    Cyan = 0x30,
    Red = 0x40,
    Magenta = 0x50,
    Brown = 0x60,
    LightGray = 0x70,
    DarkGray = 0x80,
    LightBlue = 0x90,
    LightGreen = 0xA0,
    LightCyan = 0xB0,
    LightRed = 0xC0,
    LightMagenta = 0xD0,
    Yellow = 0xE0,
    White = 0xF0
}

pub fn get_cursor_pos() -> (u8, u8) {
    unsafe {
        (CURSOR_X, CURSOR_Y)
    }
}

pub fn set_cursor_pos(x: u8, y: u8) {
    let pos = y as u16 * VGA_WIDTH as u16 + x as u16;

    // Actually update the cursor pos:
    outb(0x3D4, 0x0F);
    outb(0x3D5, (pos & 0x00FF) as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, (pos >> 8) as u8);

    // Then update our copy
    unsafe {
        CURSOR_X = x;
        CURSOR_Y = y;
    }
}

// Could probably optimize to set 4 chars at a time
pub fn print_str(msg: &str, fg: ForegroundColor, bg: BackgroundColor) {
    // Get references to the data we need
    let (mut crsr_x, mut crsr_y) = get_cursor_pos();
    let video_mem = VGA_MEMORY as *mut u8;

    for c in msg.chars() {
        let offset = crsr_y as u16 * VGA_WIDTH as u16 + crsr_x as u16;
        match c {
            '\n' => {
                crsr_y += 1;
                crsr_x = 0;
            }, '\r' => {
                crsr_x = 0;
            }, _ => {
                unsafe {
                    let pos = video_mem.add(((offset as u16) * 2) as usize);
                    *pos = c as u8;
                    let pos = pos.add(1);
                    *pos = fg as u8 | bg as u8;
                }
                crsr_x += 1;
            }
        }
    }

    // Update cursor
    let mut new_crsr_x = crsr_x;
    let new_crsr_y = if new_crsr_x >= VGA_WIDTH {
        new_crsr_x -= VGA_WIDTH;
        crsr_y + 1
    } else {
        crsr_y
    };
    set_cursor_pos(new_crsr_x, new_crsr_y);
}

pub fn print_u64(val: u64, fg: ForegroundColor, bg: BackgroundColor) {
    // Get references to the data we need
    let (mut crsr_x, crsr_y) = get_cursor_pos();
    let video_mem = VGA_MEMORY as *mut u8;

    for i in 0..8 {
        let b = val >> (8 * (7 - i));
        let high = (b as u8 >> 4) & 0x0F;
        let low = b as u8 & 0x0F;

        let high_c = if high > 9 {
            high - 10 + 'A' as u8
        } else {
            high + '0' as u8
        } as char;
        let low_c = if low > 9 {
            low - 10 + 'A' as u8
        } else {
            low + '0' as u8
        } as char;

        let offset = crsr_y as u16 * VGA_WIDTH as u16 + crsr_x as u16;
        unsafe {
            let pos = video_mem.add(((offset as u16) * 2) as usize);
            *pos = high_c as u8;
            let pos = pos.add(1);
            *pos = fg as u8 | bg as u8;
        }
        crsr_x += 1;

        let offset = crsr_y as u16 * VGA_WIDTH as u16 + crsr_x as u16;
        unsafe {
            let pos = video_mem.add(((offset as u16) * 2) as usize);
            *pos = low_c as u8;
            let pos = pos.add(1);
            *pos = fg as u8 | bg as u8;
        }
        crsr_x += 1;
    }

    // Update cursor
    let mut new_crsr_x = crsr_x;
    let new_crsr_y = if new_crsr_x >= VGA_WIDTH {
        new_crsr_x -= VGA_WIDTH;
        crsr_y + 1
    } else {
        crsr_y
    };
    set_cursor_pos(new_crsr_x, new_crsr_y);
}

// Could probably optimize to set 4 chars at a time. Needs memset for that tho
pub fn clear_screen(color: BackgroundColor) {
    // 64-bit OS, so we can set 4 colors at a time to speed stuff up
    let col_u8 = color as u8 | ForegroundColor::White as u8;

    let video_mem = VGA_MEMORY as *mut u8;
    for offset in 0..VGA_WIDTH as usize * VGA_HEIGHT as usize * 2 {
        if offset % 2 == 0 {
            continue;
        }

        unsafe {
            *video_mem.add(offset) = col_u8;
        }
    }
}
