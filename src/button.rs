
use color::Color;
use dimensions::Dimensions;
use opengl_graphics::Gl;
use mouse::Mouse;
use point::Point;
use rectangle;
use ui_context::{
    UIID,
    UiContext,
};
use widget::Widget;

/// Represents the state of the Button widget.
#[deriving(PartialEq, Clone, Copy)]
pub enum State {
    Normal,
    Highlighted,
    Clicked,
}

impl State {
    /// Return the associated Rectangle state.
    fn as_rectangle_state(&self) -> rectangle::State {
        match self {
            &State::Normal => rectangle::State::Normal,
            &State::Highlighted => rectangle::State::Highlighted,
            &State::Clicked => rectangle::State::Clicked,
        }
    }
}

widget_fns!(Button, State, Widget::Button(State::Normal));

/// Check the current state of the button.
fn get_new_state(is_over: bool,
                 prev: State,
                 mouse: Mouse) -> State {
    use mouse::ButtonState::{Down, Up};
    use self::State::{Normal, Highlighted, Clicked};
    match (is_over, prev, mouse.left) {
        (true,  Normal,  Down) => Normal,
        (true,  _,       Down) => Clicked,
        (true,  _,       Up)   => Highlighted,
        (false, Clicked, Down) => Clicked,
        _                      => Normal,
    }
}

/// A context on which the builder pattern can be implemented.
pub struct ButtonContext<'a> {
    uic: &'a mut UiContext,
    ui_id: UIID,
    pos: Point,
    dim: Dimensions,
    maybe_color: Option<Color>,
    maybe_frame: Option<f64>,
    maybe_frame_color: Option<Color>,
    maybe_label: Option<&'a str>,
    maybe_label_color: Option<Color>,
    maybe_label_font_size: Option<u32>,
    maybe_callback: Option<||:'a>,
}

pub trait ButtonBuilder<'a> {
    /// A button builder method to be implemented by the UiContext.
    fn button(&'a mut self, ui_id: UIID) -> ButtonContext<'a>;
}

impl<'a> ButtonBuilder<'a> for UiContext {

    /// Create a button context to be built upon.
    fn button(&'a mut self, ui_id: UIID) -> ButtonContext<'a> {
        ButtonContext {
            uic: self,
            ui_id: ui_id,
            pos: [0.0, 0.0],
            dim: [64.0, 64.0],
            maybe_callback: None,
            maybe_color: None,
            maybe_frame: None,
            maybe_frame_color: None,
            maybe_label: None,
            maybe_label_color: None,
            maybe_label_font_size: None,
        }
    }

}

impl_callable!(ButtonContext, ||:'a);
impl_colorable!(ButtonContext);
impl_frameable!(ButtonContext);
impl_labelable!(ButtonContext);
impl_positionable!(ButtonContext);
impl_shapeable!(ButtonContext);

impl<'a> ::draw::Drawable for ButtonContext<'a> {
    fn draw(&mut self, graphics: &mut Gl) {

        let state = *get_state(self.uic, self.ui_id);
        let mouse = self.uic.get_mouse_state();
        let is_over = rectangle::is_over(self.pos, mouse.pos, self.dim);
        let new_state = get_new_state(is_over, state, mouse);

        // Callback.
        match (is_over, state, new_state) {
            (true, State::Clicked, State::Highlighted) => match self.maybe_callback {
                Some(ref mut callback) => (*callback)(), None => (),
            }, _ => (),
        }

        // Draw.
        let rect_state = new_state.as_rectangle_state();
        let color = self.maybe_color.unwrap_or(self.uic.theme.shape_color);
        let frame_w = self.maybe_frame.unwrap_or(self.uic.theme.frame_width);
        let maybe_frame = match frame_w > 0.0 {
            true => Some((frame_w, self.maybe_frame_color.unwrap_or(self.uic.theme.frame_color))),
            false => None,
        };
        match self.maybe_label {
            None => {
                rectangle::draw(
                    self.uic.win_w, self.uic.win_h, graphics, rect_state, self.pos,
                    self.dim, maybe_frame, color
                )
            },
            Some(text) => {
                let text_color = self.maybe_label_color.unwrap_or(self.uic.theme.label_color);
                let size = self.maybe_label_font_size.unwrap_or(self.uic.theme.font_size_medium);
                rectangle::draw_with_centered_label(
                    self.uic.win_w, self.uic.win_h, graphics, self.uic, rect_state,
                    self.pos, self.dim, maybe_frame, color,
                    text, size, text_color
                )
            },
        }

        set_state(self.uic, self.ui_id, new_state, self.pos, self.dim);

    }
}
