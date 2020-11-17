//! Contains the `Turtle` struct.

use crate::{prelude::*, util::polar_to_xy};

/// A simple turtle drawer on the plane Z = 0 (no depth).
///
/// # Examples
/// ```
/// use graphics::prelude::*;
/// use graphics::turtle::Turtle;
///
/// let xmax = 500.;
/// let ymax = 500.;
///
/// // A turtle at the center of the screen
/// let mut turtle = Turtle::new(Ppm::new(), xmax / 2., ymax / 2., Rgb::WHITE);
///
/// turtle.direction = 0.;
///
/// let radius = xmax / 4.;
/// turtle.forward(radius);
/// turtle.turn_left(90.);
///
/// let circumf = radius * std::f64::consts::TAU;
/// let total_steps = 360;
///
/// turtle.pen_down = true;
///
/// for _ in 0..total_steps {
///     turtle.forward(circumf / total_steps as f64);
///     turtle.turn_left(360. / total_steps as f64);
/// }
///
/// let img = turtle.get_screen();
///
/// // img.display();
/// // img.save("circle.png");
/// ```
///
/// # Ownership
/// When making a [`Turtle`], the [`Screen`] will be owned by the turtle. If you want to work with the [`Screen`] directly, you need to drop
/// the turtle.
///
/// Even though multiple turtles drawing on the same [`Screen`], at the same time, is a possibility,
/// it requires concurrency and a lot of work to get right. Sorry if you're disapointed that you
/// fell in love with NetLogo in introCS and can't find that love here. Love the [`Turtle`] in a new way or implement that on your own.
/// Rust is known for [fearless concurrency], so, march ahead!
///
/// # Generics
/// This struct is parameterized by [`Screen`], so only a type that impl [`Screen`] can be used.
/// This makes sense because any screen should be able to support drawing via a turtle, and the turtle should be able to draw on any
/// screen.
///
/// For more info on generics, see the Rust book's section on [generics] and [generic data types].
///
/// [`Turtle`]: ./struct.Turtle.html
/// [`Screen`]: ../screen/trait.Screen.html
/// [fearless concurrency]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
/// [generics]: https://doc.rust-lang.org/book/ch10-00-generics.html
/// [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html
pub struct Turtle<T: Screen> {
    x: f64,
    y: f64,
    /// Direction of turtle in degrees. Goes counterclockwise, and `0.` is facing right.
    pub direction: f64,
    /// If `true`, movement of turtle will draw on the screen.
    pub pen_down: bool,
    /// The color to draw with.
    pub fg_color: Rgb,
    img: T,
}

impl<T: Screen> Turtle<T> {
    /// Make a new [`Turtle`] at (`x`, `y`), facing right.
    ///
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

    /// Get the inner [`Screen`] (T) instance.
    ///
    /// This method will destroy (move) the turtle. You should use this when you want to continue working with the image directly,
    /// or write the image to a file.
    ///
    /// [`Screen`]: ../screen/trait.Screen.html
    pub fn get_screen(self) -> T {
        self.img
    }
}
