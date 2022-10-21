
use std::ffi::c_int;

pub const SDL_BLENDMODE_NONE : c_int = 0x00;
pub const SDL_BLENDMODE_BLEND : c_int = 0x01;
pub const SDL_BLENDMODE_ADD : c_int = 0x02;
pub const SDL_BLENDMODE_MOD : c_int = 0x04;
pub const SDL_BLENDMODE_MUL : c_int = 0x08;