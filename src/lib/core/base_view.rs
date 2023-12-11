use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::mem::swap;
use std::rc::Rc;
use glfw::Action;
use crate::lib::core::animation::{Animatable, get_highlight_animation};
use nanovg::Context as NVGcontext;
use nanovg::Solidity::Hole;
use nanovg_sys::{nvgRGBA, nvgBeginPath, nvgFill, nvgFillColor, nvgRect, nvgPathWinding, nvgRoundedRect, nvgBoxGradient, nvgFillPaint, nvgLinearGradient, nvgRadialGradient, nvgStrokeColor, nvgStrokeWidth, nvgStroke, nvgStrokePaint, nvgRestore, nvgMoveTo, nvgLineTo, nvgRGB};
use ordered_float::OrderedFloat;
use rand::Rng;
use rust_i18n::t;
use crate::lib::core::frame_context::FrameContext;
use crate::lib::core::style::{Style, STYLE};
use crate::lib::core::theme::Theme;
use crate::lib::core::time::{get_cpu_time_msec, Timestamp};
use yoga::{Align, Edge, Node as YGNode, Node, style, StyleUnit};
use yoga::prelude::*;
use yoga::StyleUnit::{Auto, Percent, UndefinedValue};
use crate::lib::core::{audio, util};
use crate::lib::core::actions::{ActionIdentifier, ActionListener};
use crate::lib::core::audio::Sound;
use crate::lib::core::input::ControllerButton;
use crate::lib::core::view::GenericEvent;

pub static TRANSPARENT: nanovg::Color = nanovg::Color::from_rgba(0, 0, 0, 0);

// Focus direction when navigating
#[derive(Clone, Copy)]
pub enum FocusDirection
{
    Up,
    Down,
    Left,
    Right
}

// View background
pub enum ViewBackground
{
    None,
    Sidebar,
    Backdrop,
    ShapeColor,
}

pub enum AlignSelf
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

// View visibility
pub enum Visibility
{
    Visible, // the view is visible
    Invisible, // the view is invisible but still takes some space
    Gone, // the view is invisible and doesn't take any space
}

// Position attribute behavior
pub enum PositionType
{
    Relative, // position attributes offset the view from the base layout
    Absolute, // position attributes set the absolute coordinates of the view
}

// The animation to play when
// pushing / popping an activity or
// showing / hiding a view.
#[derive(Clone, Copy, PartialEq)]
pub enum TransitionAnimation
{
    Fade, // the old activity fades away and the new one fades in
    SlideLeft, // the old activity slides out to the left and the new one slides in from the right
    SlideRight, // inverted SLIDE_LEFT
}

// A View shape's shadow type
pub enum ShadowType
{
    None, // do not draw any shadow around the shape
    Generic, // generic all-purpose shadow
    Custom, // customized shadow (use the provided methods to tweak it)
}

pub type AutoAttributeHandler = fn();
pub type IntAttributeHandler = fn(i32);
pub type FloatAttributeHandler = fn(f64);
pub type StringAttributeHandler = fn(String);
pub type ColorAttributeHandler = fn(nanovg::Color);
pub type BoolAttributeHandler = fn(bool);
pub type FilePathAttributeHandler = fn(String);

const AUTO: f32 = f32::NAN;

// Superclass for all the other views
// Lifecycle of a view is :
//   new -> [willAppear -> willDisappear] -> delete
//
// Users have do to the new, the rest of the lifecycle is taken
// care of by the library
//
// willAppear and willDisappear can be called zero or multiple times
// before deletion (in case of a TabLayout for instance)
pub struct BaseView {
    background: ViewBackground,
    highlight_alpha: Animatable,
    highlight_padding: f32,
    highlight_corner_radius: f32,
    click_alpha: Animatable, // animated between 0 and 1
    highlight_shaking: bool,
    highlight_shake_start: Timestamp,
    highlight_shake_direction: FocusDirection,
    highlight_shake_amplitude: f32,
    fade_in: bool, // is the fade in animation running?
    in_fade_animation: bool, // is any fade animation running?
    theme_override: Option<Theme>,
    hidden: bool,
    focusable: bool,
    focus_sound: audio::Sound,
    hide_highlight_background: bool,
    detached: bool,
    detached_origin_x: f32,
    detached_origin_y: f32,
    translation_x: f32,
    translation_y: f32,
    wireframe_enabled: bool,

    actions: Vec<crate::lib::core::actions::Action>,

    /**
     * Parent user data, typically the index of the view
     * in the internal layout structure
     */
    parent_userdata: bool,

    culled: bool, // will be culled by the parent Box, if any

    bound_documents: Vec<String>,

    auto_attributes: HashMap<String, AutoAttributeHandler>,
    percentage_attributes: HashMap<String, FloatAttributeHandler>,
    float_attributes: HashMap<String, FloatAttributeHandler>,
    string_attributes: HashMap<String, StringAttributeHandler>,
    color_attributes: HashMap<String, ColorAttributeHandler>,
    bool_attributes: HashMap<String, BoolAttributeHandler>,
    file_path_attributes: HashMap<String, FilePathAttributeHandler>,

    known_attributes: HashSet<String>,

    maximum_allowed_xml_elements: u32,

    line_color: nanovg::Color,
    line_top: f32,
    line_right: f32,
    line_bottom: f32,
    line_left: f32,

    visibility: Visibility,

    background_color: nanovg::Color,
    border_color: nanovg::Color,
    border_thickness: f32,
    corner_radius: f32,
    shadow_type: ShadowType,
    show_shadow: bool,

    custom_focus_by_id: HashMap<FocusDirection, String>,
    custom_focus_by_ptr: HashMap<FocusDirection, Rc<RefCell<BaseView>>>,

    collapse_state: Animatable,

    focused: bool,

    focus_event: GenericEvent,

    yg_node: YGNode,

    id: String,

    alpha: Animatable,
}

impl BaseView {

    pub fn new() -> Self {
        let mut s = Self {
            background: ViewBackground::None,
            highlight_alpha: Animatable::new(0.0),
            highlight_padding: 0.0,
            highlight_corner_radius: STYLE.get_metric("brls/highlight/corner_radius").unwrap().clone(),
            click_alpha: Animatable::new(0.0),
            highlight_shaking: false,
            highlight_shake_start: 0,
            highlight_shake_direction: FocusDirection::Up,
            highlight_shake_amplitude: 0.0,
            fade_in: false,
            in_fade_animation: false,
            theme_override: None,
            hidden: false,
            focusable: false,
            focus_sound: audio::Sound::SoundNone,
            hide_highlight_background: false,
            detached: false,
            detached_origin_x: 0.0,
            detached_origin_y: 0.0,
            translation_x: 0.0,
            translation_y: 0.0,
            wireframe_enabled: false,
            actions: vec![],
            parent_userdata: false,
            culled: false,
            bound_documents: vec![],
            auto_attributes: Default::default(),
            percentage_attributes: Default::default(),
            float_attributes: Default::default(),
            string_attributes: Default::default(),
            color_attributes: Default::default(),
            bool_attributes: Default::default(),
            file_path_attributes: Default::default(),
            known_attributes: Default::default(),
            maximum_allowed_xml_elements: 0,
            line_color: TRANSPARENT,
            line_top: 0.0,
            line_right: 0.0,
            line_bottom: 0.0,
            line_left: 0.0,
            visibility: Visibility::Visible,
            background_color: TRANSPARENT,
            border_color: TRANSPARENT,
            border_thickness: 0.0,
            corner_radius: 0.0,
            shadow_type: ShadowType::None,
            show_shadow: false,
            custom_focus_by_id: Default::default(),
            custom_focus_by_ptr: Default::default(),
            collapse_state: Animatable::new(1.0),
            focused: false,
            focus_event: GenericEvent::new(),
            yg_node: yoga::Node::new(),
            id: "".to_string(),
            alpha: Animatable::new(1.0),
        };

        // style!(s.yg_node, Width(Auto), Height(Auto));
        // s.register_common_attributes();
        s
    }

