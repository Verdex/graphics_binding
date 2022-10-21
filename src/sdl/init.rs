
pub const SDL_INIT_TIMER : u32 = 0x01;
pub const SDL_INIT_AUDIO : u32 = 0x10;
pub const SDL_INIT_VIDEO : u32 = 0x20;
pub const SDL_INIT_JOYSTICK : u32 = 0x02_00;
pub const SDL_INIT_HAPTIC : u32 = 0x10_00;
pub const SDL_INIT_GAMECONTROLLER : u32 = 0x20_00;
pub const SDL_INIT_EVENTS : u32 = 0x40_00;
pub const SDL_INIT_SENSOR : u32 = 0x80_00;
pub const SDL_INIT_NOPARACHUTE : u32 = 0x10_00_00;
pub const SDL_INIT_EVERYTHING : u32 = SDL_INIT_TIMER 
                                    | SDL_INIT_AUDIO
                                    | SDL_INIT_VIDEO
                                    | SDL_INIT_EVENTS
                                    | SDL_INIT_JOYSTICK
                                    | SDL_INIT_HAPTIC
                                    | SDL_INIT_GAMECONTROLLER
                                    | SDL_INIT_SENSOR;