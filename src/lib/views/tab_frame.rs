use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::base_view::FocusDirection;
use crate::lib::core::frame_context::FrameContext;
use crate::lib::core::view::View;
use crate::lib::views::applet_frame::AppletFrame;

pub struct TabFrame {
    scrolling_frame: AppletFrame
}

impl TabFrame {
    pub(crate) fn create() -> Rc<RefCell<Box<dyn View>>> {
        todo!()
    }
}

impl View for TabFrame {
    fn frame(&self, ctx: &FrameContext) {
        todo!()
    }

    fn get_default_focus(&self) -> Box<dyn View> {
        todo!()
    }

    fn get_next_focus(&self, direction: FocusDirection, current_view: &dyn View) -> Box<dyn View> {
        todo!()
    }

    fn on_focus_lost(&self) {
        todo!()
    }

    fn on_focus_gained(&self) {
        todo!()
    }

    fn describe(&self) -> String {
        todo!()
    }

    fn get_view(&self, id: &str) -> Rc<RefCell<Option<Box<dyn View>>>> {
        todo!()
    }

    fn get_parent(&self) -> Rc<RefCell<Option<Box<dyn View>>>> {
        todo!()
    }
}