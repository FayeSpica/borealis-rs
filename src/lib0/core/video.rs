use std::cell::RefCell;
use std::rc::Rc;

// A VideoContext is responsible for providing a nanovg context for the app
// (so by extension it manages all the graphics state as well as the window / context).
// The VideoContext implementation must also provide the nanovg implementation. As such, there
// can only be one VideoContext linked at any time in the binary.
// Context creation and teardown can be done in the constructor and destructor.
pub trait VideoContext {
    /**
     * Called at the beginning of every frame to clear the window
     */
    fn clear(&self, color: nanovg::Color);

    /**
     * Called at the beginning of every frame to begin it.
     */
    fn begin_frame(&self);

    /**
     * Called at the end of every frame to end it (swap buffers...).
     */
    fn end_frame(&self);

    /**
     * Can be called by the application to reset the graphics
     * state, in case there is a need to use the graphics API
     * directly (for instance direct OpenGL calls).
     */
    fn reset_state(&self);

    fn get_nvg_context(&mut self) -> Rc<RefCell<nanovg::Context>>;
}