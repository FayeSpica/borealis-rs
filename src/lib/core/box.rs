use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::base_view::{BaseView, FocusDirection};
use crate::lib::core::frame_context::FrameContext;
use crate::lib::core::view::View;

pub enum JustifyContent
{
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum AlignItems
{
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

pub enum Axis
{
    Row,
    Column,
}

pub enum Direction
{
    Inherit,
    LeftToRight,
    RightToLeft,
}

// Generic FlexBox layout
pub struct BoxView {
    view: BaseView,
    axis: Axis,
    children: Vec<Box<BaseView>>
}

impl BoxView {
    pub(crate) fn create() -> Rc<RefCell<Box<dyn View>>> {
        todo!()
    }
}

impl View for BoxView {
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

// An empty view that has auto x auto and grow=1.0 to push
// all the next views in its box to the right (or to the bottom)
pub struct Padding {
    view: BaseView,
}

impl Padding {
    pub(crate) fn create() -> Rc<RefCell<Box<dyn View>>> {
        todo!()
    }
}

impl View for Padding {
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