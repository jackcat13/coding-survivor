use std::{collections::VecDeque, thread};

use raylib::ffi::{self};

pub const BACKSPACE: char = '\x08';
pub const CARRIAGE_RETURN: char = '\x13';
pub static mut KEYS_PRESSED: VecDeque<char> = VecDeque::new();

// Accept mutable static references here because concurrent access to push first
// and pop last is expected
#[allow(static_mut_refs)]
pub fn start_keyboard_thread() {
    thread::spawn(move || {
        loop {
            unsafe { 
                if let Some(key) = get_key_pressed() {
                    KEYS_PRESSED.push_front(key);
                }
            };
        }
    });
}

/// Gets latest key pressed.
#[inline]
pub fn get_key_pressed() -> Option<char> {
    let key = unsafe { ffi::GetKeyPressed() };
    if key == 259 {
        return Some(BACKSPACE);
    } else if key == 257 {
        return Some(CARRIAGE_RETURN);
    }
    let key = unsafe { ffi::GetCharPressed() };
    if key > 0 {
        return char::from_u32(key as u32);
    }
    None
}
 
