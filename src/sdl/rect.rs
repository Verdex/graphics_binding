
use std::ffi::{c_int, c_float};

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
pub struct SDL_FRect {
    pub x : c_float, 
    pub y : c_float,
    pub w : c_float,
    pub h : c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Point {
    pub x : c_int,
    pub y : c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_FPoint {
    pub x : c_float,
    pub y : c_float,
}