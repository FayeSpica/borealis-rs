use std::cell::RefCell;
use std::rc::Rc;
use nanovg::Context as NVGcontext;
use crate::lib::core::font::FontStash;
use crate::lib::core::theme::Theme;
pub struct FrameContext<'a> {
    pub vg: Rc<RefCell<NVGcontext>>,
    pub pixel_ratio: f32,
    pub font_stash: &'a FontStash,
    pub theme: &'a Theme,
}

impl FrameContext<'_> {
    pub fn new<'a>(vg: Rc<RefCell<NVGcontext>>, pixel_ratio: f32, font_stash: &'a FontStash, theme: &'a Theme) -> FrameContext<'a> {
        FrameContext {
            vg,
            pixel_ratio,
            font_stash,
            theme,
        }
    }
}