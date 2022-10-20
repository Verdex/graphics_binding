
#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_WindowEvent {
    event_type : u32,
    timestamp : u32,
    window_id : u32,
    event : u8,
    data_1 : i32,
    data_2 : i32,
}

const SDL_WINDOWEVENT_NONE : u8 = 0;
const SDL_WINDOWEVENT_SHOWN : u8 = 1;
const SDL_WINDOWEVENT_HIDDEN : u8 = 1;
const SDL_WINDOWEVENT_EXPOSED : u8 = 1;
const SDL_WINDOWEVENT_MOVED : u8 = 1;
const SDL_WINDOWEVENT_RESIZED : u8 = 1;
const SDL_WINDOWEVENT_SIZE_CHANGED : u8 = 1;
const SDL_WINDOWEVENT_MINIMIZED : u8 = 1;
const SDL_WINDOWEVENT_MAXIMIZED : u8 = 1;
const SDL_WINDOWEVENT_RESTORED : u8 = 1;
const SDL_WINDOWEVENT_ENTER : u8 = 1;
const SDL_WINDOWEVENT_LEAVE : u8 = 1;
const SDL_WINDOWEVENT_FOCUS_GAINED: u8 = 1;
const SDL_WINDOWEVENT_FOCUS_LOST : u8 = 1;
const SDL_WINDOWEVENT_CLOSE : u8 = 1;
const SDL_WINDOWEVENT_TAKE_FOCUS : u8 = 1;
const SDL_WINDOWEVENT_HIT_TEST : u8 = 1;
const SDL_WINDOWEVENT_ICCPROF_CHANGED : u8 = 1;
const SDL_WINDOWEVENT_DISPLAY_CHANGED : u8 = 1;

