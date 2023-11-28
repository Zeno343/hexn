use crate::sys::*;

pub enum WindowEvent {
    Quit,
    Keyboard {
        down: bool,
        timestamp: u32,
        keycode: KeyCode,
        modifier: u32,
    },
}

#[repr(u32)]
pub enum KeyCode {
    Tab = SDLK_TAB,
    None,
}
#[derive(Copy, Clone)]
pub struct WindowEvents {
    _priv: (),
}

impl WindowEvents {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Iterator for WindowEvents {
    type Item = WindowEvent;

    fn next(&mut self) -> Option<WindowEvent> {
        unsafe {
            let mut event = core::mem::zeroed();

            if SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_QUIT => Some(WindowEvent::Quit),
                    SDL_KEYDOWN | SDL_KEYUP => {
                        let SDL_KeyboardEvent {
                            type_,
                            timestamp,
                            keysym: SDL_Keysym { sym, mod_, .. },
                            ..
                        } = event.key;

                        let down = match type_ {
                            SDL_KEYDOWN => true,
                            SDL_KEYUP => false,
                            _ => panic!(),
                        };

                        Some(WindowEvent::Keyboard {
                            down,
                            timestamp,
                            keycode: match sym as u32 {
                                SDLK_TAB => KeyCode::Tab,
                                _ => KeyCode::None,
                            },
                            modifier: mod_ as u32,
                        })
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}
