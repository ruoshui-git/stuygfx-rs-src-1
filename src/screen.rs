//! Contains the `Screen` trait.
//!
//! The [`Screen`] trait can potentially abstract over actual image buffers. However, we will probably
//! only use Ppm in this class, so, really, this trait is mostly unecessary.
//! Here serves more as a demonstration of Rust's generics and dynamic dispatch to those new to the language.
//!
//! [`Screen`]: ./trait.Screen.html

use std::io;

use crate::{color::Rgb, util};

/// A trait for image buffers.
///
/// This trait requires basic functions to be implemented, and will provide default implementation for drawing lines, shapes, etc.
/// This is also where most of the drawing work for this graphics course will be done.
pub trait Screen {
    /// Plot a point on the screen at (`x`, `y`, `z`)
    fn plot(&mut self, x: i64, y: i64, z: f64, color: Rgb);

    /// Save image to the given filename. Output file type is guessed from the file extension.
    fn save(&self, filepath: &str) -> io::Result<()>;

    /// Return the width of the screen
    fn width(&self) -> usize;

    /// Return the height of the screen
    fn height(&self) -> usize;

    /// Write image data to the given `writer`. `writer` will ***not*** be buffered before writing to.
    fn write_to_buf<T: io::Write>(&self, writer: &mut T) -> io::Result<()>;

    /// Display the image with imagemagick
    fn display(&self) -> io::Result<()>;

    /// Clear the screen (fill with `color`) and reset configurations like z-buffer
    fn clear(&mut self, color: Rgb);

    // Default methods -----

    /// Draw a line from `p0 (x, y, z)` to `p1 (x, y z)`, with the given `color`
    /// 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use graphics::prelude::*;
    /// 
    /// let mut img = Ppm::new();
    /// 
    /// img.draw_line((3., 499. - 1., 0.), (250., 0., 0.), Rgb::WHITE);
    /// ```
    ///
    /// # Implementation
    /// 
    /// This method will call [`plot`] to to draw a single point on the image.
    /// 
    /// `color` should be directly passed down to `plot`, so all pixels of a line have the same color. 
    /// (You can definitely change that behavior if you want to do something cool!)
    /// 
    /// The `z` coordinates will be used later. For now, just ignore them and use `0.` for `z` when `plot`ting.
    /// 
    /// The method accepts 2 tuples of length 3. If you don't know how to work with them, you can learn about them on [Rust By Example].
    /// The 2 tuples are meant to represent (x, y, z), in that order. Same goes for the tuple in [`plot`].
    /// 
    /// ## Implementation note (from past error):
    /// 
    /// Always add 2A or 2B when updating D. Half of that value will distort line.
    /// 
    /// [`plot`]: #method.plot
    /// [Rust By Example]: https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html
    fn draw_line(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64), color: Rgb) {
        todo!("draw_line")
    }

    /// Draw a line from (x, y, z) with a certain magnitude and angle, on the same z-plane as the point.
    ///
    /// Angle goes counter clockwise from x axis.
    ///
    /// Returns the other endpoint of the line `(x1, y1, z)` as a tuple.
    ///
    /// This function requires [`draw_line`] to be properly implemented.
    ///
    /// [`draw_line`]: #method.draw_line
    fn draw_line_degrees(
        &mut self,
        p0: (f64, f64, f64),
        angle_degrees: f64,
        mag: f64,
        color: Rgb,
    ) -> (f64, f64, f64) {
        let (dx, dy) = util::polar_to_xy(mag, angle_degrees);
        let p1 = (p0.0 + dx, p0.1 + dy, p0.2);

        self.draw_line(p0, p1, color);
        p1
    }
}
