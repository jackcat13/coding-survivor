use std::{collections::VecDeque, thread};

use raylib::ffi::{self, KeyboardKey};

pub const BACKSPACE: char = '\x08';
pub const CARRIAGE_RETURN: char = '\x13';
pub const ARROW_UP: char = '↑';
pub static mut KEYS_PRESSED: VecDeque<char> = VecDeque::new();

#[allow(static_mut_refs)]
pub fn start_keyboard_thread() {
    thread::spawn(move || {
        loop {
            process_key_pressed();
        }
    });
}

// Accept mutable static references here because concurrent access to push first
// and pop last is expected
#[allow(static_mut_refs)]
#[inline]
pub fn process_key_pressed() {
    let key = unsafe { ffi::GetKeyPressed() };
    if key == KeyboardKey::KEY_BACKSPACE as i32 {
        unsafe { KEYS_PRESSED.push_front(BACKSPACE) };
    } else if key == KeyboardKey::KEY_ENTER as i32 {
        unsafe { KEYS_PRESSED.push_front(CARRIAGE_RETURN) };
    } else if key == KeyboardKey::KEY_UP as i32 {
        unsafe { KEYS_PRESSED.push_front(ARROW_UP) };
    } else {
        //Process actual character in another thread to avoid performance loss
        thread::spawn(move || {
            let key = unsafe { ffi::GetCharPressed() };
            if key > 0 {
                if let Some(character) = char::from_u32(key as u32) {
                    unsafe { KEYS_PRESSED.push_front(character) };
                }
            }
        });
    }
}
 
