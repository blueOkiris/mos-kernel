/*
 * Author: Dylan Turner
 * Description: Entry point for the Cyub OS kernel
 */

use crate::{
    terminal::{
        set_cursor_pos, print_str, clear_screen, Color
    }, idt::idt_init
};

#[no_mangle]
pub extern "C" fn kernel_start() {
    idt_init();

    clear_screen(Color::Black);
    print_color_welcome();
}

fn print_color_welcome() {
    set_cursor_pos(0, 0);
    print_str(" ", Color::Black, Color::Black);
    print_str(" ", Color::Black, Color::Blue);
    print_str(" ", Color::Black, Color::Green);
    print_str(" ", Color::Black, Color::Cyan);
    print_str(" ", Color::Black, Color::Red);
    print_str(" ", Color::Black, Color::Magenta);
    print_str(" ", Color::Black, Color::Brown);
    print_str(" ", Color::Black, Color::LightGray);
    print_str(" ", Color::Black, Color::DarkGray);
    print_str(" ", Color::Black, Color::LightBlue);
    print_str(" ", Color::Black, Color::LightCyan);
    print_str(" ", Color::Black, Color::LightGreen);
    print_str(" ", Color::Black, Color::LightRed);
    print_str(" ", Color::Black, Color::LightMagenta);
    print_str(" ", Color::Black, Color::Yellow);
    print_str(" ", Color::Black, Color::White);

    set_cursor_pos(0, 1);
    print_str("W", Color::Black, Color::White);
    print_str("e", Color::Blue, Color::White);
    print_str("l", Color::Green, Color::White);
    print_str("c", Color::Cyan, Color::White);
    print_str("o", Color::Red, Color::White);
    print_str("m", Color::Magenta, Color::White);
    print_str("e", Color::Brown, Color::White);
    print_str(" ", Color::LightGray, Color::White);
    print_str("t", Color::DarkGray, Color::White);
    print_str("o", Color::LightBlue, Color::White);
    print_str(" ", Color::LightCyan, Color::White);
    print_str("M", Color::LightGreen, Color::White);
    print_str("O", Color::LightRed, Color::White);
    print_str("S", Color::LightMagenta, Color::White);
    print_str("!", Color::Yellow, Color::White);
    print_str("!", Color::White, Color::Black);

    set_cursor_pos(0, 2);
}
