use crate::models::Input;
use ggez::input::keyboard::KeyCode;

impl Input {
    pub fn from_keycode(keycode: &KeyCode) -> Option<Input> {
        match keycode {
            KeyCode::Up => Some(Input::UP),
            KeyCode::Right => Some(Input::RIGHT),
            KeyCode::Down => Some(Input::DOWN),
            KeyCode::Left => Some(Input::LEFT),
            KeyCode::Space => Some(Input::SPACE),
            KeyCode::R => Some(Input::R),
            _ => None
        }
    }
}