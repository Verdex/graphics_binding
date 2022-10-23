
pub mod sdl;
pub mod ttf;



#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::*;
    use std::{thread, time};
    use std::mem::MaybeUninit;

    use crate::sdl::pixels::*;
    use crate::sdl::rect::*;
    use crate::sdl::event::*;
    use crate::sdl::opaque::*;
    use crate::sdl::functions::*;
    use crate::ttf::functions::*;

    #[test]
    fn example() {
        use crate::sdl::init::*;
        use crate::sdl::video::*;

        unsafe {
            SDL_Init(SDL_INIT_EVENTS | SDL_INIT_VIDEO);
            TTF_Init();

            let title = &CString::new("the title").expect("CString::new failed");

            let window = SDL_CreateWindow(title.as_ptr(), 100, 100, 1000, 1000, SDL_WINDOW_RESIZABLE);

            let renderer = SDL_CreateRenderer(window, -1, 0);

            let font_file_name = &CString::new("C:\\Windows\\Fonts\\arial.ttf").expect("CString::new failed");

            let font = TTF_OpenFont(font_file_name.as_ptr(), 12);

            if font.is_null() {
                let error = SDL_GetError();
                let w = CStr::from_ptr(error).to_str().unwrap();
                assert!( false, "{}", w );
            }

            let color = SDL_Color { r: 100, g: 100, b: 100, a: 150 };

            let message = &CString::new("the message").expect("CString::new failed");
            let surface_message = TTF_RenderText_Solid(font, message.as_ptr(), color);

            if surface_message.is_null() {
                let error = SDL_GetError();
                let w = CStr::from_ptr(error).to_str().unwrap();
                assert!( false, "{}", w );
            }

            let texture = SDL_CreateTextureFromSurface(renderer, surface_message);

            if texture.is_null() {
                let error = SDL_GetError();
                let w = CStr::from_ptr(error).to_str().unwrap();
                assert!( false, "{}", w );
            }

            let rect = SDL_Rect { x: 10, y : 10, w : 200, h : 100 };


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

                let x = SDL_RenderCopy(renderer, texture, std::ptr::null(), &rect as *const SDL_Rect); // TODO

                if x < 0 {
                    let error = SDL_GetError();
                    let w = CStr::from_ptr(error).to_str().unwrap();
                    assert!( false, "{}", w );
                }

                SDL_RenderPresent(renderer);


                let x = time::Duration::from_millis(100);
                thread::sleep(x);
            }

            TTF_Quit();

            SDL_DestroyWindow(window);

            SDL_Quit();
        }
    }
}
