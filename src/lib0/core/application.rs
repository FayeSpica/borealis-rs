use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::c_float;
use std::mem::swap;
use std::rc::Rc;
use glfw::Key::N;
use log::{debug, error, info, warn};
use nanovg_sys::NVGcontext;
use rust_i18n::t;
use crate::lib::core::actions::ActionIdentifier;
use crate::lib::core::activity::Activity;
use crate::lib::core::audio::{AudioPlayer, Sound};
use crate::lib::core::font::{FONT_INVALID, FontStash};
use crate::lib::core::frame_context::FrameContext;
use crate::lib::core::input::{ControllerButton, ControllerState};
use crate::lib::core::input::ControllerButton::{ButtonA, ButtonBack, ButtonStart};
use crate::lib::core::platform::Platform;
use crate::lib::core::theme::{get_dark_theme, get_light_theme, Theme, ThemeVariant};
use crate::lib::core::time::{get_cpu_time_usec, Timestamp};
use crate::lib::core::base_view::{FocusDirection, TransitionAnimation};
use crate::lib::core::r#box::{BoxView, Padding};
use crate::lib::core::view::{GenericEvent, View, VoidEvent};
use crate::lib::platforms::platform::GlfwPlatform;
use crate::lib::views::applet_frame::AppletFrame;
use crate::lib::views::button::Button;
use crate::lib::views::header::Header;
use crate::lib::views::image::Image;
use crate::lib::views::label::Label;
use crate::lib::views::rectangle::Rectangle;
use crate::lib::views::scrolling_frame::ScrollingFrame;
use crate::lib::views::sidebar::Sidebar;
use crate::lib::views::tab_frame::TabFrame;

// Constants used for scaling as well as
// creating a window of the right size on PC
const ORIGINAL_WINDOW_WIDTH: u32  = 1280;
const ORIGINAL_WINDOW_HEIGHT: u32 = 720;

pub struct Application {
    quit_requested: bool,
    platform: Rc<RefCell<Box<dyn Platform>>>,
    title: String,
    window_width: u32,
    window_height: u32,
    content_width: u32,
    content_height: u32,
    old_controller_state: ControllerState,
    block_inputs_tokens: i32,
    current_focus: Rc<RefCell<Option<Box<dyn View>>>>,
    repetition_old_focus: Rc<RefCell<Option<Box<dyn View>>>>,
    activities_stack: VecDeque<Rc<RefCell<Box<dyn Activity>>>>,
    focus_stack: VecDeque<Rc<RefCell<Box<dyn View>>>>,
    audio_player: Rc<RefCell<Option<Box<dyn AudioPlayer>>>>,
    font_stash: Option<FontStash>,
    window_scale: f32,

    global_quit_enabled: bool,
    global_quit_identifier: ActionIdentifier,

    common_footer: String,


    global_focus_change_event: GenericEvent,
    global_hints_update_event: VoidEvent,
    xml_views_register: HashMap<String, XMLViewCreator>,
}

static BUTTON_REPEAT_DELAY: i32 = 15;
static BUTTON_REPEAT_CADENCY: i32 = 5;

