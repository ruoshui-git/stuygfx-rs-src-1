//! A very simple turtle drawer
//! 
//! This drawer will only draw on the plane Z = 0 (no depth)

use crate::{prelude::*, util::polar_to_xy};

/// A simple turtle drawer on the plane Z = 0 (no depth)
/// 
/// # Note
/// When making a [`Turtle`], the [`Screen`] will be owned by the turtle. If you want to work with the [`Screen`], you need to drop
/// the turtle. 
/// 
/// Even though multiple turtles on the same [`Screen`], at the same time, is a possibility, 
/// it requires concurrency and a lot of work to get right. Sorry if you're disapointed that you
/// fell in love with NetLogo in introCS and can't find that love here. Love the [`Turtle`] in a new way or implement that on your own.
/// Rust is known for [fearless concurrency], so, march ahead!
/// 
/// [`Turtle`]: ./struct.Turtle.html
/// [`Screen`]: ../screen/trait.Screen.html
/// [fearless concurrency]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
pub struct Turtle<T: Screen> {
    x: f64,
    y: f64,
    /// Direction of turtle in degrees. Goes counterclockwise, and `0.` is facing right.
    pub direction: f64,
    /// Whether movement of turtle will draw on the screen
    pub pen_down: bool,
    /// The color to draw with
    pub fg_color: Rgb,
    img: T,
}

impl<T: Screen> Turtle<T> {

    /// Make a new [`Turtle`]
    /// [`Turtle`]: ./struct.Turtle.html
    pub fn new(screen: T, x: f64, y: f64, fg_color: Rgb) -> Turtle<T> {
        Turtle {
            x,
            y,
            direction: 0.0,
            pen_down: false,
            img: screen,
            fg_color,
        }
    }

    /// Move forward, draw if `pen_down` is true.
    pub fn forward(&mut self, steps: f64) {
        let (x0, y0) = (self.x, self.y);
        let (dx, dy) = polar_to_xy(steps.into(), self.direction);
        let (x1, y1) = (x0 + dx, y0 + dy);
        if self.pen_down {
            self.img
                .draw_line((x0, y0, 0.), (x1, y1, 0.), self.fg_color);
        }
        self.x = x1;
        self.y = y1;
    }

    /// Turn right `angle_deg` degrees without changing location.
    pub fn turn_right(&mut self, angle_deg: f64) {
        self.direction = (self.direction + angle_deg) % 360.0;
    }

    /// Turn left `angle_deg` degrees without changing location.
    pub fn turn_left(&mut self, angle_deg: f64) {
        self.turn_right(-angle_deg);
    }

    /// Set position to (x, y), draw a line to the point if `pen_down` is true.
    pub fn move_to(&mut self, x: f64, y: f64) {
        if self.pen_down {
            self.img
                .draw_line((self.x, self.y, 0.), (x, y, 0.), self.fg_color);
        }
        self.x = x;
        self.y = y;
    }

    /// Get the inner Screen (T) instance
    ///
    /// This method will drop the turtle. You should use this when you want to continue working with the image directly,
    /// or write the image to file.
    pub fn get_screen(self) -> T {
        self.img
    }
}
