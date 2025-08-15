use wut::gamepad::{Button, State};

#[derive(Debug)]
pub struct Controls {
    pub up: Button,
    pub down: Button,
    pub left: Button,
    pub right: Button,
    pub accept: Button,
    pub cancel: Button,
}

impl Controls {
    pub fn check(&self, input: &State) -> Action {
        if input.trigger.contains(self.up) {
            Action::Up
        } else if input.trigger.contains(self.down) {
            Action::Down
        } else if input.trigger.contains(self.left) {
            Action::Left
        } else if input.trigger.contains(self.right) {
            Action::Right
        } else if input.trigger.contains(self.accept) {
            Action::Accept
        } else if input.trigger.contains(self.cancel) {
            Action::Cancel
        } else {
            Action::None
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            up: Button::Up,
            down: Button::Down,
            left: Button::Left,
            right: Button::Right,
            accept: Button::A,
            cancel: Button::B,
        }
    }
}

pub enum Action {
    None,
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
}
