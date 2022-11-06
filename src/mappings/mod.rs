use crate::models::Input;
use ggez::input::keyboard::KeyCode;

impl Input {
    pub fn from_keycode(keycode: &KeyCode) -> Option<Input> {
        match keycode {
            KeyCode::Z => Some(Input::UP),
            KeyCode::D => Some(Input::RIGHT),
            KeyCode::S => Some(Input::DOWN),
            KeyCode::Q => Some(Input::LEFT),
            _ => None
        }
    }
}