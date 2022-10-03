
use std::ffi::*;

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

const WINDOW_EVENT_TYPE : u32 = 0x200;

#[repr(C)]
union SDL_Event {
    event_type : u32,
    window : SDL_WindowEvent,
}

/*
*/

/*#[repr(C)]
union SDL_Event {
    Uint32

type

event type, shared with all events

SDL_CommonEvent 
SDL_WindowEvent
SDL_KeyboardEvent
SDL_TextEditingEvent
SDL_TextInputEvent
SDL_MouseMotionEvent
SDL_MouseButtonEvent
SDL_MouseWheelEvent
SDL_JoyAxisEvent
SDL_JoyBallEvent
SDL_JoyHatEvent
SDL_JoyButtonEvent
SDL_JoyDeviceEvent
SDL_ControllerAxisEvent
SDL_ControllerButtonEvent
SDL_ControllerDeviceEvent
SDL_AudioDeviceEvent
SDL_QuitEvent
SDL_UserEvent
SDL_SysWMEvent
SDL_TouchFingerEvent
SDL_MultiGestureEvent
SDL_DollarGestureEvent
SDL_DropEvent
  }*/


#[link(name="SDL2")]
extern {
    fn SDL_Init(flags : u32) -> c_int; // TODO:  return zero for ok
    fn SDL_Quit();
    fn SDL_CreateWindow( title : *const c_char, x : c_int, y : c_int, width : c_int, height : c_int, flags : u32) -> *const c_void;
    fn SDL_DestroyWindow(window : *const c_void);
    // TODO:  Not tested
    fn SDL_GetError() -> *const c_char;
    fn SDL_PollEvent(event : *mut SDL_Event) -> c_int;
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
                        WINDOW_EVENT_TYPE => {
                            let z = e.window;
                            println!( "window event = {}", z.event );
                        },
                        _ => {},
                    }
                }

                let x = time::Duration::from_millis(100);
                thread::sleep(x);
            }

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            SDL_DestroyWindow(window);

            SDL_Quit();
        }
    }
}
