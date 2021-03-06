use callback::Callable;
use frame::Frameable;
use color::{ Color, Colorable };
use label::{ FontSize, Labelable };
use dimensions::Dimensions;
use mouse::Mouse;
use point::Point;
use position::Positionable;
use shape::Shapeable;
use rectangle;
use graphics::Graphics;
use graphics::character::CharacterCache;
use ui::{ UIID, Ui };
use widget::Widget;

/// Represents the state of the Toggle widget.
#[derive(PartialEq, Clone, Copy)]
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

widget_fns!(Toggle, State, Widget::Toggle(State::Normal));

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
pub struct Toggle<'a, F> {
    ui_id: UIID,
    pos: Point,
    dim: Dimensions,
    maybe_callback: Option<F>,
    maybe_color: Option<Color>,
    maybe_frame: Option<f64>,
    maybe_frame_color: Option<Color>,
    maybe_label: Option<&'a str>,
    maybe_label_color: Option<Color>,
    maybe_label_font_size: Option<u32>,
    value: bool,
}

impl<'a, F> Toggle<'a, F> {

    /// Create a toggle context to be built upon.
    pub fn new(ui_id: UIID, value: bool) -> Toggle<'a, F> {
        Toggle {
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
            value: value,
        }
    }

}

impl<'a, F> Colorable for Toggle<'a, F> {
    fn color(mut self, color: Color) -> Self {
        self.maybe_color = Some(color);
        self
    }
}

impl<'a, F> Frameable for Toggle<'a, F> {
    fn frame(mut self, width: f64) -> Self {
        self.maybe_frame = Some(width);
        self
    }
    fn frame_color(mut self, color: Color) -> Self {
        self.maybe_frame_color = Some(color);
        self
    }
}

impl<'a, F> Callable<F> for Toggle<'a, F> {
    fn callback(mut self, cb: F) -> Self {
        self.maybe_callback = Some(cb);
        self
    }
}

impl<'a, F> Labelable<'a> for Toggle<'a, F>
{
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }

    fn label_color(mut self, color: Color) -> Self {
        self.maybe_label_color = Some(color);
        self
    }

    fn label_font_size(mut self, size: FontSize) -> Self {
        self.maybe_label_font_size = Some(size);
        self
    }
}

impl<'a, F> Positionable for Toggle<'a, F> {
    fn point(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }
}

impl<'a, F> Shapeable for Toggle<'a, F> {
    fn get_dim(&self) -> Dimensions { self.dim }
    fn dim(mut self, dim: Dimensions) -> Self { self.dim = dim; self }
}

impl<'a, F> ::draw::Drawable for Toggle<'a, F> where F: FnMut(bool) + 'a {
    fn draw<B, C>(&mut self, ui: &mut Ui<C>, graphics: &mut B)
        where
            B: Graphics<Texture = <C as CharacterCache>::Texture>,
            C: CharacterCache
    {
        let color = self.maybe_color.unwrap_or(ui.theme.shape_color);
        let color = match self.value {
            true => color,
            false => color * Color::new(0.1, 0.1, 0.1, 1.0)
        };
        let state = *get_state(ui, self.ui_id);
        let mouse = ui.get_mouse_state();
        let is_over = rectangle::is_over(self.pos, mouse.pos, self.dim);
        let new_state = get_new_state(is_over, state, mouse);
        let rect_state = new_state.as_rectangle_state();
        match self.maybe_callback {
            Some(ref mut callback) => {
                match (is_over, state, new_state) {
                    (true, State::Clicked, State::Highlighted) =>
                        (*callback)(match self.value { true => false, false => true }),
                    _ => (),
                }
            }, None => (),
        }
        let frame_w = self.maybe_frame.unwrap_or(ui.theme.frame_width);
        let maybe_frame = match frame_w > 0.0 {
            true => Some((frame_w, self.maybe_frame_color.unwrap_or(ui.theme.frame_color))),
            false => None,
        };
        match self.maybe_label {
            None => {
                rectangle::draw(
                    ui.win_w, ui.win_h, graphics, rect_state, self.pos,
                    self.dim, maybe_frame, color
                )
            },
            Some(text) => {
                let text_color = self.maybe_label_color.unwrap_or(ui.theme.label_color);
                let size = self.maybe_label_font_size.unwrap_or(ui.theme.font_size_medium);
                rectangle::draw_with_centered_label(
                    ui.win_w, ui.win_h, graphics, ui, rect_state,
                    self.pos, self.dim, maybe_frame, color,
                    text, size, text_color
                )
            },
        }

        set_state(ui, self.ui_id, Widget::Toggle(new_state), self.pos, self.dim);

    }
}
