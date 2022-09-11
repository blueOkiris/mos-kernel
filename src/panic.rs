/*
 * Author: Dylan Turner
 * Description: Replacement panic function for Rust
 */

use core::panic::PanicInfo;
use crate::terminal::{
    set_cursor_pos, print_str, ForegroundColor, BackgroundColor
};

#[no_mangle]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_cursor_pos(0, 0);

    match info.message() {
        None => print_str(
            "An unknown panic error occured :(", ForegroundColor::White, BackgroundColor::Black
        ), Some(msg) => print_str(
            msg.as_str().unwrap(), ForegroundColor::White, BackgroundColor::Black
        )
    }

    loop {
    }
}
