
use std::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Keysym {
    pub scancode : c_int,
    pub keycode : i32,
    pub key_mod : u16,
    unused : u32,
}

pub const KMOD_NONE : u16 = 0x0000;
pub const KMOD_LSHIFT : u16 = 0x0001;
pub const KMOD_RSHIFT : u16 = 0x0002;
pub const KMOD_LCTRL : u16 = 0x0040;
pub const KMOD_RCTRL : u16 = 0x0080;
pub const KMOD_LALT : u16 = 0x0100;
pub const KMOD_RALT : u16 = 0x0200;
pub const KMOD_LGUI : u16 = 0x0400;
pub const KMOD_RGUI : u16 = 0x0800;
pub const KMOD_NUM : u16 = 0x1000;
pub const KMOD_CAPS : u16 = 0x2000;
pub const KMOD_MODE : u16 = 0x4000;
pub const KMOD_SCROLL : u16 = 0x8000;
pub const KMOD_CTRL : u16 = KMOD_LCTRL | KMOD_RCTRL; 
pub const KMOD_SHIFT : u16 = KMOD_LSHIFT | KMOD_RSHIFT;
pub const KMOD_ALT : u16 = KMOD_LALT | KMOD_RALT;
pub const KMOD_GUI : u16 = KMOD_LGUI | KMOD_RGUI;

pub const SDL_SCANCODE_UNKNOWN : c_int = 0;
pub const SDL_SCANCODE_A : c_int = 4;
pub const SDL_SCANCODE_B : c_int = 5;
pub const SDL_SCANCODE_C : c_int = 6;
pub const SDL_SCANCODE_D : c_int = 7;
pub const SDL_SCANCODE_E : c_int = 8;
pub const SDL_SCANCODE_F : c_int = 9;
pub const SDL_SCANCODE_G : c_int = 10;
pub const SDL_SCANCODE_H : c_int = 11;
pub const SDL_SCANCODE_I : c_int = 12;
pub const SDL_SCANCODE_J : c_int = 13;
pub const SDL_SCANCODE_K : c_int = 14;
pub const SDL_SCANCODE_L : c_int = 15;
pub const SDL_SCANCODE_M : c_int = 16;
pub const SDL_SCANCODE_N : c_int = 17;
pub const SDL_SCANCODE_O : c_int = 18;
pub const SDL_SCANCODE_P : c_int = 19;
pub const SDL_SCANCODE_Q : c_int = 20;
pub const SDL_SCANCODE_R : c_int = 21;
pub const SDL_SCANCODE_S : c_int = 22;
pub const SDL_SCANCODE_T : c_int = 23;
pub const SDL_SCANCODE_U : c_int = 24;
pub const SDL_SCANCODE_V : c_int = 25;
pub const SDL_SCANCODE_W : c_int = 26;
pub const SDL_SCANCODE_X : c_int = 27;
pub const SDL_SCANCODE_Y : c_int = 28;
pub const SDL_SCANCODE_Z : c_int = 29;

pub const SDL_SCANCODE_1 : c_int = 30;
pub const SDL_SCANCODE_2 : c_int = 31;
pub const SDL_SCANCODE_3 : c_int = 32;
pub const SDL_SCANCODE_4 : c_int = 33;
pub const SDL_SCANCODE_5 : c_int = 34;
pub const SDL_SCANCODE_6 : c_int = 35;
pub const SDL_SCANCODE_7 : c_int = 36;
pub const SDL_SCANCODE_8 : c_int = 37;
pub const SDL_SCANCODE_9 : c_int = 38;
pub const SDL_SCANCODE_0 : c_int = 39;

pub const SDL_SCANCODE_RETURN : c_int = 40;
pub const SDL_SCANCODE_ESCAPE : c_int = 41;
pub const SDL_SCANCODE_BACKSPACE : c_int = 42;
pub const SDL_SCANCODE_TAB : c_int = 43;
pub const SDL_SCANCODE_SPACE : c_int = 44;

pub const SDL_SCANCODE_MINUS : c_int = 45;
pub const SDL_SCANCODE_EQUALS : c_int = 46;

pub const SDL_SCANCODE_LEFTBRACKET : c_int = 47;
pub const SDL_SCANCODE_RIGHTBRACKET : c_int = 48;
pub const SDL_SCANCODE_BACKSLASH : c_int = 49;

pub const SDL_SCANCODE_NONUSHASH : c_int = 50;

pub const SDL_SCANCODE_SEMICOLON : c_int = 51;
pub const SDL_SCANCODE_APOSTROPHE : c_int = 52;
pub const SDL_SCANCODE_GRAVE : c_int = 53;
pub const SDL_SCANCODE_COMMA : c_int = 54;
pub const SDL_SCANCODE_PERIOD : c_int = 55;
pub const SDL_SCANCODE_SLASH : c_int = 56;
pub const SDL_SCANCODE_CAPSLOCK : c_int = 57;

pub const SDL_SCANCODE_F1 : c_int = 58;
pub const SDL_SCANCODE_F2 : c_int = 59;
pub const SDL_SCANCODE_F3 : c_int = 60;
pub const SDL_SCANCODE_F4 : c_int = 61;
pub const SDL_SCANCODE_F5 : c_int = 62;
pub const SDL_SCANCODE_F6 : c_int = 63;
pub const SDL_SCANCODE_F7 : c_int = 64;
pub const SDL_SCANCODE_F8 : c_int = 65;
pub const SDL_SCANCODE_F9 : c_int = 66;
pub const SDL_SCANCODE_F10 : c_int = 67;
pub const SDL_SCANCODE_F11 : c_int = 68;
pub const SDL_SCANCODE_F12 : c_int = 69;

pub const SDL_SCANCODE_PRINTSCREEN : c_int = 70;
pub const SDL_SCANCODE_SCROLLLOCK : c_int = 71;
pub const SDL_SCANCODE_PAUSE : c_int = 72;
pub const SDL_SCANCODE_INSERT : c_int = 73;

pub const SDL_SCANCODE_HOME : c_int = 74;
pub const SDL_SCANCODE_PAGEUP : c_int = 75;
pub const SDL_SCANCODE_DELETE : c_int = 76;
pub const SDL_SCANCODE_END : c_int = 77;
pub const SDL_SCANCODE_PAGEDOWN : c_int = 78;
pub const SDL_SCANCODE_RIGHT : c_int = 79;
pub const SDL_SCANCODE_LEFT : c_int = 80;
pub const SDL_SCANCODE_DOWN : c_int = 81;
pub const SDL_SCANCODE_UP : c_int = 82;


#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_KeyboardEvent {
    pub event_type : u32,
    pub timestamp : u32,
    pub window_id : u32,
    pub state : u8,
    pub repeat : u8,
    pub key : SDL_Keysym,
}


