
use std::ffi::*;
use crate::sdl::opaque::*;
use crate::sdl::pixels::*;
use super::opaque::*;

#[link(name="SDL2_ttf")]
extern {
    pub fn TTF_OpenFont(font_file_name : *const c_char, point_size : c_int) -> *const TTF_Font;
    pub fn TTF_RenderText_Solid(font : *const TTF_Font, text : *const c_char, foreground : SDL_Color) -> *const SDL_Surface;
    pub fn TTF_Quit();
    pub fn TTF_Init() -> c_int;
}
