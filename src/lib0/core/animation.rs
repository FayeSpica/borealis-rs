use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::lib::core::time::{empty_end_fn, empty_fn, FiniteTicking, get_cpu_time_msec, Ticking, TickingEndCallback, TickingGenericCallback, TickingTickCallback, Timestamp};

// An animatable is a float which value can be animated from an initial value to a target value,
// during a given amount of time. An easing function can also be specified.
//
// Declare the animatable and then use reset(initialValue) to reset the animation.
// Add as many steps as you like by calling addStep(targetValue, duration, easing) one or multiple times.
// Then, start and stop the animation with start() and stop().
//
// setEndCallback() and setTickCallback() allow you to execute code as long as the animation runs and / or once when it finishes.
// Use .getValue() to get the current value at any time.
//
// An animatable has overloads for float conversion, comparison (==) and assignment operator (=) to allow
// basic usage as a simple float. Assignment operator is a shortcut to the reset() method.
pub struct Animatable {
    running: bool,
    end_callback: TickingEndCallback,
    tick_callback: TickingTickCallback,
    callback: TickingGenericCallback,
    period: Timestamp,
    progress: Timestamp,
    current_value: f32,
}

impl Ticking for Animatable {
    fn set_end_callback(&mut self, end_callback: TickingEndCallback) {
        todo!()
    }

    fn set_tick_callback(&mut self, tick_callback: TickingTickCallback) {
        todo!()
    }

    fn get_end_callback(&mut self) -> TickingEndCallback {
        todo!()
    }

    fn get_tick_callback(&mut self) -> TickingTickCallback {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }

    fn set_running(&mut self, running: bool) {
        todo!()
    }

    fn on_update(&mut self, delta: Timestamp) -> bool {
        todo!()
    }

    fn on_start(&mut self) {
        todo!()
    }

    fn on_stop(&self) {
        todo!()
    }
}

impl FiniteTicking for Animatable {
    fn on_rewind(&mut self) {
        todo!()
    }

    fn on_reset(&mut self) {
        todo!()
    }
}

impl Animatable {

    /**
     * Creates an animatable with the given initial value.
     */
    pub fn new(value: f32) -> Animatable {
       Animatable {
           running: false,
           end_callback: empty_end_fn,
           tick_callback: empty_fn,
           callback: empty_fn,
           period: 0,
           progress: 0,
           current_value: 0.0,
       }
    }

    /**
     * Returns the current animatable value.
     */
    pub fn get_value(&self) -> f32 {
        self.current_value
    }

    /**
     * Stops and resets the animation, going back to the given initial value.
     * All steps are removed.
     * If an animation was already ongoing for that animatable, its end callback
     * will be called.
     */
    fn reset_with_value(&mut self, initial_value: f32) {
        self.current_value = initial_value;
    }

    /**
     * Adds an animation step to the target value, lasting the specified duration in milliseconds.
     *
     * An animation can have multiple steps. Target value can be greater and lower than the previous step (it can go forwards or backwards).
     * Easing function is optional, default is EasingFunction::linear.
     *
     * Duration is int32_t due to internal limitations, so a step cannot last for longer than 2 147 483 647ms.
     * The sum of the duration of all steps cannot exceed 71582min.
     */
    fn add_step(&self, target_value: f32, duration: i32) {

    }

    /**
     * Returns the progress of the animation between 0.0f and 1.0f.
     */
    fn get_progress(&self) {

    }
}

static HIGHLIGHT_SPEED: f32 = 125.0;

static HIGHLIGHT_GRADIENT_X: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{
    Arc::new(Mutex::new(0.0))
});

static HIGHLIGHT_GRADIENT_Y: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{
    Arc::new(Mutex::new(0.0))
});

static HIGHLIGHT_COLOR: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{
    Arc::new(Mutex::new(0.0))
});

pub fn update_highlight_animation() {
    let current_time: Timestamp = get_cpu_time_msec();

    // Update variables
    *HIGHLIGHT_GRADIENT_X.lock().unwrap() = (f32::cos(current_time as f32 / HIGHLIGHT_SPEED/3.0) + 1.0) / 2.0;
    *HIGHLIGHT_GRADIENT_Y.lock().unwrap() = (f32::sin(current_time as f32 / HIGHLIGHT_SPEED/3.0) + 1.0) / 2.0;
    *HIGHLIGHT_COLOR.lock().unwrap() = (f32::sin(current_time as f32 / HIGHLIGHT_SPEED * 2.0) + 1.0) / 2.0;
}

pub fn get_highlight_animation() -> (f32, f32, f32) {
    (*HIGHLIGHT_GRADIENT_X.lock().unwrap(), *HIGHLIGHT_GRADIENT_Y.lock().unwrap(), *HIGHLIGHT_COLOR.lock().unwrap())
}