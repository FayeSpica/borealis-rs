use crate::lib::core::time::{empty_end_fn, empty_fn, Ticking, Timestamp};
use crate::lib::core::timer::RepeatingTimer;

// A RepeatingTask is a task executed repeatedly on the main thread at a given period in ms
pub struct RepeatingTask {
    repeat_timer: RepeatingTimer
}

impl RepeatingTask {

    /**
     * Create a RepeatingTask with the given period in ms.
     *
     * The task is guaranteed to wait for at least the given period of time between
     * handler runs, but it can wait for slightly longer (usually less than a ms late)
     */
    pub fn new(period: Timestamp) -> Self {
        let mut repeat_timer = RepeatingTimer::new(period);

        let mut repeating_task = RepeatingTask {
            repeat_timer,
        };

        repeating_task.repeat_timer.set_end_callback(empty_end_fn);

        repeating_task
    }

    /**
     * Task handler executed repeatedly on the main thread at the given period.
     */
    pub fn run(&self) {

    }

    /**
     * Starts the task.
     */
    pub fn start(&mut self) {
        // self.repeat_timer.start();
    }

    /**
     * Stops the task.
     */
    pub fn stop(&mut self) {
        self.repeat_timer.stop();
    }
}