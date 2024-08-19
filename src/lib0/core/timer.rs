use glm::ext::e;
use crate::lib::core::time::{empty_end_fn, empty_fn, FiniteTicking, Ticking, TickingEndCallback, TickingGenericCallback, TickingTickCallback, TickManager, Timestamp};

// A Timer allows to run a callback once after a given period of time, in ms
// Add the callback with setEndCallback(), set the duration with setDuration() then start the timer
pub struct Timer {
    running: bool,
    end_callback: TickingEndCallback,
    tick_callback: TickingTickCallback,
    duration: Timestamp,
    progress: Timestamp,
}

impl Ticking for Timer {
    fn set_end_callback(&mut self, end_callback: TickingEndCallback) {
        self.end_callback = end_callback;
    }

    fn set_tick_callback(&mut self, tick_callback: TickingTickCallback) {
        self.tick_callback = tick_callback;
    }

    fn get_end_callback(&mut self) -> TickingEndCallback {
        self.end_callback
    }

    fn get_tick_callback(&mut self) -> TickingTickCallback {
        self.tick_callback
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    fn on_update(&mut self, delta: Timestamp) -> bool {
        self.progress += delta;
        self.progress < self.duration
    }

    fn on_start(&mut self) {
        self.progress = 0;
    }

    fn on_stop(&self) {

    }
}

impl FiniteTicking for Timer {
    fn on_rewind(&mut self) {
        self.progress = 0;
    }

    fn on_reset(&mut self) {
        self.progress = 0;
        self.duration = 0;
    }
}

impl Timer {
    /**
     * Starts the timer directly with a given duration, in ms.
     */
    fn start_with_duration(&mut self, duration: Timestamp, tick_manager: &mut TickManager) {
        self.duration = duration;
        self.start(tick_manager);
    }

    /**
     * Sets the duration of the timer, in ms.
     * Does not stop or reset it.
     */
    fn set_duration(&mut self, duration: Timestamp) {
        self.duration = duration;
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop();
    }
}

// A RepeatingTimer allows to run a callback repeatedly at a given time interval, in ms
// Add the callback with setCallback(), set the period with setPeriod() then start the timer
pub struct RepeatingTimer {
    running: bool,
    end_callback: TickingEndCallback,
    tick_callback: TickingTickCallback,
    callback: TickingGenericCallback,
    period: Timestamp,
    progress: Timestamp,
}

impl Ticking for RepeatingTimer {
    fn set_end_callback(&mut self, end_callback: TickingEndCallback) {
        self.end_callback = end_callback;
    }

    fn set_tick_callback(&mut self, tick_callback: TickingTickCallback) {
        self.tick_callback = tick_callback;
    }

    fn get_end_callback(&mut self) -> TickingEndCallback {
        self.end_callback
    }

    fn get_tick_callback(&mut self) -> TickingTickCallback {
        self.tick_callback
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    fn on_update(&mut self, delta: Timestamp) -> bool {
        self.progress += delta;

        if self.progress >= self.period {
            (self.callback)();
            self.progress = 0;
        }

        return true; // never stop
    }

    fn on_start(&mut self) {
        self.progress = 0;
    }

    fn on_stop(&self) {

    }
}

impl RepeatingTimer {

    pub fn new(period: Timestamp) -> RepeatingTimer {
        RepeatingTimer {
            running: false,
            end_callback: empty_end_fn,
            tick_callback: empty_fn,
            callback: empty_fn,
            period,
            progress: 0,
        }
    }

    /**
     * Starts the timer directly with a given period, in ms.
     */
    fn start_with_period(&mut self, period: Timestamp, tick_manager: &mut TickManager) {
        self.period = period;
        self.start(tick_manager);
    }

    /**
     * Sets the period of the timer, in ms.
     * Does not stop or reset it.
     */
    fn set_period(&mut self, period: Timestamp) {
        self.period = period;
    }

    /**
     * Sets the callback of the timer.
     * Tick callback is still executed every tick.
     * End callback is executed when the timer is stopped.
     */
    fn set_callback(&mut self, callback: TickingGenericCallback) {
        self.callback = callback;
    }
}

impl Drop for RepeatingTimer {
    fn drop(&mut self) {
        self.stop();
    }
}