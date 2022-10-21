
use std::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Rect {
    pub x : c_int,
    pub y : c_int,
    pub w : c_int,
    pub h : c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Point {
    pub x : c_int,
    pub y : c_int,
}