use std::string::String;
use std::option::Option;
use std::boxed::Box;
use crate::lib::core::audio::Sound;
use crate::lib::core::input::ControllerButton;
use crate::lib::core::view::View;

pub type ActionListener = Box<dyn FnMut(&mut dyn View) -> bool>;
pub type ActionIdentifier = i32;

const ACTION_NONE: ActionIdentifier = -1;

pub struct Action {
    pub button: ControllerButton,
    pub identifier: ActionIdentifier,
    pub hint_text: String,
    pub available: bool,
    pub hidden: bool,
    pub sound: Sound,
    pub action_listener: Option<ActionListener>,
}

impl Action {
    pub(crate) fn new(
        button: ControllerButton,
        identifier: ActionIdentifier,
        hint_text: &str,
        available: bool,
        hidden: bool,
        sound: Sound,
        action_listener: ActionListener,
    ) -> Self {
        Action {
            button,
            identifier,
            hint_text: hint_text.into(),
            available,
            hidden,
            sound,
            action_listener: Some(Box::new(action_listener)),
        }
    }

    fn operator_eq(&self, other: &ControllerButton) -> bool {
        self.button == *other
    }
}

