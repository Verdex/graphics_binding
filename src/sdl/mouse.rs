
use std::ffi::c_float;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseMotionEvent {
    pub event_type : u32,
    pub timestamp : u32,
    pub window_id : u32,
    pub which : u32,
    pub state : u32,
    pub x : i32,
    pub y : i32,
    pub xrel : i32,
    pub yrel : i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseButtonEvent {
    pub event_type : u32,
    pub timestamp : u32,
    pub window_id : u32,
    pub which : u32,
    pub button : u8,
    pub state : u8,
    pub clicks : u8, // 1 for single click, 2 for double click, etc
    pub x : i32,
    pub y : i32,
}

pub const SDL_BUTTON_LEFT : u8 = 1;
pub const SDL_BUTTON_MIDDLE : u8 = 2;
pub const SDL_BUTTON_RIGHT : u8 = 3;
pub const SDL_BUTTON_X1 : u8 = 4;
pub const SDL_BUTTON_X2 : u8 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseWheelEvent {
    pub event_type : u32,
    pub timestamp : u32,
    pub window_id : u32,
    pub which : u32,
    pub x : i32,
    pub y : i32,
    pub direction : u32,
    pub precise_x : c_float, 
    pub precise_y : c_float,
}

pub const SDL_MOUSEWHEEL_NORMAL : u32 = 0;
pub const SDL_MOUSEWHEEL_FLIPPED : u32 = 1;