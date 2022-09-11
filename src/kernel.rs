/*
 * Author: Dylan Turner
 * Description: Entry point for the Cyub OS kernel
 */

use crate::{
    terminal::{
        set_cursor_pos, print_str, clear_screen,
        ForegroundColor, BackgroundColor
    }, idt::idt_init
};

#[no_mangle]
pub extern "C" fn kernel_start() {
    clear_screen(BackgroundColor::Black);

    print_all_colors();

    set_cursor_pos(3, 4);
    print_str("Hello from rust kernel!\n", ForegroundColor::White, BackgroundColor::Black);
    print_str("  Hello from rust kernel!\r", ForegroundColor::LightGray, BackgroundColor::Green);
    print_str("Goodbye\n", ForegroundColor::Yellow, BackgroundColor::LightBlue);

    idt_init();
}

fn print_all_colors() {
    set_cursor_pos(0, 0);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Black);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Blue);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Green);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Cyan);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Red);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Magenta);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Brown);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightGray);
    print_str(" ", ForegroundColor::Black, BackgroundColor::DarkGray);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightBlue);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightCyan);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightGreen);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightRed);
    print_str(" ", ForegroundColor::Black, BackgroundColor::LightMagenta);
    print_str(" ", ForegroundColor::Black, BackgroundColor::Yellow);
    print_str(" ", ForegroundColor::Black, BackgroundColor::White);

    set_cursor_pos(0, 1);
    print_str("A", ForegroundColor::Black, BackgroundColor::White);
    print_str("A", ForegroundColor::Blue, BackgroundColor::White);
    print_str("A", ForegroundColor::Green, BackgroundColor::White);
    print_str("A", ForegroundColor::Cyan, BackgroundColor::White);
    print_str("A", ForegroundColor::Red, BackgroundColor::White);
    print_str("A", ForegroundColor::Magenta, BackgroundColor::White);
    print_str("A", ForegroundColor::Brown, BackgroundColor::White);
    print_str("A", ForegroundColor::LightGray, BackgroundColor::White);
    print_str("A", ForegroundColor::DarkGray, BackgroundColor::White);
    print_str("A", ForegroundColor::LightBlue, BackgroundColor::White);
    print_str("A", ForegroundColor::LightCyan, BackgroundColor::White);
    print_str("A", ForegroundColor::LightGreen, BackgroundColor::White);
    print_str("A", ForegroundColor::LightRed, BackgroundColor::White);
    print_str("A", ForegroundColor::LightMagenta, BackgroundColor::White);
    print_str("A", ForegroundColor::Yellow, BackgroundColor::White);
    print_str("A", ForegroundColor::White, BackgroundColor::Black);
}
