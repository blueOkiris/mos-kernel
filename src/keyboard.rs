/*
 * Author: Dylan Turner
 * Description: Handle key presses
 */

use crate::terminal::{
    print_char, print_str, Color, backspace
};

// Keyboard scan codes; TODO: Move to user space
const SCANCODE_TABLE: [char; 89] = [
    '\0', // Nothing
    '\0', // Escape - 0x01
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
    '\0', // Backspace - 0x0E
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    '\0', // Left control - 0x1D
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',  ';', '\'', '`',
    '\0', // Left shift - 0x2A
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',
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
pub fn keyboard_handler(code: u8) {
    let c = if (code as usize) < SCANCODE_TABLE.len() {
        SCANCODE_TABLE[code as usize]
    } /*else if (code & 0x80) > 0 && ((code & (!0x80)) as usize) < SCANCODE_TABLE.len() {
        c = SCANCODE_TABLE[(code & (!0x80)) as usize];
    } */else {
        '\0'
    };
    let is_rel = false;//(code | 0x80) > 0;

    if c != '\0' && !is_rel {
        print_char(c, Color::White, Color::Black);    
    } else if !is_rel {
        match code {
            0x01 => print_str("<ESC>", Color::Blue, Color::Black),
            0x0E => backspace(),
            0x1D => print_str("<LCTRL>", Color::Blue, Color::Black),
            0x2A => print_str("<LSHIFT>", Color::Blue, Color::Black),
            0x36 => print_str("<RSHIFT>", Color::Blue, Color::Black),
            0x38 => print_str("<LALT>", Color::Blue, Color::Black),
            0x3A => print_str("<CPSLK>", Color::Blue, Color::Black),
            0x3B => print_str("<F1>", Color::Blue, Color::Black),
            0x3C => print_str("<F2>", Color::Blue, Color::Black),
            0x3D => print_str("<F3>", Color::Blue, Color::Black),
            0x3E => print_str("<F4>", Color::Blue, Color::Black),
            0x3F => print_str("<F5>", Color::Blue, Color::Black),
            0x40 => print_str("<F6>", Color::Blue, Color::Black),
            0x41 => print_str("<F7>", Color::Blue, Color::Black),
            0x42 => print_str("<F8>", Color::Blue, Color::Black),
            0x43 => print_str("<F9>", Color::Blue, Color::Black),
            0x44 => print_str("<F10>", Color::Blue, Color::Black),
            0x45 => print_str("<NMLK>", Color::Blue, Color::Black),
            0x46 => print_str("<SCRLK>", Color::Blue, Color::Black),
            0x57 => print_str("<F11>", Color::Blue, Color::Black),
            0x48 => print_str("<F12>", Color::Blue, Color::Black),
            _ => {} // Not handled
        }
    }
}

