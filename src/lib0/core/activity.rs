use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::actions::{ActionIdentifier, ActionListener};
use crate::lib::core::audio::Sound;
use crate::lib::core::input::ControllerButton;
use crate::lib::core::base_view::{TransitionAnimation, BaseView};
use crate::lib::core::view::View;

// An activity is a "screen" of your app in which the library adds
// the UI components. The app is made of a stack of activities, each activity
// containing a views tree.
pub trait Activity {

    /**
     * Sets the content view of this activity, aka
     * the root view of the tree.
     *
     * When the activity is pushed, setContentView() is
     * automatically called with the result of createContentView().
     * As such, you should override createContentView() if you want
     * to use XML in your activity.
     */
    fn set_content_view(&mut self,  view: Rc<RefCell<Option<Box<dyn View>>>>);

    /**
     * Called when the activity is created. Should return the activity content view, if any.
     *
     * Returning nullptr means the content is not known when the activity is created.
     *
     * You can use View::createFromXMLFile(), View::createFromXMLResource() and View::createFromXMLString() to load
     * a view from respectively an XML file path, an XML name in the resources/xml romfs directory and an XML string.
     *
     * The CONTENT_FROM_XML_FILE, CONTENT_FROM_XML_RES, CONTENT_FROM_XML_STR macros
     * are made to make this process simpler: just use them in the public block of your activity
     * header and it will override createContentView() with the right code for you.
     *
     * The onContentAvailable() method will be called once the content has been created, so that
     * you can get the references to the activity views (by id).
     */
    fn create_content_view(&self) ->  Rc<RefCell<Option<Box<dyn View>>>>;

    /**
     * Called when the content view is created, so that
     * you can get the references to the activity views (by id).
     */
    fn on_content_available(&self);

    fn get_content_view(&self) ->  Rc<RefCell<Option<Box<dyn View>>>>;

    /**
     * Returns the view with the corresponding id, or nullptr
     * if it hasn't been found in the activity.
     */
    fn get_view(&self, id: &str) -> Rc<RefCell<Option<Box<dyn View>>>>;

    /**
     * Resizes the activity to fit the window. Called when the activity
     * is created and when the window is resized (Switch dock counts as window resize).
     */
    fn resize_to_fit_window(&mut self) {
        // match self.get_content_view().borrow_mut() {
        //     None => {}
        //     Some(contentView) => {
        //         // contentView.set_dimensions()
        //     }
        // }
    }

    /**
     * Returns the duration of the activity show / hide animation.
     */
    fn get_show_animation_duration(&self, animation: TransitionAnimation) -> f32 {
        // Implementation for getting the show animation duration
        unimplemented!()
    }

    /**
     * Is this activity translucent, aka can we see the
     * activities under it in the stack?
     */
    fn is_translucent(&self) -> bool {
        // Implementation for checking if the activity is translucent
        unimplemented!()
    }

    fn will_appear(&self, reset_state: bool) {
        // Implementation for the willAppear method
        unimplemented!()
    }

    fn will_disappear(&self, reset_state: bool) {
        // Implementation for the willDisappear method
        unimplemented!()
    }

    /**
     * If set to true, will force the activity to be translucent.
     */
    fn set_in_fade_animation(&self, translucent: bool) {
        // Implementation for setting in fade animation
        unimplemented!()
    }

    /**
     * Shows the activity with a fade in animation, or no animation at all.
     */
    fn show(&self, cb: fn(), animate: bool, animation_duration: f32) {
        // Implementation for showing the activity
        unimplemented!()
    }

    /**
     * Hides the activity with a fade in animation, or no animation at all.
     */
    fn hide(&self, cb: Box<dyn Fn()>, animate: bool, animation_duration: f32) {
        // Implementation for hiding the activity
        unimplemented!()
    }

    fn is_hidden(&self) -> bool {
        // Implementation for checking if the activity is hidden
        unimplemented!()
    }

    /**
     * Registers an action with the given parameters on the content view. The listener will be fired
     * when the user presses the key.
     *
     * The listener should return true if the action was consumed, false otherwise.
     * The sound will only be played if the listener returned true.
     *
     * A hidden action will not show up in the bottom-right hints.
     *
     * Must be called after the content view is set.
     *
     * Returns the identifier for the action, so it can be unregistered later on. Returns ACTION_NONE if the
     * action was not registered.
     */
    fn register_action(
        &self,
        hint_text: &str,
        button: ControllerButton,
        action_listener: ActionListener,
        hidden: bool,
        sound: Sound,
    ) -> ActionIdentifier {
        // Implementation for registering an action
        unimplemented!()
    }

    /**
     * Unregisters an action with the given identifier on the content view.
     *
     * Must be called after the content view is set.
     */
    fn unregister_action(&self, identifier: ActionIdentifier) {
        // Implementation for unregistering an action
        unimplemented!()
    }

    /**
     * Registers an action to exit the application with the default button BUTTON_START.
     *
     * Must be called after the content view is set.
     *
     * Returns the identifier for the action, so it can be unregistered later on. Returns ACTION_NONE if the
     * action was not registered.
     */
    fn register_exit_action(&self, button: ControllerButton) -> ActionIdentifier {
        // Implementation for registering an exit action
        unimplemented!()
    }

    fn on_window_size_changed(&self) {
        // Implementation for handling window size change
        unimplemented!()
    }

    fn get_default_focus(&self) -> Option<Box<dyn View>> {
        // Implementation for getting the default focus
        unimplemented!()
    }

    fn set_alpha(&self, alpha: f32) {
        // Implementation for setting alpha
        unimplemented!()
    }
}