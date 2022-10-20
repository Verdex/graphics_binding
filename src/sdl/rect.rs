
use std::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Rect {
    x : c_int,
    y : c_int,
    w : c_int,
    h : c_int,
}