    // fn draw_background(&mut self, vg: &mut nanovg::Context, ctx: &mut FrameContext, style: Style) {
    //     let x = self.get_x();
    //     let y = self.get_y();
    //     let width = self.get_width();
    //     let height = self.get_height();
    //
    //     let theme = ctx.theme;
    //
    //     match self.background {
    //         ViewBackground::None => {}
    //         ViewBackground::Sidebar => {
    //             let backdrop_height = *style.get_metric("brls/sidebar/border_height").unwrap();
    //             let sidebar_color = *theme.get_color("brls/sidebar/background").unwrap();
    //
    //             unsafe {
    //                 // Solid color
    //                 nvgBeginPath(vg.into());
    //                 nvgFillColor(vg.into(), self.a_color(sidebar_color).into());
    //                 nvgRect(vg.into(), x, y + backdrop_height, width, height - backdrop_height * 2);
    //                 nvgFill(vg.into());
    //
    //                 //Borders gradient
    //                 // Top
    //                 let top_gradient = nvgLinearGradient(vg.into(), x, y + backdrop_height, x, y, self.a_color(sidebar_color).into(), TRANSPARENT.into());
    //                 nvgBeginPath(vg.into());
    //                 nvgFillPaint(vg.into(), top_gradient);
    //                 nvgRect(vg.into(), x, y, width, backdrop_height);
    //                 nvgFill(vg.into());
    //
    //                 // Bottom
    //                 let bottom_gradient = nvgLinearGradient(vg.into(), x, y + height - backdrop_height, x, y + height, self.a_color(sidebar_color).into(), TRANSPARENT.into());
    //                 nvgBeginPath(vg.into());
    //                 nvgFillPaint(vg.into(), bottom_gradient);
    //                 nvgRect(vg.into(), x, y + height - backdrop_height, width, backdrop_height);
    //                 nvgFill(vg.into());
    //             }
    //         }
    //         ViewBackground::Backdrop => {
    //             unsafe {
    //                 nvgFillColor(vg.into(), self.a_color(*theme.get_color("brls/backdrop").unwrap()).into());
    //                 nvgBeginPath(vg.into());
    //                 nvgRect(vg.into(), x, y, width, height);
    //                 nvgFill(vg.into());
    //             }
    //         }
    //         ViewBackground::ShapeColor => {
    //             unsafe {
    //                 nvgFillColor(vg.into(), self.a_color(self.background_color).into());
    //                 nvgBeginPath(vg.into());
    //
    //                 if self.corner_radius > 0.0 {
    //                     nvgRoundedRect(vg.into(), x, y, width, height, self.corner_radius);
    //                 } else {
    //                     nvgRect(vg.into(), x, y, width, height);
    //                 }
    //
    //                 nvgFill(vg.into());
    //             }
    //         }
    //     }
    // }
    //
    // fn draw_shadow(&mut self, vg: &mut NVGcontext, ctx: &mut FrameContext, style: Style, x: f32, y: f32, width: f32, height: f32) {
    //     let mut shadow_width = 0.0f32;
    //     let mut shadow_feather = 0.0f32;
    //     let mut shadow_opacity = 0.0f32;
    //     let mut shadow_offset = 0.0f32;
    //
    //     match self.shadow_type {
    //         ShadowType::None => {}
    //         ShadowType::Generic => {
    //             shadow_width = *style.get_metric("brls/shadow/width").unwrap();
    //             shadow_feather = *style.get_metric("brls/shadow/feather").unwrap();
    //             shadow_opacity = *style.get_metric("brls/shadow/opacity").unwrap();
    //             shadow_offset = *style.get_metric("brls/shadow/offset").unwrap();
    //         }
    //         ShadowType::Custom => {}
    //     }
    //
    //     unsafe {
    //         let shadow_paint = nvgBoxGradient(
    //             vg.into(),
    //             x, y + shadow_width,
    //             width, height,
    //             self.corner_radius * 2, shadow_feather,
    //             nanovg::Color::from_rgba(0, 0, 0, (shadow_opacity * self.alpha.get_value()) as u8).into(), TRANSPARENT.into(),
    //         );
    //
    //         nvgBeginPath(vg.into());
    //         nvgRect(
    //             vg.into(),
    //             x - shadow_offset,
    //             y - shadow_offset,
    //             width + shadow_offset * 2,
    //             height + shadow_offset * 3,
    //         );
    //         nvgRoundedRect(vg.into(), x, y, width, height, self.corner_radius);
    //         nanovg_sys::nvgPathWinding(vg.into(), Hole.into());
    //         nvgFillPaint(vg.into(), shadow_paint);
    //         nvgFill(vg.into());
    //     }
    // }
    //
    // fn draw_border(&mut self, vg: &mut NVGcontext, ctx: &mut FrameContext, style: Style, x: f32, y: f32, width: f32, height: f32) {
    //     unsafe {
    //         nvgBeginPath(vg.into());
    //         nvgStrokeColor(vg.into(), self.border_color.into());
    //         nvgStrokeWidth(vg.into(), self.border_thickness.into());
    //         nvgRoundedRect(vg.into(), x, y, width, height, self.corner_radius);
    //         nvgStroke(vg.into());
    //     }
    // }
    //
    // fn draw_highlight(&mut self, vg: &mut NVGcontext, theme: Theme, alpha: f32, style: Style, background: bool) {
    //     unsafe {
    //         nanovg_sys::nvgSave(vg.into());
    //         nanovg_sys::nvgResetScissor(vg.into());
    //     }
    //
    //     let padding = self.highlight_padding;
    //     let corner_radius = self.highlight_corner_radius;
    //     let stroke_width = *style.get_metric("brls/highlight/stroke_width").unwrap();
    //
    //     let x = self.get_x() - padding - stroke_width / 2;
    //     let y = self.get_y() - padding - stroke_width / 2;
    //     let width = self.get_width() + padding * 2 + stroke_width;
    //     let height = self.get_height() + padding * 2 + stroke_width;
    //
    //     // Shake animation
    //     if self.highlight_shaking {
    //         let cur_time = get_cpu_time_msec();
    //         let t = ((cur_time - self.highlight_shake_start) / 10) as f32;
    //
    //         if t >= *style.get_metric("brls/animations/highlight_shake").unwrap() {
    //             self.highlight_shaking = false;
    //         } else {
    //             match self.highlight_shake_direction {
    //                 FocusDirection::Up => {
    //                     y -= util::shake_animation(t, self.highlight_shake_amplitude);
    //                 }
    //                 FocusDirection::Down => {
    //                     y += util::shake_animation(t, self.highlight_shake_amplitude);
    //                 }
    //                 FocusDirection::Left => {
    //                     x -= util::shake_animation(t, self.highlight_shake_amplitude);
    //                 }
    //                 FocusDirection::Right => {
    //                     x += util::shake_animation(t, self.highlight_shake_amplitude);
    //                 }
    //             }
    //         }
    //     }
    //
    //     // Draw
    //     if background {
    //         // Background
    //         let highlight_background_color = *theme.get_color("brls/highlight/background").unwrap();
    //         unsafe {
    //             nvgFillColor(
    //                 vg.into(),
    //                 self.from_rgbaf(
    //                     highlight_background_color.red(),
    //                     highlight_background_color.green(),
    //                     highlight_background_color.blue(),
    //                     self.highlight_alpha.get_value(),
    //                 ).into()
    //             );
    //             nvgBeginPath(vg.into());
    //             nanovg_sys::nvgRoundedRect(vg.into(), x, y, width, height, corner_radius);
    //             nvgFill(vg.into());
    //         }
    //     } else {
    //         let shadow_offset = *style.get_metric("brls/highlight/shadow_offset").unwrap();
    //
    //         unsafe {
    //             // Shadow
    //             let shadow_paint = nvgBoxGradient(
    //                 vg.into(),
    //                 x,
    //                 y + *style.get_metric("brls/highlight/shadow_width").unwrap(),
    //                 width,
    //                 height,
    //                 corner_radius * 2.0,
    //                 shadow_offset,
    //                 nvgRGBA(0, 0, 0, (*style.get_metric("brls/highlight/shadow_opacity").unwrap() * alpha) as u8),
    //                 TRANSPARENT.into(),
    //             );
    //             nvgBeginPath(vg.into());
    //             nvgRect(vg.into(), x - shadow_offset, y - shadow_offset, width + shadow_offset * 2.0, height + shadow_offset * 3.0);
    //             nvgRoundedRect(vg.into(), x, y, width, height, corner_radius);
    //             nvgPathWinding(vg.into(), Hole.into());
    //             nvgFillPaint(vg.into(), shadow_paint);
    //             nvgFill(vg.into());
    //         }
    //
    //         // Border
    //         let (gradient_x, gradient_y, color) = get_highlight_animation();
    //         let highlightColor1 = *theme.get_color("brls/highlight/color1").unwrap();
    //
    //         let pulsationColor = self.from_rgbaf((color * highlightColor1.red()) + (1 - color) * highlightColor1.red(),
    //                                    (color * highlightColor1.green()) + (1 - color) * highlightColor1.green(),
    //                                    (color * highlightColor1.blue()) + (1 - color) * highlightColor1.blue(),
    //                                    alpha);
    //
    //         let mut borderColor = *theme.get_color("brls/highlight/color2").unwrap();
    //         borderColor.set_alpha(0.5 * alpha * self.get_alpha());
    //
    //         let stroke_width = *style.get_metric("brls/highlight/stroke_width").unwrap();
    //
    //         unsafe {
    //             let border1Paint = nvgRadialGradient(vg.into(),
    //                                                  x + gradient_x * width, y + gradient_y * height,
    //                                                  stroke_width * 10, stroke_width * 40,
    //                                                  borderColor.into(), TRANSPARENT.into());
    //
    //             let border2Paint = nvgRadialGradient(vg.into(),
    //                                                  x + (1 - gradient_x) * width, y + (1 - gradient_y) * height,
    //                                                  stroke_width * 10, stroke_width * 40,
    //                                                  borderColor.into(), TRANSPARENT.into());
    //
    //             nvgBeginPath(vg.into());
    //             nvgStrokeColor(vg.into(), pulsationColor.into());
    //             nvgStrokeWidth(vg.into(), stroke_width);
    //             nvgRoundedRect(vg.into(), x, y, width, height, corner_radius);
    //             nvgStroke(vg.into());
    //
    //             nvgBeginPath(vg.into());
    //             nvgStrokePaint(vg.into(), border1Paint);
    //             nvgStrokeWidth(vg.into(), stroke_width);
    //             nvgRoundedRect(vg.into(), x, y, width, height, corner_radius);
    //             nvgStroke(vg.into());
    //
    //             nvgBeginPath(vg.into());
    //             nvgStrokePaint(vg.into(), border2Paint);
    //             nvgStrokeWidth(vg.into(), stroke_width);
    //             nvgRoundedRect(vg.into(), x, y, width, height, corner_radius);
    //             nvgStroke(vg.into());
    //         }
    //     }
    //
    //     unsafe {
    //         nvgRestore(vg.into());
    //     }
    // }
    //
    // fn draw_click_animation(&mut self, vg: &mut NVGcontext, ctx: &mut FrameContext, x: f32, y: f32, width: f32, height: f32) {
    //     let theme = ctx.theme;
    //     let mut color = *theme.get_color("brls/click_pulse").unwrap();
    //
    //     color.set_alpha(color.alpha() * self.click_alpha.get_value());
    //
    //     unsafe {
    //         nvgFillColor(vg.into(), self.a_color(color).into());
    //         nvgBeginPath(vg.into());
    //
    //         if self.corner_radius > 0.0 {
    //             nvgRoundedRect(vg.into(), x, y, width, height, self.corner_radius);
    //         } else {
    //             nvgRect(vg.into(), x, y, width, height);
    //         }
    //
    //         nvgFill(vg.into());
    //     }
    // }
    //
    // fn draw_wireframe(&mut self, ctx: &mut FrameContext, x: f32, y: f32, width: f32, height: f32) {
    //     unsafe {
    //         nvgStrokeWidth(ctx.vg.into(), 1.0);
    //
    //         // Outline
    //         nvgBeginPath(ctx.vg.into());
    //         nvgStrokeColor(ctx.vg.into(), nvgRGB(0, 0, 255));
    //         nvgRect(ctx.vg.into(), x, y, width, height);
    //         nvgStroke(ctx.vg.into());
    //
    //         if self.has_parent()
    //         {
    //             // Diagonals
    //             nvgFillColor(ctx.vg.into(), nvgRGB(0, 0, 255));
    //
    //             nvgBeginPath(ctx.vg.into());
    //             nvgMoveTo(ctx.vg.into(), x, y);
    //             nvgLineTo(ctx.vg.into(), x + width, y + height);
    //             nvgFill(ctx.vg.into());
    //
    //             nvgBeginPath(ctx.vg.into());
    //             nvgMoveTo(ctx.vg.into(), x + width, y);
    //             nvgLineTo(ctx.vg.into(), x, y + height);
    //             nvgFill(ctx.vg.into());
    //         }
    //
    //         // Padding
    //         nvgBeginPath(ctx.vg.into());
    //         nvgStrokeColor(ctx.vg.into(), nvgRGB(0, 255, 0));
    //
    //         // Retrieve padding values
    //         let padding_top = self.yg_node.get_layout_padding_top();
    //         let padding_left = self.yg_node.get_layout_padding_left();
    //         let padding_bottom = self.yg_node.get_layout_padding_bottom();
    //         let padding_right = self.yg_node.get_layout_padding_right();
    //
    //         // Top
    //         if padding_top > 0.0 {
    //             nvgRect(ctx.vg.into(), x, y, width, padding_top);
    //         }
    //
    //         // Left
    //         if padding_left > 0.0 {
    //             nvgRect(ctx.vg.into(), x, y, padding_left, height);
    //         }
    //
    //         // Bottom
    //         if padding_bottom > 0.0 {
    //             nvgRect(ctx.vg.into(), x, y + height - padding_bottom, width, padding_bottom);
    //         }
    //
    //         // Right
    //         if padding_right > 0.0 {
    //             nvgRect(ctx.vg.into(), x + width - padding_right, y, padding_right, height);
    //         }
    //
    //         nvgStroke(ctx.vg.into());
    //
    //         // Margins
    //         nvgBeginPath(ctx.vg.into());
    //         nvgStrokeColor(ctx.vg.into(), nvgRGB(255, 0, 0));
    //
    //         // Retrieve margin values
    //         let margin_top = self.yg_node.get_layout_margin_top();
    //         let margin_left = self.yg_node.get_layout_margin_left();
    //         let margin_bottom = self.yg_node.get_layout_margin_bottom();
    //         let margin_right = self.yg_node.get_layout_margin_right();
    //
    //         // Top
    //         if margin_top > 0.0 {
    //             nvgRect(ctx.vg.into(), x - margin_left, y - margin_top, width + margin_left + margin_right, margin_top);
    //         }
    //
    //         // Left
    //         if margin_left > 0.0 {
    //             nvgRect(ctx.vg.into(), x - margin_left, y - margin_top, margin_left, height + margin_top + margin_bottom);
    //         }
    //
    //         // Bottom
    //         if margin_bottom > 0.0 {
    //             nvgRect(ctx.vg.into(), x - margin_left, y + height, width + margin_left + margin_right, margin_bottom);
    //         }
    //
    //         // Right
    //         if margin_right > 0.0 {
    //             nvgRect(ctx.vg.into(), x + width, y - margin_top, margin_right, height + margin_top + margin_bottom);
    //         }
    //
    //         nvgStroke(ctx.vg.into());
    //     }
    // }
    //
    // fn draw_line(&mut self, ctx: &mut FrameContext, x: f32, y: f32, width: f32, height: f32) {
    //     // Don't setup and draw empty nvg path if there is no line to draw
    //     if self.line_top <= 0.0 && self.line_right <= 0.0 && self.line_bottom <= 0.0 && self.line_left <= 0.0 {
    //         return;
    //     }
    //
    //     unsafe {
    //         nvgBeginPath(ctx.vg.into());
    //         nvgFillColor(ctx.vg.into(), self.a_color(self.line_color).into());
    //
    //         if self.line_top > 0.0 {
    //             nvgRect(ctx.vg.into(), x, y, width, self.line_top);
    //         }
    //
    //         if self.line_right > 0.0 {
    //             nvgRect(ctx.vg.into(), x + width, y, self.line_right, height);
    //         }
    //
    //         if self.line_bottom > 0.0 {
    //             nvgRect(ctx.vg.into(), x, y + height - self.line_bottom, width, self.line_bottom);
    //         }
    //
    //         if self.line_left > 0.0 {
    //             nvgRect(ctx.vg.into(), x - self.line_left, y, self.line_left, height);
    //         }
    //
    //         nvgFill(ctx.vg.into());
    //     }
    // }
    //
    // fn register_common_attributes(&mut self) {
    //     // todo
    // }
    //
    // fn print_xml_attribute_error_message(&mut self, name: &str, value: &str) {
    //     // todo
    // }
    //
    // fn a_color(&self, color: nanovg::Color) -> nanovg::Color {
    //     color
    // }
    //
    // fn from_rgb(&self, r: u8, g: u8, b: u8) -> nanovg::Color {
    //     self.a_color(nanovg::Color::from_rgb(r, g, b))
    // }
    //
    // fn from_rgba(&self, r: u8, g: u8, b: u8, a: u8) -> nanovg::Color {
    //     self.a_color(nanovg::Color::from_rgba(r, g, b, a))
    // }
    //
    // fn from_rgbf(&self, r: f32, g: f32, b: f32) -> nanovg::Color {
    //     self.a_color(nanovg::Color::from_rgb(r as u8, g as u8, b as u8))
    // }
    //
    // fn from_rgbaf(&self, r: f32, g: f32, b: f32, a: f32) -> nanovg::Color {
    //     self.a_color(nanovg::Color::from_rgba(r as u8, g as u8, b as u8, a as u8))
    // }
    //
    // /**
    //  * Should the hint alpha be animated when
    //  * pushing the view?
    //  */
    // fn animate_hint() -> bool {
    //     false
    // }
    //
    // fn set_background(&mut self, background: ViewBackground) {
    //     self.background = background;
    // }
    //
    // pub(crate) fn shake_highlight(&mut self, direction: FocusDirection) {
    //     self.highlight_shaking = true;
    //     self.highlight_shake_start = get_cpu_time_msec();
    //     self.highlight_shake_direction = direction;
    //     self.highlight_shake_amplitude = rand::thread_rng().gen_range(10..25) as f32;
    // }
    //
    // fn get_x(&mut self) -> f32 {
    //     return if self.detached {
    //         self.detached_origin_x + self.translation_x
    //     } else if self.has_parent() {
    //         self.get_parent().unwrap().get_x() + self.yg_node.get_layout_left() + self.translation_x
    //     } else {
    //         self.yg_node.get_layout_left() + self.translation_x
    //     }
    // }
    //
    // fn get_y(&mut self) -> f32 {
    //     return if self.detached {
    //         self.detached_origin_y + self.translation_y
    //     } else if self.has_parent() {
    //         self.get_parent().unwrap().get_y() + self.yg_node.get_layout_top() + self.translation_y
    //     } else {
    //         self.yg_node.get_layout_top() + self.translation_y
    //     }
    // }
    //
    // fn get_width(&self) -> f32 {
    //     self.yg_node.get_layout_width()
    // }
    //
    // fn get_height(&self) -> f32 {
    //     self.yg_node.get_layout_height()
    // }
    //
    // fn get_height_include_collapse(&self) -> f32 {
    //     self.get_height() * self.collapse_state.get_value()
    // }
    //
    // /**
    //  * Triggers a layout of the whole view tree. Must be called
    //  * after a yoga node property is changed.
    //  *
    //  * Only methods that change yoga nodes properties should
    //  * call this method.
    //  */
    // fn invalidate(&mut self) {
    //     if self.yg_node.get_measure().is_some() {
    //         self.yg_node.mark_dirty();
    //     }
    //
    //     if let Some(parent) = self.get_parent() {
    //         if !self.detached() {
    //             parent.invalidate();
    //         }
    //     } else {
    //         self.yg_node.calculate_layout(yoga::Undefined, yoga::Undefined, yoga::Direction::LTR);
    //     }
    // }
    //
    // /**
    //  * Called when a layout pass ends on that view.
    //  */
    // fn on_layout(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Returns the view with the corresponding id in the view or its children,
    //  * or nullptr if it hasn't been found.
    //  *
    //  * Research is done recursively by traversing the tree starting from this view.
    //  * This view's parents are not traversed.
    //  */
    // pub(crate) fn get_view(&self, id: &str) -> Option<&Self> {
    //     if self.id == id {
    //         Some(self)
    //     } else {
    //         None
    //     }
    // }
    //
    // // -----------------------------------------------------------
    // // Flex layout properties
    // // -----------------------------------------------------------
    //
    // /**
    //  * Sets the preferred width of the view. Use brls::View::AUTO
    //  * to have the layout automatically resize the view.
    //  *
    //  * If set to anything else than AUTO, the view is guaranteed
    //  * to never shrink below the given width.
    //  */
    // fn set_width(&mut self, width: f32) {
    //     self.yg_node.set_min_width(StyleUnit::Percent(OrderedFloat::from(0)));
    //
    //     if width == AUTO {
    //         self.yg_node.set_width(StyleUnit::Auto);
    //         self.yg_node.set_min_width(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_width(StyleUnit::Point(OrderedFloat::from(width)));
    //         self.yg_node.set_min_width(StyleUnit::Point(OrderedFloat::from(width)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the preferred height of the view. Use brls::View::AUTO
    //  * to have the layout automatically resize the view.
    //  *
    //  * If set to anything else than AUTO, the view is guaranteed
    //  * to never shrink below the given height.
    //  */
    // fn set_height(&mut self, height: f32) {
    //     self.yg_node.set_min_height(StyleUnit::Percent(OrderedFloat::from(0)));
    //
    //     if height == AUTO {
    //         self.yg_node.set_height(StyleUnit::Auto);
    //         self.yg_node.set_min_height(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_height(StyleUnit::Point(OrderedFloat::from(height)));
    //         self.yg_node.set_min_height(StyleUnit::Point(OrderedFloat::from(height)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Shortcut to setWidth + setHeight.
    //  *
    //  * Only does one layout pass instead of two when using the two methods separately.
    //  */
    // pub fn set_dimensions(&mut self, width: f32, height: f32) {
    //     self.yg_node.set_min_width(StyleUnit::Point(OrderedFloat::from(width)));
    //     self.yg_node.set_min_height(StyleUnit::Point(OrderedFloat::from(height)));
    //
    //     if width == AUTO {
    //         self.yg_node.set_width(StyleUnit::Auto);
    //         self.yg_node.set_min_width(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_width(StyleUnit::Point(OrderedFloat::from(width)));
    //         self.yg_node.set_min_width(StyleUnit::Point(OrderedFloat::from(width)));
    //     }
    //
    //     if height == AUTO {
    //         self.yg_node.set_height(StyleUnit::Auto);
    //         self.yg_node.set_min_height(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_height(StyleUnit::Point(OrderedFloat::from(height)));
    //         self.yg_node.set_min_height(StyleUnit::Point(OrderedFloat::from(height)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the preferred width of the view in percentage of
    //  * the parent view width. Between 0.0f and 100.0f.
    //  */
    // fn set_width_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_width(StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.yg_node.set_min_width(StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the preferred height of the view in percentage of
    //  * the parent view height. Between 0.0f and 100.0f.
    //  */
    // fn set_height_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_height(StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.yg_node.set_min_height(StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the maximum width of the view, in pixels.
    //  *
    //  * This constraint is stronger than the grow factor: the view
    //  * is guaranteed to never be larger than the given max width.
    //  *
    //  * Use View::AUTO to disable the max width constraint.
    //  */
    // fn set_max_width(&mut self, max_width: f32) {
    //     if max_width == AUTO {
    //         self.yg_node.set_max_width(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_max_width(StyleUnit::Point(OrderedFloat::from(max_width)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the maximum height of the view, in pixels.
    //  *
    //  * This constraint is stronger than the grow factor: the view
    //  * is guaranteed to never be larger than the given max height.
    //  *
    //  * Use View::AUTO to disable the max height constraint.
    //  */
    // fn set_max_height(&mut self, max_height: f32) {
    //     if max_height == AUTO {
    //         self.yg_node.set_max_height(StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_max_height(StyleUnit::Point(OrderedFloat::from(max_height)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the maximum width of the view, in parent width percentage.
    //  *
    //  * This constraint is stronger than the grow factor: the view
    //  * is guaranteed to never be larger than the given max width.
    //  *
    //  * Use View::AUTO to disable the max width constraint.
    //  */
    // fn set_max_width_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_max_width(StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the maximum height of the view, in pixels.
    //  *
    //  * This constraint is stronger than the grow factor: the view
    //  * is guaranteed to never be larger than the given max height.
    //  *
    //  * Use View::AUTO to disable the max height constraint.
    //  */
    // fn set_max_height_percentage(&mut self, max_height: f32) {
    //     self.yg_node.set_max_height(StyleUnit::Percent(OrderedFloat::from(max_height)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the grow factor of the view, aka the percentage
    //  * of remaining space to give this view, in the containing box axis.
    //  * Opposite of shrink.
    //  * Default is 0.0f;
    //  */
    // fn set_grow(&mut self, grow: f32) {
    //     self.yg_node.set_flex_grow(grow);
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the shrink factor of the view, aka the percentage of space
    //  * the view is allowed to shrink for if there is not enough space for everyone
    //  * in the contaning box axis. Opposite of grow.
    //  * Default is 1.0f;
    //  */
    // fn set_shrink(&mut self, shrink: f32) {
    //     self.yg_node.set_flex_shrink(shrink);
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the margin of the view, aka the space that separates
    //  * this view and the surrounding ones in all 4 directions.
    //  *
    //  * Use brls::View::AUTO to have the layout automatically select the
    //  * margin.
    //  *
    //  * Only works with views that have parents - top level views that are pushed
    //  * on the stack don't have parents.
    //  *
    //  * Only does one layout pass instead of four when using the four methods separately.
    //  */
    // fn set_margins(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
    //     if top == AUTO {
    //         self.yg_node.set_margin(Edge::Top, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Top, StyleUnit::Point(OrderedFloat::from(top)));
    //     }
    //
    //     if left == AUTO {
    //         self.yg_node.set_margin(Edge::Left, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Left, StyleUnit::Point(OrderedFloat::from(left)));
    //     }
    //
    //     if bottom == AUTO {
    //         self.yg_node.set_margin(Edge::Bottom, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Bottom, StyleUnit::Point(OrderedFloat::from(bottom)));
    //     }
    //
    //     if right == AUTO {
    //         self.yg_node.set_margin(Edge::Right, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Right, StyleUnit::Point(OrderedFloat::from(right)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the top margin of the view, aka the space that separates
    //  * this view and the surrounding ones.
    //  *
    //  * Only works with views that have parents - top level views that are pushed
    //  * on the stack don't have parents.
    //  *
    //  * Use brls::View::AUTO to have the layout automatically select the
    //  * margin.
    //  */
    // fn set_margin_top(&mut self, top: f32) {
    //     if top == AUTO {
    //         self.yg_node.set_margin(Edge::Top, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Top, StyleUnit::Point(OrderedFloat::from(top)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the right margin of the view, aka the space that separates
    //  * this view and the surrounding ones.
    //  *
    //  * Only works with views that have parents - top level views that are pushed
    //  * on the stack don't have parents.
    //  *
    //  * Use brls::View::AUTO to have the layout automatically select the
    //  * margin.
    //  */
    // fn set_margin_right(&mut self, right: f32) {
    //     if right == AUTO {
    //         self.yg_node.set_margin(Edge::Right, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Right, StyleUnit::Point(OrderedFloat::from(right)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // fn get_margin_right(&mut self) -> f32 {
    //     self.yg_node.get_style().margin(Edge::Right).unwrap_or_default()
    // }
    //
    // fn get_margin_left(&mut self) -> f32 {
    //     self.yg_node.get_style().margin(Edge::Left).unwrap_or_default()
    // }
    //
    // /**
    //  * Sets the bottom margin of the view, aka the space that separates
    //  * this view and the surrounding ones.
    //  *
    //  * Only works with views that have parents - top level views that are pushed
    //  * on the stack don't have parents.
    //  *
    //  * Use brls::View::AUTO to have the layout automatically select the
    //  * margin.
    //  */
    // fn set_margin_bottom(&mut self, bottom: f32) {
    //     if bottom == AUTO {
    //         self.yg_node.set_margin(Edge::Bottom, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Bottom, StyleUnit::Point(OrderedFloat::from(bottom)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the right margin of the view, aka the space that separates
    //  * this view and the surrounding ones.
    //  *
    //  * Only works with views that have parents - top level views that are pushed
    //  * on the stack don't have parents.
    //  *
    //  * Use brls::View::AUTO to have the layout automatically select the
    //  * margin.
    //  */
    // fn set_margin_left(&mut self, left: f32) {
    //     if left == AUTO {
    //         self.yg_node.set_margin(Edge::Left, StyleUnit::Auto);
    //     } else {
    //         self.yg_node.set_margin(Edge::Left, StyleUnit::Point(OrderedFloat::from(left)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the visibility of the view.
    //  */
    // fn set_visibility(&mut self, visibility: Visibility) {
    //     // Only change YG properties and invalidate if going from or to GONE
    //     if (self.visibility == Visibility::Gone && visibility != Visibility::Gone) || (self.visibility != Visibility::Gone && visibility == Visibility::Gone)
    //     {
    //         if visibility == Visibility::Gone {
    //             self.yg_node.set_display(yoga::Display::None);
    //         } else {
    //             self.yg_node.set_display(yoga::Display::Flex);
    //         }
    //
    //         self.invalidate();
    //     }
    //
    //     self.visibility = visibility;
    //
    //     if visibility == Visibility::Visible {
    //         self.will_appear();
    //     }
    //     else {
    //         self.will_disappear();
    //     }
    // }
    //
    // /**
    //  * Sets the top position of the view, in pixels.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  *
    //  * If relative, it will simply offset the view by the given amount.
    //  *
    //  * If absolute, it will behave like the "display: absolute;" CSS property
    //  * and move the view freely in its parent. Use 0 to snap to the parent top edge.
    //  * Absolute positioning ignores padding.
    //  *
    //  * Use View::AUTO to disable (not the same as 0).
    //  */
    // fn set_position_top(&mut self, pos: f32) {
    //     if pos == AUTO {
    //         self.yg_node.set_position(Edge::Top, StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_position(Edge::Top, StyleUnit::Point(OrderedFloat::from(pos)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the right position of the view, in pixels.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  *
    //  * If relative, it will simply offset the view by the given amount.
    //  *
    //  * If absolute, it will behave like the "display: absolute;" CSS property
    //  * and move the view freely in its parent. Use 0 to snap to the parent right edge.
    //  * Absolute positioning ignores padding.
    //  *
    //  * Use View::AUTO to disable (not the same as 0).
    //  */
    // fn set_position_right(&mut self, pos: f32) {
    //     if pos == AUTO {
    //         self.yg_node.set_position(Edge::Right, StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_position(Edge::Right, StyleUnit::Point(OrderedFloat::from(pos)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the bottom position of the view, in pixels.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  *
    //  * If relative, it will simply offset the view by the given amount.
    //  *
    //  * If absolute, it will behave like the "display: absolute;" CSS property
    //  * and move the view freely in its parent. Use 0 to snap to the parent bottom edge.
    //  * Absolute positioning ignores padding.
    //  *
    //  * Use View::AUTO to disable (not the same as 0).
    //  */
    // fn set_position_bottom(&mut self, pos: f32) {
    //     if pos == AUTO {
    //         self.yg_node.set_position(Edge::Bottom, StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_position(Edge::Bottom, StyleUnit::Point(OrderedFloat::from(pos)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the left position of the view, in pixels.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  *
    //  * If relative, it will simply offset the view by the given amount.
    //  *
    //  * If absolute, it will behave like the "display: absolute;" CSS property
    //  * and move the view freely in its parent. Use 0 to snap to the parent left edge.
    //  * Absolute positioning ignores padding.
    //  *
    //  * Use View::AUTO to disable (not the same as 0).
    //  */
    // fn set_position_left(&mut self, pos: f32) {
    //     if pos == AUTO {
    //         self.yg_node.set_position(Edge::Left, StyleUnit::UndefinedValue);
    //     } else {
    //         self.yg_node.set_position(Edge::Left, StyleUnit::Point(OrderedFloat::from(pos)));
    //     }
    //
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the top position of the view, in percents.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  */
    // fn set_position_top_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_position(Edge::Top, StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the right position of the view, in percents.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  */
    // fn set_position_right_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_position(Edge::Right, StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the bottom position of the view, in percents.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  */
    // fn set_position_bottom_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_position(Edge::Bottom, StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the left position of the view, in percents.
    //  *
    //  * The behavior of this attribute changes depending on the
    //  * position type of the view.
    //  */
    // fn set_position_left_percentage(&mut self, percentage: f32) {
    //     self.yg_node.set_position(Edge::Left, StyleUnit::Percent(OrderedFloat::from(percentage)));
    //     self.invalidate();
    // }
    //
    // /**
    //  * Sets the id of the view.
    //  */
    // fn set_id(&mut self, id: &str) {
    //     self.id = id.parse().unwrap();
    // }
    //
    // /**
    //  * Overrides align items of the parent box.
    //  *
    //  * Default is AUTO.
    //  */
    // fn set_align_self(&mut self, align: AlignSelf) {
    //     match align {
    //         AlignSelf::Auto => {
    //             self.yg_node.set_align_self(Align::Auto);
    //         }
    //         AlignSelf::FlexStart => {
    //             self.yg_node.set_align_self(Align::FlexStart);
    //         }
    //         AlignSelf::Center => {
    //             self.yg_node.set_align_self(Align::Center);
    //         }
    //         AlignSelf::FlexEnd => {
    //             self.yg_node.set_align_self(Align::FlexEnd);
    //         }
    //         AlignSelf::Stretch => {
    //             self.yg_node.set_align_self(Align::Stretch);
    //         }
    //         AlignSelf::Baseline => {
    //             self.yg_node.set_align_self(Align::Baseline);
    //         }
    //         AlignSelf::SpaceBetween => {
    //             self.yg_node.set_align_self(Align::SpaceBetween);
    //         }
    //         AlignSelf::SpaceAround => {
    //             self.yg_node.set_align_self(Align::SpaceAround);
    //         }
    //     }
    //
    //     self.invalidate();
    // }
    //
    // // -----------------------------------------------------------
    // // Styling and view shape properties
    // // -----------------------------------------------------------
    //
    // /**
    //  * Sets the line color for the view. To be used with setLineTop(),
    //  * setLineRight()...
    //  *
    //  * The "line" is separate from the shape "border".
    //  */
    // fn set_line_color(&mut self, color: nanovg::Color) {
    //     self.line_color = color;
    // }
    //
    // /**
    //  * Sets the top line thickness. Use setLineColor()
    //  * to change the line color.
    //  *
    //  * The "line" is separate from the shape "border".
    //  */
    // fn set_line_top(&mut self, thickness: f32) {
    //     self.line_top = thickness;
    // }
    //
    // /**
    //  * Sets the right line thickness. Use setLineColor()
    //  * to change the line color.
    //  *
    //  * The "line" is separate from the shape "border".
    //  */
    // fn set_line_right(&mut self, thickness: f32) {
    //     self.line_right = thickness;
    // }
    //
    // /**
    //  * Sets the bottom line thickness. Use setLineColor()
    //  * to change the line color.
    //  *
    //  * The "line" is separate from the shape "border".
    //  */
    // fn set_line_bottom(&mut self, thickness: f32) {
    //     self.line_bottom = thickness;
    // }
    //
    // /**
    //  * Sets the left line thickness. Use setLineColor()
    //  * to change the line color.
    //  *
    //  * The "line" is separate from the shape "border".
    //  */
    // fn set_line_left(&mut self, thickness: f32) {
    //     self.line_left = thickness;
    // }
    //
    // /**
    //  * Sets the view shape background color.
    //  */
    // fn set_background_color(&mut self, color: nanovg::Color) {
    //     self.background_color = color;
    //     self.set_background(ViewBackground::ShapeColor);
    // }
    //
    // /**
    //  * Sets the view shape border color.
    //  */
    // fn set_border_color(&mut self, color: nanovg::Color) {
    //     self.border_color = color;
    // }
    //
    // /**
    //  * Sets the view shape border thickness.
    //  */
    // fn set_border_thickness(&mut self, thickness: f32) {
    //     self.border_thickness = thickness;
    // }
    //
    // fn get_border_thickness(&mut self) -> f32 {
    //     self.border_thickness
    // }
    //
    // /**
    //  * Sets the view shape corner radius.
    //  * 0 means no rounded corners.
    //  */
    // fn set_corner_radius(&mut self, radius: f32) {
    //     self.corner_radius = radius;
    // }
    //
    // /**
    //  * Sets the view shape shadow type.
    //  * Default is NONE.
    //  */
    // fn set_shadow_type(&mut self, shadow_type: ShadowType) {
    //     self.shadow_type = shadow_type;
    // }
    //
    // /**
    //  * Sets the shadow visibility.
    //  */
    // fn set_shadow_visibility(&mut self, visible: bool) {
    //     self.show_shadow = visible;
    // }
    //
    // /**
    //  * If set to true, the highlight background will be hidden for this view
    //  * (the white rectangle that goes behind the view, replacing the usual background shape).
    //  */
    // fn set_hide_highlight_background(&mut self, hide: bool) {
    //     self.hide_highlight_background = hide;
    // }
    //
    // /**
    //  * Sets the highlight padding of the view, aka the space between the
    //  * highlight rectangle and the view. The highlight rect is enlarged, the view is untouched.
    //  */
    // fn set_highlight_padding(&mut self, padding: f32) {
    //     self.highlight_padding = padding;
    // }
    //
    // /**
    //  * Sets the highlight rectangle corner radius.
    //  */
    // fn set_highlight_corner_radius(&mut self, radius: f32) {
    //     self.highlight_corner_radius = radius;
    // }
    //
    // // -----------------------------------------------------------
    //
    // /**
    //  * Returns the "nearest" view with the corresponding id, or nullptr if none has
    //  * been found. "Nearest" means the closest in the vicinity
    //  * of this view. The siblings are searched as well as its children.
    //  *
    //  * Research is done by traversing the tree upwards, starting from this view.
    //  * The current algorithm is very inefficient.
    //  */
    // pub(crate) fn get_nearest_view(&mut self, id: &str) -> Option<&BaseView> {
    //     // First try children of ours
    //     if let Some(child) = self.get_view(id) {
    //         return Some(child);
    //     }
    //
    //     // Then go up one level and try again
    //     if let Some(parent) = self.get_parent() {
    //         return parent.get_nearest_view(id);
    //     }
    //
    //     None
    // }
    //
    // /**
    //  * If set to true, will force the view to be translucent.
    //  */
    // fn set_in_fade_animation(&mut self, translucent: bool) {
    //     self.in_fade_animation = translucent;
    // }
    //
    // /**
    //  * Sets the view to be focusable.
    //  *
    //  * Required to be able to use actions that need
    //  * focus on that view (such as an A press).
    //  */
    // fn set_focusable(&mut self, focusable: bool) {
    //     self.focusable = focusable;
    // }
    //
    // fn is_focusable(&self) -> bool {
    //     self.focusable
    // }
    //
    // /**
    //  * Sets the sound to play when this view gets focused.
    //  */
    // fn set_focus_sound(&mut self, sound: audio::Sound) {
    //     self.focus_sound = sound;
    // }
    //
    // pub(crate) fn get_focus_sound(&mut self) -> audio::Sound {
    //     self.focus_sound.clone()
    // }
    //
    // /**
    //  * Sets the detached flag to true.
    //  * This action is irreversible.
    //  *
    //  * A detached view will, as the name suggests, not be
    //  * attached to their parent Yoga node. That means that invalidation
    //  * and layout need to be taken care of manually by the parent.
    //  *
    //  * detach() must be called before adding the view to the parent.
    //  */
    // fn detach(&mut self) {
    //     self.detached = true;
    // }
    //
    // fn is_detached(&self) -> bool {
    //     self.detached
    // }
    //
    // /**
    //  * Sets the position of the view, if detached.
    //  */
    // fn set_detached_position(&mut self, x: f32, y: f32) {
    //     self.detached_origin_x = x;
    //     self.detached_origin_y = y;
    // }
    //
    // fn set_parent(&self, parent: &mut Box<BaseView>) {
    //     self.parent_userdata;
    // }
    //
    // pub(crate) fn get_parent(&mut self) -> Option<&mut Box<BaseView>> {
    //     // Placeholder; replace with your implementation
    //     None
    // }
    //
    // pub(crate) fn has_parent(&self) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // fn get_parent_user_data(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Registers an action with the given parameters. The listener will be fired when the user presses
    //  * the key when the view is focused.
    //  *
    //  * The listener should return true if the action was consumed, false otherwise.
    //  * The sound will only be played if the listener returned true.
    //  *
    //  * A hidden action will not show up in the bottom-right hints.
    //  *
    //  * Returns the identifier for the action, so it can be unregistered later on. Returns ACTION_NONE if the
    //  * action was not registered.
    //  */
    // fn register_action(
    //     &mut self,
    //     hint_text: &str,
    //     button: ControllerButton,
    //     action_listener: ActionListener,
    //     hidden: bool,
    //     sound: Sound,
    // ) -> ActionIdentifier {
    //     let next_indentifier: ActionIdentifier = match self.actions.len() == 0  {
    //         true => {1}
    //         false => {
    //             self.actions.last().unwrap().identifier + 1
    //         }
    //     };
    //
    //     let mut found = false;
    //
    //     for action in self.actions {
    //         if action.identifier == next_indentifier {
    //             found = true;
    //             break;
    //         }
    //     }
    //
    //     if !found {
    //         self.actions.push(crate::lib::core::actions::Action::new(
    //             button,
    //             next_indentifier,
    //             hint_text,
    //             true,
    //             hidden,
    //             sound,
    //             action_listener,
    //         ));
    //     }
    //
    //     next_indentifier
    // }
    //
    // /**
    //  * Unregisters an action with the given identifier.
    //  */
    // fn unregister_action(&mut self, identifier: ActionIdentifier) {
    //     self.actions.retain(|action| {
    //         action.identifier != identifier
    //     });
    // }
    //
    // // Shortcut to register a generic "A OK" click action
    // fn register_click_action(&mut self, action_listener: ActionListener) {
    //     self.register_action("brls/hints/ok", ControllerButton::ButtonA, action_listener, false, Sound::SoundClick);
    // }
    //
    // // Update action hint for the specified controller button
    // fn update_action_hint(&mut self, button: ControllerButton, hint_text: String) {
    //     // Update the action hint based on the button
    //     // Implement this based on your actual requirements
    // }
    //
    // // Set action availability for the specified controller button
    // fn set_action_available(&mut self, button: ControllerButton, available: bool) {
    //     // Set action availability based on the button
    //     // Implement this based on your actual requirements
    // }
    //
    // // Reset click animation state
    // pub(crate) fn reset_click_animation(&mut self) {
    //     self.click_animation_playing = false;
    // }
    //
    // // Play click animation, with an optional reverse parameter
    // fn play_click_animation(&mut self, reverse: bool) {
    //     self.reset_click_animation();
    // }
    //
    // fn get_yg_node(&self) -> &Node {
    //     &self.yg_node
    // }
    //
    // pub(crate) fn get_actions(&self) -> &Vec<crate::lib::core::actions::Action> {
    //     &self.actions
    // }
    //
    // /**
    //  * Called each frame
    //  * Do not override it to draw your view,
    //  * override draw() instead
    //  */
    // pub(crate) fn frame(&self, ctx: &FrameContext) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Called by frame() to draw the view onscreen.
    //  * Views should not draw outside of their bounds (they
    //  * may be clipped if they do so).
    //  */
    // fn draw(&self, vg: &NVGcontext, x: f32, y: f32, width: f32, height: f32, style: &Style, ctx: &FrameContext) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Called when the view will appear
    //  * on screen, before or after layout().
    //  *
    //  * Can be called if the view has
    //  * already appeared, so be careful.
    //  */
    // fn will_appear(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Called when the view will disappear
    //  * from the screen.
    //  *
    //  * Can be called if the view has
    //  * already disappeared, so be careful.
    //  */
    // fn will_disappear(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    //
    // /**
    //  * Called when the show() animation (fade in)
    //  * ends
    //  */
    // fn on_show_animation_end(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Shows the view with a fade in animation.
    //  */
    // fn show(&self, cb: Option<Box<dyn Fn()>>) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Shows the view with a fade in animation, or no animation at all.
    //  */
    // fn show_with_animation(&self, cb: Option<Box<dyn Fn()>>, animate: bool, animation_duration: f32) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Returns the duration of the view show / hide animation.
    //  */
    // fn get_show_animation_duration(&self, animation: TransitionAnimation) -> f32 {
    //     // Placeholder; replace with your implementation
    //     0.0
    // }
    //
    // /**
    //  * Hides the view in a collapse animation
    //  */
    // fn collapse(&self, animated: bool) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // fn is_collapsed(&self) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // fn set_alpha(&self, alpha: f32) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Shows the view in a expand animation (opposite
    //  * of collapse)
    //  */
    // fn expand(&self, animated: bool) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Hides the view with a fade out animation.
    //  */
    // fn hide(&self, cb: Option<Box<dyn Fn()>>) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Hides the view with a fade out animation, or no animation at all.
    //  */
    // fn hide_with_animation(&self, cb: Option<Box<dyn Fn()>>, animate: bool, animation_duration: f32) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // fn is_hidden(&self) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // /**
    //  * Is this view translucent?
    //  *
    //  * If you override it please return
    //  * <value> || View::isTranslucent()
    //  * to keep the fadeIn transition
    //  */
    // fn is_translucent(&self) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // fn is_focused(&self) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // /**
    //  * Returns the default view to focus when focusing this view
    //  * Typically the view itself or one of its children.
    //  *
    //  * Returning nullptr means that the view is not focusable
    //  * (and neither are its children)
    //  *
    //  * By default, a view is focusable if the flag is set to true with setFocusable()
    //  * and if the view is visible.
    //  *
    //  * When pressing a key, the flow is :
    //  *    1. starting from the currently focused view's parent, traverse the tree upwards and
    //  *       repeatedly call getNextFocus() on every view until we find a next view to focus or meet the end of the tree
    //  *    2. if a view is found, getNextFocus() will internally call getDefaultFocus() for the selected child
    //  *    3. give focus to the result, if it exists
    //  */
    // fn get_default_focus(&self) -> Option<&BaseView> {
    //     // Placeholder; replace with your implementation
    //     None
    // }
    //
    // /**
    //  * Returns the next view to focus given the requested direction
    //  * and the currently focused view (as parent user data)
    //  *
    //  * Returning nullptr means that there is no next view to focus
    //  * in that direction - getNextFocus will then be called on our
    //  * parent if any
    //  */
    // pub(crate) fn get_next_focus(&self, direction: FocusDirection, current_view: &BaseView) -> Option<&BaseView> {
    //     // Placeholder; replace with your implementation
    //     None
    // }
    //
    // /**
    //  * Sets a custom navigation route from this view to the target one.
    //  */
    // fn set_custom_navigation_route_by_ptr(&self, direction: FocusDirection, target: &BaseView) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Sets a custom navigation route from this view to the target one, by ID.
    //  * The final target view will be the "nearest" with the given ID.
    //  *
    //  * Resolution of the ID to View is made when the navigation event occurs, not when the
    //  * route is registered.
    //  */
    // fn set_custom_navigation_route_by_id(&self, direction: FocusDirection, target_id: &str) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // pub(crate) fn has_custom_navigation_route_by_ptr(&self, direction: FocusDirection) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // pub(crate) fn has_custom_navigation_route_by_id(&self, direction: FocusDirection) -> bool {
    //     // Placeholder; replace with your implementation
    //     false
    // }
    //
    // fn get_custom_navigation_route_by_ptr(&self, direction: FocusDirection) -> Option<&BaseView> {
    //     // Placeholder; replace with your implementation
    //     None
    // }
    //
    // fn get_custom_navigation_route_by_id(&self, direction: FocusDirection) -> Option<&str> {
    //     // Placeholder; replace with your implementation
    //     None
    // }
    //
    // /**
    //  * Fired when focus is gained.
    //  */
    // fn on_focus_gained(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // /**
    //  * Fired when focus is lost.
    //  */
    // fn on_focus_lost(&self) {
    //     // Placeholder; replace with your implementation
    // }
    //
    // // Fired when focus is gained on this view's parent, or the parent of the parent...
    // fn on_parent_focus_gained(&mut self, focused_view: &BaseView) {
    //     // Implement based on your requirements
    // }
    //
    // // Fired when focus is lost on one of this view's parents
    // fn on_parent_focus_lost(&mut self, focused_view: &BaseView) {
    //     // Implement based on your requirements
    // }
    //
    // // Fired when the window size changes
    // fn on_window_size_changed(&self) {
    //     // Nothing by default
    // }
    //
    // // Get focus event
    // fn get_focus_event(&self) -> Option<&GenericEvent> {
    //     // Implement based on your requirements
    //     None
    // }
    //
    // // Get alpha
    // fn get_alpha(&self) -> f32 {
    //     self.alpha.get_value()
    // }
    //
    // // Forces this view and its children to use the specified theme
    // fn override_theme(&mut self, new_theme: Rc<RefCell<Theme>>) {
    //     // Implement based on your requirements
    // }
    //
    // // Enable/disable culling for that view
    // fn set_culled(&mut self, culled: bool) {
    //     self.culled = culled;
    // }
    //
    // // Check if view is culled
    // fn is_culled(&self) -> bool {
    //     self.culled
    // }
    //
    // // Set the Y translation of this view
    // fn set_translation_y(&mut self, translate_y: f32) {
    //     self.translation_y = translate_y;
    // }
    //
    // // Set the X translation of this view
    // fn set_translation_x(&mut self, translate_x: f32) {
    //     self.translation_x = translate_x;
    // }
    //
    // // Enable/disable wireframe mode
    // fn set_wireframe_enabled(&mut self, wireframe: bool) {
    //     self.wireframe_enabled = wireframe;
    // }
    //
    // // Check if wireframe mode is enabled
    // fn is_wireframe_enabled(&self) -> bool {
    //     self.wireframe_enabled
    // }
    //
    // // Resolves the value of the given XML attribute string
    // fn get_string_xml_attribute_value(value: &str) -> String {
    //     // Implement based on your requirements
    //     value.to_string()
    // }
    //
    // // Resolves the value of the given XML attribute file path
    // fn get_file_path_xml_attribute_value(value: &str) -> String {
    //     // Implement based on your requirements
    //     value.to_string()
    // }
}