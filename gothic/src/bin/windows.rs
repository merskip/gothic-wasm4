use std::os::raw::c_void;

use gothic::GothicApplication;
use wasm4::application::Application;
use wasm4::gamepad::Gamepad;
use wasm4::inputs::Inputs;

#[no_mangle]
pub fn rect(x: i32, y: i32, width: u32, height: u32) {}

#[no_mangle]
pub fn oval(x: i32, y: i32, width: u32, height: u32) {}

#[no_mangle]
pub fn line(x1: i32, y1: i32, x2: i32, y2: i32) {}

#[no_mangle]
pub fn hline(x: i32, y: i32, len: u32) {}

#[no_mangle]
pub fn vline(x: i32, y: i32, len: u32) {}

#[no_mangle]
pub fn text(text: *const u8, x: i32, y: i32) {}

#[no_mangle]
pub fn textUtf8(text: *const u8, byte_length: usize, x: i32, y: i32) {}

#[no_mangle]
pub fn textUtf16(text: *const u16, byte_length: usize, x: i32, y: i32) {}

#[no_mangle]
pub fn blit(sprite: *const u8, x: i32, y: i32, width: u32, height: u32, flags: u32) {}

#[no_mangle]
pub fn blitSub(
    sprite: *const u8,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    src_x: u32,
    src_y: u32,
    stride: u32,
    flags: u32,
) {}

#[no_mangle]
pub fn tone(frequency: u32, duration: u32, volume: u32, flags: u32) {
    println!("frequency: {}, duration: {}, volume: {}, flags: {}", frequency, duration, volume, flags);
}

#[no_mangle]
pub fn diskr(dest: *mut u8, size: usize) -> u32 { 0 }

#[no_mangle]
pub fn diskw(src: *const u8, size: usize) -> u32 { 0 }

#[no_mangle]
pub fn trace(trace: *const u8) {}

#[no_mangle]
pub fn traceUtf8(trace: *const u8, byte_length: usize) {}

#[no_mangle]
pub fn traceUtf16(trace: *const u16, byte_length: usize) {}

#[no_mangle]
pub fn tracef(fmt: *const u8, stack_ptr: *const c_void) {}

fn main() {
    let mut application = GothicApplication::start();

    let inputs = unsafe { Inputs::new() };
    application.update(&inputs);

    println!("Hello world!");
}
