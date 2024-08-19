use std::cell::RefCell;
use std::rc::Rc;
use glfw::{Action, GamepadState, Glfw, JoystickEvent};
use log::info;
use once_cell::sync::Lazy;
use crate::lib::core::input::{ControllerState, InputManager};

// Input manager for GLFW gamepad and keyboard
pub struct GLFWInputManager {
    g: Rc<RefCell<Glfw>>,
}

fn glfw_joystick_call_back(id: glfw::JoystickId, event: glfw::JoystickEvent) {
    match event {
        JoystickEvent::Connected => {
            info!("glfw: joystick {:#?} connected", id);
        }
        JoystickEvent::Disconnected => {
            info!("glfw: joystick {:#?} disconnected", id);
        }
    }
}

impl GLFWInputManager {
    pub fn new(g: Rc<RefCell<Glfw>>) -> Self {

        g.borrow_mut().set_joystick_callback(glfw_joystick_call_back);

        GLFWInputManager {
            g,
        }
    }
}

// LT and RT do not exist here because they are axes
pub static GLFW_BUTTONS_MAPPING: Lazy<Vec<glfw::GamepadButton>> = Lazy::new(||{
    vec![
        glfw::GamepadButton::ButtonA,
        glfw::GamepadButton::ButtonB,
        glfw::GamepadButton::ButtonX,
        glfw::GamepadButton::ButtonY,
        glfw::GamepadButton::ButtonLeftBumper,
        glfw::GamepadButton::ButtonRightBumper,
        glfw::GamepadButton::ButtonBack,
        glfw::GamepadButton::ButtonStart,
        glfw::GamepadButton::ButtonGuide,
        glfw::GamepadButton::ButtonLeftThumb,
        glfw::GamepadButton::ButtonRightThumb,
        glfw::GamepadButton::ButtonDpadUp,
        glfw::GamepadButton::ButtonDpadRight,
        glfw::GamepadButton::ButtonDpadDown,
        glfw::GamepadButton::ButtonDpadLeft,
    ]
});

// LT and RT do not exist here because they are axes
pub static GLFW_GAMEPAD_TO_KEYBOARD: Lazy<Vec<glfw::Key>> = Lazy::new(||{
    vec![
        glfw::Key::Enter,
        glfw::Key::Backspace,
        glfw::Key::Unknown,
        glfw::Key::Unknown,
        glfw::Key::Unknown,
        glfw::Key::Unknown,
        glfw::Key::F1,
        glfw::Key::Escape,
        glfw::Key::Unknown,
        glfw::Key::Unknown,
        glfw::Key::Unknown,
        glfw::Key::Up,
        glfw::Key::Right,
        glfw::Key::Down,
        glfw::Key::Left,
    ]
});

impl InputManager for GLFWInputManager {
    fn get_controller_state(&self) -> ControllerState {
        let mut state = ControllerState::new();
        let joystick = self.g.borrow_mut().get_joystick(glfw::JoystickId::Joystick1);
        if joystick.is_present() {
            match joystick.get_gamepad_state() {
                None => {}
                Some(gamepad_state) => {
                    for i in 0..GLFW_BUTTONS_MAPPING.len() {
                        // Add keyboard keys on top of gamepad buttons
                        let key = GLFW_GAMEPAD_TO_KEYBOARD[i];
                        if key != glfw::Key::Unknown {
                            // todo!()
                        }

                        // Translate GLFW gamepad to borealis controller
                        let glfw_gamepad_button = GLFW_BUTTONS_MAPPING[i];
                        match gamepad_state.get_button_state(glfw_gamepad_button) {
                            Action::Release => {
                                state.set_button(i, true);
                            }
                            Action::Press => {}
                            Action::Repeat => {}
                        }
                    }
                }
            }
        }
        state
    }
}