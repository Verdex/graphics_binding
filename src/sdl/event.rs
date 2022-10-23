
use super::keyboard::SDL_KeyboardEvent;
use super::window::SDL_WindowEvent;
use super::mouse::{SDL_MouseMotionEvent, SDL_MouseButtonEvent, SDL_MouseWheelEvent};

pub const SDL_WINDOWEVENT : u32 = 0x200;
pub const SDL_KEYDOWN : u32 = 0x300;
pub const SDL_KEYUP : u32 = 0x301;
pub const SDL_QUIT : u32 = 0x100;
pub const SDL_MOUSEMOTION : u32 = 0x400;
pub const SDL_MOUSEBUTTONDOWN : u32 = 0x401;
pub const SDL_MOUSEBUTTONUP : u32 = 0x402;
pub const SDL_MOUSEWHEEL : u32 = 0x403;

pub const SDL_RELEASED : u8 = 0;
pub const SDL_PRESSED : u8 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_QuitEvent {
    pub event_type : u32,
    pub timestamp : u32,
}

#[repr(C)]
pub union SDL_Event {
    pub event_type : u32,
    pub window : SDL_WindowEvent,
    pub keyboard : SDL_KeyboardEvent,
    pub quit : SDL_QuitEvent,
    pub mouse_motion : SDL_MouseMotionEvent,
    pub mouse_button : SDL_MouseButtonEvent,
    pub mouse_wheel : SDL_MouseWheelEvent,
}