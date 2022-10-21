
use std::ffi::c_int;

pub const SDL_RENDERER_SOFTWARE : u32 = 0x01;
pub const SDL_RENDERER_ACCELERATED : u32 = 0x02;
pub const SDL_RENDERER_PRESENTVSYNC : u32 = 0x04;
pub const SDL_RENDERER_TARGETTEXTURE : u32 = 0x08;


pub const SDL_FLIP_NONE : c_int = 0x00;
pub const SDL_FLIP_HORIZONTAL : c_int = 0x01;
pub const SDL_FLIP_VERTICAL : c_int = 0x02;