type XMLViewCreator = fn() -> Rc<RefCell<Box<dyn View>>>;

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Application {
            quit_requested: false,
            platform: Rc::new(RefCell::new(Box::new(GlfwPlatform::new(title, width,  height)))),
            title: title.into(),
            window_width: width,
            window_height: height,
            content_width: 0,
            content_height: 0,
            old_controller_state: ControllerState::new(),
            block_inputs_tokens: 0,
            current_focus: Rc::new(RefCell::new(None)),
            repetition_old_focus: Rc::new(RefCell::new(None)),
            activities_stack:  VecDeque::new(),
            focus_stack: VecDeque::new(),
            audio_player: Rc::new(RefCell::new(None)),
            font_stash: None,
            window_scale: 1f32,
            global_quit_enabled: false,
            global_quit_identifier: 0,
            common_footer: "".to_string(),
            global_focus_change_event: GenericEvent::new(),
            global_hints_update_event: VoidEvent::new(),
            xml_views_register: Default::default(),
        }
    }

    pub fn main_loop(&mut self) -> bool {

        // Main loop callback
        if !self.platform.borrow_mut().main_loop_iteration() || self.quit_requested {
            self.exit();
            return false
        }

        // Input
        let input_manager = self.platform.borrow_mut().get_input_manager().borrow_mut();
        let controller_state = input_manager.get_controller_state();

        // Trigger controller events
        let mut any_button_pressed = false;
        let mut repeating = false;
        let mut button_press_time: Timestamp = 0;
        let mut repeating_button_timer: i32 = 0;

        for i in 0..controller_state.buttons.len() {
            if controller_state.buttons[i] {
                any_button_pressed = true;
                repeating = repeating_button_timer > BUTTON_REPEAT_DELAY && repeating_button_timer % BUTTON_REPEAT_CADENCY == 0;

                if !self.old_controller_state.buttons[i] || repeating {
                    // self.on_controller_button_pressed(i, repeating);
                }
            }

            if controller_state.buttons[i] != self.old_controller_state.buttons[i] {
                repeating_button_timer = 0;
                button_press_time = 0;
            }
        }

        if any_button_pressed && get_cpu_time_usec() - button_press_time > 1000 {
            button_press_time = get_cpu_time_usec();
            repeating_button_timer +=1; // Increased once every ~1ms
        }

        self.old_controller_state = controller_state;

        // Animations
        self.update_highlight_animation();
        // Ticking::updateTickings();

        // Render
        self.frame();
        true
    }

    pub fn get_platform(&self) -> Rc<RefCell<Box<dyn Platform>>> {
        Rc::clone(&self.platform)
    }

    pub fn get_audio_player(&self) -> Rc<RefCell<Box<dyn AudioPlayer>>> {
        self.platform.borrow_mut().get_audio_player()
    }

    pub fn quit(&mut self) {
        self.quit_requested = true;
    }

    pub fn navigate(&mut self, direction: FocusDirection) {
        // if let Some(mut current_focus) = &self.current_focus {
        //     let mut next_focus: Option<T>  = None;
        //
        //     // Handle custom navigation routes
        //     // By View ptr
        //     if current_focus.has_custom_navigation_route_by_ptr(direction) {
        //         next_focus = current_focus.get_custom_navigation_route_ptr(direction);
        //
        //         if next_focus.is_none() {
        //             warn!("Tried to follow a navigation route that leads to a None view! (from=\"{}\", direction={})", current_focus.describe(), direction as i32);
        //         }
        //     }
        //     // By ID
        //     else if current_focus.has_custom_navigation_route_by_id(direction) {
        //         if let Some(id) = current_focus.get_custom_navigation_route_id(direction) {
        //             next_focus = current_focus.get_nearest_view(&id);
        //
        //             if next_focus.is_none() {
        //                 warn!("Tried to follow a navigation route that leads to an unknown view ID! (from=\"{}\", direction={}, targetId=\"{}\")", current_focus.describe(), direction as i32, id);
        //             }
        //         }
        //     }
        //     // Do nothing if current focus doesn't have a parent
        //     // (in which case there is nothing to traverse)
        //     else if let Some(mut current_focus) = current_focus.get_parent() {
        //         // Get next view to focus by traversing the views tree upwards
        //         next_focus = current_focus.get_next_focus(direction, &current_focus);
        //
        //         while next_focus.is_none() {
        //             if !current_focus.has_parent() || !current_focus.get_parent().unwrap().has_parent() {
        //                 // stop when we reach the root of the tree
        //                 break;
        //             }
        //
        //             current_focus = current_focus.get_parent().unwrap();
        //             next_focus = current_focus.get_parent().unwrap().get_next_focus(direction, &current_focus);
        //         }
        //     }
        //
        //     // No view to focus at the end of the traversal: wiggle and return
        //     if next_focus.is_none() {
        //         if let Some(mut current_focus) = &self.current_focus.get_mut() {
        //             current_focus.shake_highlight(direction);
        //         }
        //         if let Some(audio_player) = &self.audio_player.get_mut() {
        //             audio_player.play(Sound::SoundFocusError);
        //         }
        //         return;
        //     }
        //
        //     // Otherwise play sound and give it focus
        //     if let Some(audio_player) = &self.audio_player.get_mut() {
        //         if let Some(focus_sound) = next_focus.as_ref().unwrap().get_focus_sound() {
        //             audio_player.play(*focus_sound);
        //         }
        //     }
        //     self.give_focus(next_focus.unwrap());
        // }
    }


    pub fn on_controller_button_pressed(&mut self, button: ControllerButton, repeating: bool) {
        if self.block_inputs_tokens != 0 {
            debug!("{:?} button press blocked (tokens={})", button, self.block_inputs_tokens);
        }

        // let mut repetition_old_focus = self.repetition_old_focus.borrow_mut();
        // if repeating && repetition_old_focus == self.current_focus.borrow_mut() {
        //     return;
        // }

        self.repetition_old_focus = Rc::clone(&self.current_focus);

        // Actions
        if self.handle_action(button) {
            return;
        }

        // Navigation
        // Only navigate if the button hasn't been consumed by an action
        // (allows overriding DPAD buttons using actions)
        match button {
            ControllerButton::ButtonLt => {}
            ControllerButton::ButtonLb => {}
            ControllerButton::ButtonLsb => {}
            ControllerButton::ButtonUp => {
                self.navigate(FocusDirection::Up);
            }
            ControllerButton::ButtonRight => {
                self.navigate(FocusDirection::Right);
            }
            ControllerButton::ButtonDown => {
                self.navigate(FocusDirection::Down);
            }
            ControllerButton::ButtonLeft => {
                self.navigate(FocusDirection::Left);
            }
            ControllerButton::ButtonBack => {}
            ControllerButton::ButtonGuide => {}
            ControllerButton::ButtonStart => {}
            ControllerButton::ButtonRsb => {}
            ControllerButton::ButtonY => {}
            ControllerButton::ButtonB => {}
            ControllerButton::ButtonA => {}
            ControllerButton::ButtonX => {}
            ControllerButton::ButtonRb => {}
            ControllerButton::ButtonRt => {}
        }
    }


    pub fn get_current_focus(&mut self) -> Rc<RefCell<Option<Box<dyn View>>>> {
        Rc::clone(&self.current_focus)
    }

    pub fn handle_action(&mut self, button: ControllerButton) -> bool {
        // if self.activities_stack.is_empty() {
        //     return false;
        // }
        //
        // let mut hint_parent = self.current_focus;
        // let mut consumed_buttons = HashSet::new();
        //
        // if hint_parent.is_none() {
        //     hint_parent = self.activities_stack.get(self.activities_stack.len() - 1).unwrap()
        //         .get_content_view();
        // }
        //
        // while let Some(mut current_hint_parent) = hint_parent {
        //     for action in current_hint_parent.get_actions() {
        //         if action.button != button {
        //             continue;
        //         }
        //
        //         if consumed_buttons.contains(&action.button) {
        //             continue;
        //         }
        //
        //         if action.available {
        //             if action.action_listener(&mut current_hint_parent) {
        //                 if button == ButtonA {
        //                     current_hint_parent.reset_click_animation();
        //                 }
        //
        //                 self.get_audio_player().borrow_mut().play(action.sound.clone());
        //
        //                 consumed_buttons.insert(action.button.clone());
        //             }
        //         }
        //     }
        //
        //     hint_parent = current_hint_parent.get_parent();
        // }
        //
        // // Only play the error sound if action is a click
        // if button == ButtonA && consumed_buttons.is_empty() {
        //     self.get_audio_player().borrow_mut().play(Sound::SoundClickError);
        // }
        //
        // !consumed_buttons.is_empty()
        true
    }

    pub fn frame(&self) {
        todo!();
        let video_context = self.platform.borrow_mut().get_video_context().borrow_mut();
        let mut frame_context = FrameContext::new(
            self.get_nvg_context(),
            (self.window_width / self.window_height) as f32,
            &self.font_stash.unwrap(),
            self.get_theme(),
        );

        // Begin frame and clear
        let background_color = frame_context.theme.get_color("brls/background").unwrap();
        video_context.begin_frame();
        video_context.clear(background_color.clone());

        unsafe {
            nanovg_sys::nvgBeginFrame(self.get_nvg_context().borrow_mut().raw(), self.window_width as c_float, self.window_height as c_float, frame_context.pixel_ratio);
            nanovg_sys::nvgScale(self.get_nvg_context().borrow_mut().raw(), self.window_scale, self.window_scale);
        }

        let mut views_to_draw: Vec<Rc<RefCell<Option<Box<dyn View>>>>> = Vec::new();

        // Draw all activities in the stack
        // until we find one that's not translucent
        for i in 0..self.activities_stack.len() {
            let activity = &self.activities_stack[self.activities_stack.len() - 1 - i];

            views_to_draw.push(activity.borrow_mut().get_content_view());

            if !activity.borrow().is_translucent() {
                break;
            }
        }

        // for i in 0..views_to_draw.len() {
        //     let view = views_to_draw.get(views_to_draw.len() - 1 - i).unwrap();
        //     view.borrow_mut().unwrap().frame(&mut frame_context);
        // }

        // End frame
        unsafe {
            nanovg_sys::nvgResetTransform(self.get_nvg_context().borrow_mut().raw());
            nanovg_sys::nvgEndFrame(self.get_nvg_context().borrow_mut().raw());
        }

        self.platform.borrow_mut().get_video_context().borrow_mut().end_frame();
    }

    pub fn exit(&mut self) {
        info!("Exiting...");
        self.clear();
    }

    pub fn set_display_framerate(&self, enabled: bool) {
        // To be implemented
    }

    pub fn toggle_framerate_display(&self) {
        // To be implemented (call setDisplayFramerate)
    }

    pub fn register_fps_toggle_action(&self, activity: &mut dyn Activity) {
        // activity.register_action("FPS", ButtonBack, , true)
    }

    pub fn set_global_quit(&mut self, enabled: bool) {
        self.global_quit_enabled = enabled;
        // for activity in self.activities_stack {
        //     match enabled {
        //         true => {
        //             self.global_quit_identifier = activity.borrow_mut().register_exit_action(ButtonStart);
        //         }
        //         false => {
        //             activity.borrow_mut().unregister_action(self.global_quit_identifier);
        //         }
        //     }
        // }
    }

    pub fn set_global_fps_toggle(&mut self, enabled: bool) {

    }

    pub fn notify(&mut self, text: &str) {
        // To be implemented
    }

    pub fn give_focus(&mut self, view: Option<Box<dyn View>>) {
        // let old_focus = self.current_focus.take();
        // let new_focus = view.map(|v| v.get_default_focus());
        //
        // if old_focus.as_ref() != new_focus.as_ref() {
        //     if let Some(old) = old_focus.as_deref() {
        //         old.on_focus_lost();
        //     }
        //
        //     self.current_focus = Rc::new(RefCell::new(new_focus));
        //     self.global_focus_change_event.fire(&Rc::clone(&self.current_focus));
        //
        //     if let Some(new) = new_focus.as_deref() {
        //         new.on_focus_gained();
        //         println!("Giving focus to {}", new.describe());
        //     }
        // }
    }

    pub fn pop_activity(&mut self, animation: TransitionAnimation, cb: fn()) {
        // if self.activities_stack.len() <= 1 {
        //     // never pop the first activity
        //     return;
        // }
        //
        // self.block_inputs();
        //
        // let last = self.activities_stack.pop().unwrap();
        // last.will_disappear(true);
        // last.set_in_fade_animation(true);
        //
        // let wait = animation == TransitionAnimation::Fade;
        //
        // // Hide animation (and show previous activity, if any)
        // last.hide(
        //     move || {
        //         last.set_in_fade_animation(false);
        //         // No need to manually delete in Rust
        //
        //         // Animate the old activity once the new one
        //         // has ended its animation
        //         if !self.activities_stack.is_empty() && wait {
        //             let new_last = self.activities_stack.last().unwrap();
        //
        //             if new_last.is_hidden() {
        //                 new_last.will_appear(false);
        //                 new_last.show(cb, true, new_last.get_show_animation_duration(animation));
        //             } else {
        //                 cb();
        //             }
        //         }
        //
        //         self.unblock_inputs();
        //     },
        //     true,
        //     last.get_show_animation_duration(animation),
        // );
        //
        // // Animate the old activity immediately
        // if !wait && !self.activities_stack.is_empty() {
        //     let to_show : Box<dyn Activity>= self.activities_stack.last().unwrap();
        //     to_show.will_appear(false);
        //     to_show.show(cb.clone(), true, to_show.get_show_animation_duration(animation));
        // }
        //
        // // Focus
        // if let Some(new_focus) = self.focus_stack.pop() {
        //     println!("Giving focus to {}, and removing it from the focus stack", new_focus.describe());
        //     self.give_focus(new_focus);
        // }
    }

    pub fn push_activity(&mut self, mut activity_rc: Rc<RefCell<Box<dyn Activity>>>, animation: TransitionAnimation) {
        self.block_inputs();

        let mut activity = activity_rc.borrow_mut();
        // Create the activity content view
        let content_view = activity.create_content_view();
        activity.on_content_available();
        //
        // // Call hide() on the previous activity in the stack if no
        // // activities are translucent, then call show() once the animation ends
        // let last = self.activities_stack.last().cloned();
        // let fade_out = last.as_ref().map_or(false, |last| !last.is_translucent() && !activity.is_translucent());
        // let wait = animation == TransitionAnimation::Fade;
        //
        // if self.global_quit_enabled {
        //     self.global_quit_identifier = activity.register_exit_action(ButtonStart);
        // }

        // if self.global_fps_toggle_enabled {
        //     self.global_fps_toggle_identifier = self.register_fps_toggle_action(activity.clone());
        // }

        // Fade out animation
        // if fade_out {
        //     activity.set_in_fade_animation(true);
        //
        //     // Animate the new activity directly
        //     if !wait {
        //         activity.show(
        //             || self.unblock_inputs(),
        //             true,
        //             activity.get_show_animation_duration(animation),
        //         );
        //     }
        //
        //     last.unwrap().hide(
        //         || {
        //             let new_last = self.activities_stack.last().unwrap();
        //             new_last.set_in_fade_animation(false);
        //
        //             // Animate the new activity once the old one
        //             // has ended its animation
        //             if wait {
        //                 new_last.show(|| self.unblock_inputs(), true, new_last.get_show_animation_duration(animation));
        //             }
        //         },
        //         true,
        //         last.unwrap().get_show_animation_duration(animation),
        //     );
        // }

        activity.resize_to_fit_window();

        // if !fade_out {
        //     // activity.show(|| self.unblock_inputs(), true, activity.get_show_animation_duration(animation));
        // } else {
        //     activity.set_alpha(0.0);
        // }

        // Focus
        // if let Some(current_focus) = self.current_focus.get_mut() {
        //     println!("Pushing {} to the focus stack", current_focus.describe());
        //     self.focus_stack.push_back(Rc::new(RefCell::new(self.current_focus.borrow_mut().unwrap())));
        // }

        // Layout and prepare activity
        activity.will_appear(true);
        self.give_focus(activity.get_default_focus());

        // And push it
        self.activities_stack.push_back(Rc::clone(&activity_rc));
    }

    pub fn update_highlight_animation(&self) {

    }


    pub fn clear(&mut self) {
        // for activity in self.activities_stack {
        //     activity.borrow_mut().will_appear(true);
        // }

        self.activities_stack.clear();
    }

    pub fn get_theme(&self) -> &Theme {
        if self.get_theme_variant() == ThemeVariant::Light {
            get_light_theme()
        } else {
            get_dark_theme()
        }
    }

    pub fn get_theme_variant(&self) -> ThemeVariant {
        self.platform.borrow().get_theme_variant()
    }

    pub fn get_locale(&self) -> &str {
        "CN"
    }

    fn load_font_from_file(&mut self, font_name: &str, file_path: &str) -> bool {
        let handle = unsafe {
            nanovg_sys::nvgCreateFont(
                self.get_nvg_context().borrow().raw(), // Adjust the type as needed
                font_name.as_ptr() as *const i8,
                file_path.as_ptr() as *const i8,
            )
        };

        if handle == FONT_INVALID {
            error!("Could not load the font \"{}\"", font_name);
            return false;
        }

        // self.font_stash.unwrap().insert(font_name, handle);
        true
    }

    pub fn crash(&self, text: &str) {
        // To be implemented
    }

    pub fn block_inputs(&mut self) {
        self.block_inputs_tokens += 1;
        debug!("Adding an inputs block token (tokens={})", self.block_inputs_tokens);
    }

    pub fn unblock_inputs(&mut self) {
        if self.block_inputs_tokens > 0 {
            self.block_inputs_tokens -= 1;
        }
        debug!("Removing an inputs block token (tokens={})", self.block_inputs_tokens);
    }

    pub fn get_nvg_context(&self) -> Rc<RefCell<nanovg::Context>> {
        todo!()
        // self.platform.borrow_mut().get_video_context().get_mut().get_nvg_context()
    }

    pub fn set_common_footer(&mut self, common_footer: &str) {
        self.common_footer = common_footer.into();
    }

    pub fn get_common_footer(&self) -> &str {
        self.common_footer.as_str()
    }

    pub fn on_window_resized(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;

        // Rescale UI
        self.window_scale = width as f32 / ORIGINAL_WINDOW_WIDTH as f32;

        let content_height: f32 = (height as f32 / (self.window_scale * ORIGINAL_WINDOW_HEIGHT as f32) * ORIGINAL_WINDOW_HEIGHT as f32) as f32;

        self.content_width = ORIGINAL_WINDOW_WIDTH;
        self.content_height = content_height as u32;

        info!("Window size changed to {}x{}", width, height);
        info!("New scale factor is {}", self.window_scale);

        // for activity in self.activities_stack {
        //     activity.borrow_mut().on_window_size_changed();
        // }
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_global_focus_change_event<'a>(&self) -> &'a GenericEvent {
        todo!()
        //&self.global_focus_change_event
    }

    pub fn get_global_hints_update_event<'a>(&self) -> &'a VoidEvent {
        todo!()
        //&self.global_hints_update_event
    }

    pub fn get_font(&mut self, name: &str) {
        todo!()
    }

    pub fn xml_views_register_contains(&mut self, name: &str) -> bool {
        self.xml_views_register.contains_key(name)

    }

    pub fn get_xml_view_creator(&mut self, name: &str) -> &XMLViewCreator {
        self.xml_views_register.get(name).unwrap()
    }

    pub fn register_built_in_xml_views(&mut self) {
        self.register_xml_view("brls:Box", BoxView::create);
        self.register_xml_view("brls:Rectangle", Rectangle::create);
        self.register_xml_view("brls:AppletFrame", AppletFrame::create);
        self.register_xml_view("brls:Label", Label::create);
        self.register_xml_view("brls:TabFrame", TabFrame::create);
        self.register_xml_view("brls:Sidebar", Sidebar::create);
        self.register_xml_view("brls:Header", Header::create);
        self.register_xml_view("brls:ScrollingFrame", ScrollingFrame::create);
        self.register_xml_view("brls:Image", Image::create);
        self.register_xml_view("brls:Padding", Padding::create);
        self.register_xml_view("brls:Button", Button::create);
    }

    pub fn register_xml_view(&mut self, name: &str, creator: XMLViewCreator) {
        self.xml_views_register.insert(name.into(), creator);
    }
}