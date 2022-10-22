
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_WindowEvent {
    pub event_type : u32,
    pub timestamp : u32,
    pub window_id : u32,
    pub event : u8,
    pub data_1 : i32,
    pub data_2 : i32,
}

pub const SDL_WINDOWEVENT_NONE : u8 = 0;
pub const SDL_WINDOWEVENT_SHOWN : u8 = 1;
pub const SDL_WINDOWEVENT_HIDDEN : u8 = 2;
pub const SDL_WINDOWEVENT_EXPOSED : u8 = 3;
pub const SDL_WINDOWEVENT_MOVED : u8 = 4;
pub const SDL_WINDOWEVENT_RESIZED : u8 = 5;
pub const SDL_WINDOWEVENT_SIZE_CHANGED : u8 = 6;
pub const SDL_WINDOWEVENT_MINIMIZED : u8 = 7;
pub const SDL_WINDOWEVENT_MAXIMIZED : u8 = 8;
pub const SDL_WINDOWEVENT_RESTORED : u8 = 9;
pub const SDL_WINDOWEVENT_ENTER : u8 = 10;
pub const SDL_WINDOWEVENT_LEAVE : u8 = 11;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: u8 = 12;
pub const SDL_WINDOWEVENT_FOCUS_LOST : u8 = 13;
pub const SDL_WINDOWEVENT_CLOSE : u8 = 14;
pub const SDL_WINDOWEVENT_TAKE_FOCUS : u8 = 15;
pub const SDL_WINDOWEVENT_HIT_TEST : u8 = 16;
pub const SDL_WINDOWEVENT_ICCPROF_CHANGED : u8 = 17;
pub const SDL_WINDOWEVENT_DISPLAY_CHANGED : u8 = 18;

