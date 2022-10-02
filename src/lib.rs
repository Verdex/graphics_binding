
use std::ffi::*;

#[link(name="SDL2")]
extern {
    fn SDL_Init(flags : c_uint) -> c_int;
    fn SDL_Quit();
    fn SDL_CreateWindow( title : *const c_char, x : c_int, y : c_int, width : c_int, height : c_int, flags : c_uint) -> *const c_void;
    fn SDL_DestroyWindow(window : *const c_void);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blarg() {
        use std::{thread, time};
        unsafe {
            SDL_Init(0x20);

            let title = &CString::new("the title").expect("CString::new failed");

            let window = SDL_CreateWindow(title.as_ptr(), 100, 100, 1000, 1000, 0);

            let ten_millis = time::Duration::from_millis(5000);
            let now = time::Instant::now();

            thread::sleep(ten_millis);

            SDL_DestroyWindow(window);

            SDL_Quit();
        }
    }
}
