use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::base_view::FocusDirection;
use crate::lib::core::event::Event;
use crate::lib::core::frame_context::FrameContext;

pub type GenericEvent = Event<Rc<RefCell<Option<Box<dyn View>>>>> ;
pub type VoidEvent = Event<()>;

pub trait View {

    fn frame(&self, ctx: &FrameContext);
    fn get_default_focus(&self) -> Box<dyn View>;
    fn get_next_focus(&self, direction: FocusDirection, current_view: &dyn View) -> Box<dyn View>;
    fn on_focus_lost(&self);

    fn on_focus_gained(&self);

    fn describe(&self) -> String;

    fn get_view(&self, id: &str) -> Rc<RefCell<Option<Box<dyn View>>>>;

    fn get_parent(&self) -> Rc<RefCell<Option<Box<dyn View>>>>;
}