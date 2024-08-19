use std::convert::Into;
use glfw::Glfw;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

// Abstract buttons enum - names correspond to a generic Xbox controller
// LT and RT should not be buttons but for the sake of simplicity we'll assume they are.
// Similarly, DPAD (also called HAT) is assumed to be buttons here.
#[derive(Debug, PartialEq, EnumCountMacro, EnumIter)]
pub enum ControllerButton {
    ButtonLt = 0,
    ButtonLb,

    ButtonLsb,

    ButtonUp,
    ButtonRight,
    ButtonDown,
    ButtonLeft,

    ButtonBack,
    ButtonGuide,
    ButtonStart,

    ButtonRsb,

    ButtonY,
    ButtonB,
    ButtonA,
    ButtonX,

    ButtonRb,
    ButtonRt,
}

// Abstract axis enum - names correspond to a generic Xbox controller
#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum ControllerAxis
{
    LeftX,
    LeftY,

    // No Z axis, LT and RT are in the buttons enum for the sake of simplicity

    RightX, // also called 5th axis
    RightY, // also called 4th axis
}

pub struct ControllerState{
    pub buttons: [bool; ControllerButton::COUNT],
    pub axes: [f64; ControllerAxis::COUNT],
}

impl ControllerState {
    pub fn new() -> Self {
        ControllerState {
            buttons: <[bool; 17]>::try_from(vec![false; ControllerButton::COUNT]).unwrap(),
            axes: <[f64; 4]>::try_from(vec![0f64; ControllerAxis::COUNT]).unwrap()
        }
    }

    pub fn set_button(&mut self, i: usize, valid: bool) {
        self.buttons[i] = valid;
    }
}

// Interface responsible for reporting input state to the application - button presses,
// axis position and touch screen state
pub trait InputManager {

    /**
     * Called once every frame to fill the given ControllerState struct with the controller state.
     */
    fn get_controller_state(&self, g: &Glfw) -> ControllerState;
}

