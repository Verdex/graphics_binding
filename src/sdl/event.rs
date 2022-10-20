
use super::keyboard::SDL_KeyboardEvent;
use super::window::SDL_WindowEvent;
use super::mouse::{SDL_MouseMotionEvent, SDL_MouseButtonEvent, SDL_MouseWheelEvent};

const SDL_WINDOWEVENT : u32 = 0x200;
const SDL_KEYDOWN : u32 = 0x300;
const SDL_KEYUP : u32 = 0x301;
const SDL_QUIT : u32 = 0x100;
const SDL_MOUSEMOTION : u32 = 0x400;
const SDL_MOUSEBUTTONDOWN : u32 = 0x401;
const SDL_MOUSEBUTTONUP : u32 = 0x402;
const SDL_MOUSEWHEEL : u32 = 0x403;


#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_QuitEvent {
    event_type : u32,
    timestamp : u32,
}

#[repr(C)]
union SDL_Event {
    event_type : u32,
    window : SDL_WindowEvent,
    keyboard : SDL_KeyboardEvent,
    quit : SDL_QuitEvent,
    mouse_motion : SDL_MouseMotionEvent,
    mouse_button : SDL_MouseButtonEvent,
    mouse_wheel : SDL_MouseWheelEvent,
}