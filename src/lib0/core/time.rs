use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub type Timestamp = i64;

/**
 * Returns the current CPU time in microseconds.
 */
pub fn get_cpu_time_usec() -> Timestamp {
    chrono::prelude::Utc::now().timestamp_micros()
}

/**
 * Returns the current CPU time in milliseconds.
 */
pub fn get_cpu_time_msec() -> Timestamp {
    chrono::prelude::Utc::now().timestamp_millis()
}

pub type TickingGenericCallback = fn();
pub type TickingTickCallback = fn();
pub type TickingEndCallback = fn(bool);

pub fn empty_fn(){}
pub fn empty_end_fn(t: bool) {}

pub struct TickManager {
    running_tickings: Arc<Mutex<Vec<Arc<Mutex<dyn Ticking + Send + Sync>>>>>
}

impl TickManager {
    pub fn new() -> TickManager {
        TickManager {
            running_tickings: Arc::new(Mutex::new(vec![]))
        }
    }

    /**
     * Called internally by the main loop. Takes all running tickings
     * and updates them.
     */
    fn update_tickings(&self) {
        // Update time
        let mut previous_time: Timestamp = 0;

        let current_time = get_cpu_time_msec();
        let delta = match previous_time == 0 {
            true => {0}
            false => {current_time - previous_time}
        };

        previous_time = current_time;

        // Update every running ticking, kill them and execute cb if they are finished
        // We have to clone the running tickings list to avoid altering it while
        // in the for loop (so if another ticking is started in a callback or during onUpdate())
        let binding = self.running_tickings.lock().unwrap();

        for mut ticking in binding.clone().into_iter() {
            let mut ticking_mutex = ticking.lock().unwrap();
            let run = ticking_mutex.on_update(delta);

            (ticking_mutex.get_tick_callback())();
            if !run {
                ticking_mutex.set_stop(true); // will remove the ticking from RUNNING_TICKINGS
            }
        }
    }
}

// static RUNNING_TICKINGS: Lazy<Mutex<Vec<Arc<Mutex<&mut (dyn Ticking + Send + Sync)>>>>> = Lazy::new(|| {
//     Mutex::new(vec![])
// });

// /**
//  * Called internally by the main loop. Takes all running tickings
//  * and updates them.
//  */
// fn update_tickings() {
//     // Update time
//     let mut previous_time: Timestamp = 0;
//
//     let current_time = get_cpu_time_msec();
//     let delta = match previous_time == 0 {
//         true => {0}
//         false => {current_time - previous_time}
//     };
//
//     previous_time = current_time;
//
//     // Update every running ticking, kill them and execute cb if they are finished
//     // We have to clone the running tickings list to avoid altering it while
//     // in the for loop (so if another ticking is started in a callback or during onUpdate())
//     let binding = RUNNING_TICKINGS.lock().unwrap();
//
//     for mut ticking in binding.clone().into_iter() {
//         let mut ticking_mutex = ticking.lock().unwrap();
//         let run = ticking_mutex.on_update(delta);
//
//         (ticking_mutex.get_tick_callback())();
//         if !run {
//             ticking_mutex.set_stop(true); // will remove the ticking from RUNNING_TICKINGS
//         }
//     }
// }

// Interface representing something that "ticks" every frame for a certain amount of frames,
// like a timer, an animation, a background task...
// The library manages a list of running tickings. Each ticking is reponsible for managing its own
// lifetime by returning true or false in onUpdate.
pub trait Ticking {

    /**
     * Starts the ticking, pushing it to the list of running tickings.
     * If the ticking is finished, it will be restarted.
     * If the ticking is already running, this method will have no effect.
     */
    fn start(&mut self, tick_manager: &mut TickManager) where Self: Send + Sync, Self: Sized {
        if self.is_running() {
            return;
        }

        // tick_manager.running_tickings.lock().unwrap().push(Arc::new(Mutex::new(self)));

        self.set_running(true);

        self.on_start();
    }

    /**
     * Stops the ticking if it was running, and executes the end callback.
     */
    fn stop(&mut self) {
        self.set_stop(false);
    }

    /**
     * Sets a callback to be executed when the
     * ticking finishes.
     * The callback argument will be set to true if the ticking stopped
     * on its own, false if it was stopped early by the user.
     */
    fn set_end_callback(&mut self, end_callback: TickingEndCallback);

    /**
     * Sets a callback to be executed at every tick
     * until the ticking finishes.
     *
     * The last animation tick will execute the tick callback
     * then the end callback.
     */
    fn set_tick_callback(&mut self, tick_callback: TickingTickCallback);

    fn get_end_callback(&mut self) -> TickingEndCallback;

    fn get_tick_callback(&mut self) -> TickingTickCallback;

    /**
     * Returns true if the ticking is currently running.
     */
    fn is_running(&self) -> bool;

    fn set_running(&mut self, running: bool);
    fn set_stop(&mut self, finished: bool) {
        if !self.is_running() {
            return;
        }

        // RUNNING_TICKINGS.lock().unwrap().retain(|mut ticking| {
        //     ticking.deref_mut() != self
        // });

        self.set_running(false);

        (self.get_end_callback())(finished);

        self.on_stop();
    }

    /**
     * Executed every frame while the ticking lives.
     * Delta is the time difference in ms between the last frame
     * and the current one.
     * Must return false if the ticking is finished and should be
     * removed from the list of active tickings.
     * The end callback will automatically be called then.
     */
    fn on_update(&mut self, delta: Timestamp) -> bool;

    /**
     * Called when the ticking becomes active.
     */
    fn on_start(&mut self);

    /**
     * Called when the ticking is stopped, either by the user
     * or because it finished.
     */
    fn on_stop(&self);
}

// Represents a "finite" ticking that runs for a known amount of time
// and can be seek / reset / rewound.
pub trait FiniteTicking: Ticking {
    /**
     * Rewinds the ticking to go back to the beginning
     * without losing its state (as opposed to reset() that clears
     * everything in the ticking).
     * Does not start or stop it.
     */
    fn rewind(&mut self) {
        self.on_rewind();
    }

    /**
     * Stops and resets the ticking, clearing its state
     * in the process (as opposed to rewind() that just restarts
     * the ticking from the beginning without losing the state).
     */
    fn reset(&mut self) {
        self.stop();
        self.on_reset()
    }

    /**
     * Called when the ticking gets rewound.
     */
    fn on_rewind(&mut self);

    /**
     * Called when the ticking gets reset.
     */
    fn on_reset(&mut self);
}
