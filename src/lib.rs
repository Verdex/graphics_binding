
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

#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_Keysym {
    scancode : c_int,
    keycode : i32,
    key_mod : u16,
    unused : u32,
}

const KMOD_NONE : u16 = 0x0000;
const KMOD_LSHIFT : u16 = 0x0001;
const KMOD_RSHIFT : u16 = 0x0002;
const KMOD_LCTRL : u16 = 0x0040;
const KMOD_RCTRL : u16 = 0x0080;
const KMOD_LALT : u16 = 0x0100;
const KMOD_RALT : u16 = 0x0200;
const KMOD_LGUI : u16 = 0x0400;
const KMOD_RGUI : u16 = 0x0800;
const KMOD_NUM : u16 = 0x1000;
const KMOD_CAPS : u16 = 0x2000;
const KMOD_MODE : u16 = 0x4000;
const KMOD_SCROLL : u16 = 0x8000;
const KMOD_CTRL : u16 = KMOD_LCTRL | KMOD_RCTRL; 
const KMOD_SHIFT : u16 = KMOD_LSHIFT | KMOD_RSHIFT;
const KMOD_ALT : u16 = KMOD_LALT | KMOD_RALT;
const KMOD_GUI : u16 = KMOD_LGUI | KMOD_RGUI;

const SDL_SCANCODE_UNKNOWN : c_int = 0;
const SDL_SCANCODE_A : c_int = 4;
const SDL_SCANCODE_B : c_int = 5;
const SDL_SCANCODE_C : c_int = 6;
const SDL_SCANCODE_D : c_int = 7;
const SDL_SCANCODE_E : c_int = 8;
const SDL_SCANCODE_F : c_int = 9;
const SDL_SCANCODE_G : c_int = 10;
const SDL_SCANCODE_H : c_int = 11;
const SDL_SCANCODE_I : c_int = 12;
const SDL_SCANCODE_J : c_int = 13;
const SDL_SCANCODE_K : c_int = 14;
const SDL_SCANCODE_L : c_int = 15;
const SDL_SCANCODE_M : c_int = 16;
const SDL_SCANCODE_N : c_int = 17;
const SDL_SCANCODE_O : c_int = 18;
const SDL_SCANCODE_P : c_int = 19;
const SDL_SCANCODE_Q : c_int = 20;
const SDL_SCANCODE_R : c_int = 21;
const SDL_SCANCODE_S : c_int = 22;
const SDL_SCANCODE_T : c_int = 23;
const SDL_SCANCODE_U : c_int = 24;
const SDL_SCANCODE_V : c_int = 25;
const SDL_SCANCODE_W : c_int = 26;
const SDL_SCANCODE_X : c_int = 27;
const SDL_SCANCODE_Y : c_int = 28;
const SDL_SCANCODE_Z : c_int = 29;

const SDL_SCANCODE_1 : c_int = 30;
const SDL_SCANCODE_2 : c_int = 31;
const SDL_SCANCODE_3 : c_int = 32;
const SDL_SCANCODE_4 : c_int = 33;
const SDL_SCANCODE_5 : c_int = 34;
const SDL_SCANCODE_6 : c_int = 35;
const SDL_SCANCODE_7 : c_int = 36;
const SDL_SCANCODE_8 : c_int = 37;
const SDL_SCANCODE_9 : c_int = 38;
const SDL_SCANCODE_0 : c_int = 39;

const SDL_SCANCODE_RETURN : c_int = 40;
const SDL_SCANCODE_ESCAPE : c_int = 41;
const SDL_SCANCODE_BACKSPACE : c_int = 42;
const SDL_SCANCODE_TAB : c_int = 43;
const SDL_SCANCODE_SPACE : c_int = 44;

const SDL_SCANCODE_MINUS : c_int = 45;
const SDL_SCANCODE_EQUALS : c_int = 46;

const SDL_SCANCODE_LEFTBRACKET : c_int = 47;
const SDL_SCANCODE_RIGHTBRACKET : c_int = 48;
const SDL_SCANCODE_BACKSLASH : c_int = 49;

const SDL_SCANCODE_NONUSHASH : c_int = 50;

const SDL_SCANCODE_SEMICOLON : c_int = 51;
const SDL_SCANCODE_APOSTROPHE : c_int = 52;
const SDL_SCANCODE_GRAVE : c_int = 53;
const SDL_SCANCODE_COMMA : c_int = 54;
const SDL_SCANCODE_PERIOD : c_int = 55;
const SDL_SCANCODE_SLASH : c_int = 56;
const SDL_SCANCODE_CAPSLOCK : c_int = 57;

const SDL_SCANCODE_F1 : c_int = 58;
const SDL_SCANCODE_F2 : c_int = 59;
const SDL_SCANCODE_F3 : c_int = 60;
const SDL_SCANCODE_F4 : c_int = 61;
const SDL_SCANCODE_F5 : c_int = 62;
const SDL_SCANCODE_F6 : c_int = 63;
const SDL_SCANCODE_F7 : c_int = 64;
const SDL_SCANCODE_F8 : c_int = 65;
const SDL_SCANCODE_F9 : c_int = 66;
const SDL_SCANCODE_F10 : c_int = 67;
const SDL_SCANCODE_F11 : c_int = 68;
const SDL_SCANCODE_F12 : c_int = 69;

const SDL_SCANCODE_PRINTSCREEN : c_int = 70;
const SDL_SCANCODE_SCROLLLOCK : c_int = 71;
const SDL_SCANCODE_PAUSE : c_int = 72;
const SDL_SCANCODE_INSERT : c_int = 73;

const SDL_SCANCODE_HOME : c_int = 74;
const SDL_SCANCODE_PAGEUP : c_int = 75;
const SDL_SCANCODE_DELETE : c_int = 76;
const SDL_SCANCODE_END : c_int = 77;
const SDL_SCANCODE_PAGEDOWN : c_int = 78;
const SDL_SCANCODE_RIGHT : c_int = 79;
const SDL_SCANCODE_LEFT : c_int = 80;
const SDL_SCANCODE_DOWN : c_int = 81;
const SDL_SCANCODE_UP : c_int = 82;


#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_KeyboardEvent {
    event_type : u32,
    timestamp : u32,
    window_id : u32,
    state : u8,
    repeat : u8,
    key : SDL_Keysym,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SDL_QuitEvent {
    event_type : u32,
    timestamp : u32,
}

const SDL_WINDOWEVENT : u32 = 0x200;
const SDL_KEYDOWN : u32 = 0x300;
const SDL_KEYUP : u32 = 0x301;
const SDL_QUIT : u32 = 0x100;

#[repr(C)]
union SDL_Event {
    event_type : u32,
    window : SDL_WindowEvent,
    keyboard : SDL_KeyboardEvent,
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
                        SDL_WINDOWEVENT => {
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
