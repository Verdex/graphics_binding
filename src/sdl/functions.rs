
use std::ffi::*;
use crate::sdl::rect::*;
use crate::sdl::event::*;
use crate::sdl::opaque::*;



#[link(name="SDL2")]
extern {
    pub fn SDL_Init(flags : u32) -> c_int; 
    pub fn SDL_Quit();
    pub fn SDL_CreateWindow( title : *const c_char, x : c_int, y : c_int, width : c_int, height : c_int, flags : u32) -> *const SDL_Window;
    pub fn SDL_DestroyWindow(window : *const SDL_Window);
    pub fn SDL_PollEvent(event : *mut SDL_Event) -> c_int;
    pub fn SDL_GetError() -> *const c_char;
    pub fn SDL_RenderPresent(renderer : *const SDL_Renderer);
    pub fn SDL_CreateRenderer(window : *const SDL_Window, index : c_int, flags : u32) -> *const SDL_Renderer;
    pub fn SDL_CreateTextureFromSurface(renderer : *const SDL_Renderer, surface : *const SDL_Surface) -> *const SDL_Texture;
    pub fn SDL_RenderCopy(renderer : *const SDL_Renderer, texture : *const SDL_Texture, src : *const SDL_Rect, dest : *const SDL_Rect) -> c_int; // zero for success and negative for error
    // TODO:  Not tested
    pub fn SDL_RenderFlush(renderer : *const SDL_Renderer) -> c_int;
    pub fn SDL_RenderClear(renderer : *const SDL_Renderer) -> c_int;
    pub fn SDL_RenderCopyEx(renderer : *const SDL_Renderer, texture : *const SDL_Texture, src : *const SDL_Rect, dest : *const SDL_Rect, angle : c_double, center : *const SDL_Point, flip : c_int) -> c_int;
    pub fn SDL_RenderCopyExF(renderer : *const SDL_Renderer, texture : *const SDL_Texture, src : *const SDL_Rect, dest : *const SDL_FRect, angle : c_double, center : *const SDL_FPoint, flip : c_int) -> c_int;
    pub fn SDL_RenderCopyF(renderer : *const SDL_Renderer, texture : *const SDL_Texture, src : *const SDL_Rect, dest : *const SDL_FRect) -> c_int;
    pub fn SDL_RenderDrawLine(renderer : *const SDL_Renderer, x1 : c_int, y1 : c_int, x2 : c_int, y2 : c_int) -> c_int;
    pub fn SDL_RenderDrawLineF(renderer : *const SDL_Renderer, x1 : c_float, y1 : c_float, x2 : c_float, y2 : c_float) -> c_int;
    pub fn SDL_RenderDrawLines(renderer : *const SDL_Renderer, points : *const SDL_Point, count : c_int) -> c_int;
    pub fn SDL_RenderDrawLinesF(renderer : *const SDL_Renderer, points : *const SDL_FPoint, count : c_int) -> c_int;
    pub fn SDL_RenderDrawPoint(renderer : *const SDL_Renderer, x : c_int, y : c_int) -> c_int;
    pub fn SDL_RenderDrawPointF(renderer : *const SDL_Renderer, x : c_float, y : c_float) -> c_int;
    pub fn SDL_RenderDrawPoints(renderer : *const SDL_Renderer, points : *const SDL_Point, count : c_int) -> c_int;
    pub fn SDL_RenderDrawPointsF(renderer : *const SDL_Renderer, points : *const SDL_FPoint, count : c_int) -> c_int;
    pub fn SDL_RenderDrawRect(renderer : *const SDL_Renderer, rect : *const SDL_Rect) -> c_int;
    pub fn SDL_RenderDrawRectF(renderer : *const SDL_Renderer, rect : *const SDL_FRect) -> c_int;
    pub fn SDL_RenderDrawRects(renderer : *const SDL_Renderer, rects : *const SDL_Rect, count : c_int) -> c_int;
    pub fn SDL_RenderDrawRectsF(renderer : *const SDL_Renderer, rects : *const SDL_FRect, count : c_int) -> c_int;
    pub fn SDL_RenderFillRect(renderer : *const SDL_Renderer, rect : *const SDL_Rect) -> c_int;
    pub fn SDL_RenderFillRectF(renderer : *const SDL_Renderer, rect : *const SDL_FRect) -> c_int;
    pub fn SDL_RenderFillRects(renderer : *const SDL_Renderer, rects : *const SDL_Rect, count : c_int) -> c_int;
    pub fn SDL_RenderFillRectsF(renderer : *const SDL_Renderer, rects : *const SDL_FRect, count : c_int) -> c_int;
    pub fn SDL_SetRenderDrawColor(renderer : *const SDL_Renderer, r : u8, g : u8, b : u8, a : u8) -> c_int;
    pub fn SDL_RenderSetScale(renderer : *const SDL_Renderer, scale_x : c_float, scale_y : c_float) -> c_int;
    pub fn SDL_SetRenderDrawBlendMode(renderer : *const SDL_Renderer, blend_mode : c_int) -> c_int;
    pub fn SDL_FreeSurface(surface : *const SDL_Surface);
    pub fn SDL_DestroyTexture(texture : *const SDL_Texture);
    pub fn SDL_DestroyRenderer(renderer : *const SDL_Renderer);
}