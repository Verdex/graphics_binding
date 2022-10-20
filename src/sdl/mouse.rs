
use std::ffi::c_float;

#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_MouseMotionEvent {
    event_type : u32,
    timestamp : u32,
    window_id : u32,
    which : u32,
    state : u32,
    x : i32,
    y : i32,
    xrel : i32,
    yrel : i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_MouseButtonEvent {
    event_type : u32,
    timestamp : u32,
    window_id : u32,
    which : u32,
    button : u8,
    state : u8,
    clicks : u8, // 1 for single click, 2 for double click, etc
    x : i32,
    y : i32,
}

const SDL_BUTTON_LEFT : u8 = 1;
const SDL_BUTTON_MIDDLE : u8 = 2;
const SDL_BUTTON_RIGHT : u8 = 3;
const SDL_BUTTON_X1 : u8 = 4;
const SDL_BUTTON_X2 : u8 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_MouseWheelEvent {
    event_type : u32,
    timestamp : u32,
    window_id : u32,
    which : u32,
    x : i32,
    y : i32,
    direction : u32,
    precise_x : c_float, 
    precise_y : c_float,
}

const SDL_MOUSEWHEEL_NORMAL : u32 = 0;
const SDL_MOUSEWHEEL_FLIPPED : u32 = 1;


const SDL_RELEASED : u8 = 0;
const SDL_PRESSED : u8 = 1;