
mod sdl;

use std::ffi::*;
use crate::sdl::event::*;



#[link(name="SDL2")]
extern {
    fn SDL_Init(flags : u32) -> c_int; // TODO:  return zero for ok
    fn SDL_Quit();
    fn SDL_CreateWindow( title : *const c_char, x : c_int, y : c_int, width : c_int, height : c_int, flags : u32) -> *const c_void;
    fn SDL_DestroyWindow(window : *const c_void);
    fn SDL_PollEvent(event : *mut SDL_Event) -> c_int;
    // TODO:  Not tested
    fn SDL_GetError() -> *const c_char;
}

const SDL_INIT_EVENTS : u32 = 0x4000;
const SDL_INIT_VIDEO : u32 = 0x20;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{thread, time};
    use std::mem::MaybeUninit;

    #[test]
    fn blarg() {
        unsafe {
            SDL_Init(SDL_INIT_EVENTS | SDL_INIT_VIDEO);

            let title = &CString::new("the title").expect("CString::new failed");

            let window = SDL_CreateWindow(title.as_ptr(), 100, 100, 1000, 1000, 0);


            loop {
                let mut event : MaybeUninit<SDL_Event> = MaybeUninit::uninit();
                let mut event_ptr = event.as_mut_ptr();
                let z = SDL_PollEvent(event_ptr);
                println!("result = {}", z);
                if !event_ptr.is_null() {
                    let e = event.assume_init();
                    println!("event type = {}", e.event_type);
                    match e.event_type {
                        SDL_WINDOWEVENT => {
                            let z = e.window;
                            println!( "window event = {}", z.event );
                        },
                        SDL_QUIT => {
                            break;
                        }
                        _ => {},
                    }
                }

                let x = time::Duration::from_millis(100);
                thread::sleep(x);
            }

            SDL_DestroyWindow(window);

            SDL_Quit();
        }
    }
}
