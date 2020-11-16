//! PPM image and related functionalities.
//!
//! For info on file format, see the [ppm file format specification][ppm spec].
//!
//! Functionalities related to writing ppm files have been
//! already implemented for you. But you'll need to be famailiar with this module to write images.
//!
//! # The Builder Pattern
//! This module uses the [builder pattern], in which a [`PpmBuilder`] is first created, configured, and finally you call `.build()`
//! to generate a [`Ppm`] from that. This provides a convenient way to initialize a struct (object) with a variety of settings.
//! In Python, this is usually achieved through default and keyward function arguments.
//!
//! [`Ppm`]: ./struct.Ppm.html
//! [`PpmBuilder`]: ./struct.PpmBuilder.html
//! [ppm spec]: http://netpbm.sourceforge.net/doc/ppm.html
//! [builder pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

use std::{
    convert::TryFrom,
    io::{self, prelude::*, BufWriter},
};

use crate::{color::Rgb, magick, screen::Screen};

/// Builder for [`Ppm`].
///
/// The builder pattern allows a [`Ppm`] to be constructed in one line, or on separate lines.
///
/// # Examples
///
/// ```rust
/// use graphics::prelude::*;
///
/// let mut img = PpmBuilder::new(500, 1000, 255)
///     .bg_color(Rgb::WHITE)
///     .wrap_x(true)
///     .build();
/// 
/// // draw stuff...
///
/// assert_eq!(1000, img.width());
/// assert_eq!(500, img.height());
/// assert_eq!(true, img.wrap_x);
/// assert_eq!(false, img.wrap_y);
/// assert_eq!(true, img.invert_y);
/// ```
///
/// The multi-line version
///
/// ```rust
/// use graphics::prelude::*;
///
/// let builder = PpmBuilder::new(400, 500, 255);
///
/// let builder = builder.bg_color(Rgb::gray(20));
///
/// // ... some code
///
/// let builder = builder.wrap_y(true);
/// let builder = builder.invert_y(false);
/// let mut img = builder.build();
/// 
/// // draw stuff...
///
/// assert_eq!(true, img.wrap_y);
/// assert_eq!(false, img.wrap_x);
/// assert_eq!(false, img.invert_y);
/// assert_eq!(500, img.width());
/// assert_eq!(400, img.height());
/// ```
///
/// [`Ppm`]: ./struct.Ppm.html
pub struct PpmBuilder {
    /// Height of the image to be built.
    height: usize,
    /// Width of the image to be built.
    width: usize,
    /// Max value of color_depth is 2^16, per ppm spec.
    color_depth: u16, // max = 2^16
    /// Image data. See [`Ppm`] field [`data`] for why a 1D Vec is used.
    /// 
    /// [`data`]: ./struct.Ppm.html#structfield.data
    /// [`Ppm`]: ./struct.Ppm.html
    data: Vec<Rgb>,
    /// Color to fill a Ppm on build. Not used if data is provided with [`with_data`].
    ///
    /// [`with_data`]: #method.with_data
    pub bg_color: Rgb,
    /// If true, x values outside image will be wrapped around. if false, the point will be ignored. Defaults to `false`.
    pub wrap_x: bool,
    /// If true, y values outside image will be wrapped around. If false, the point will be ignored. Defaults to `false`.
    pub wrap_y: bool,
    /// Whether y values will be inverted when plotting. Defaults to true, which puts the origin on the bottom left.
    pub invert_y: bool,
}

impl PpmBuilder {
    pub const DEFAULT_BG_COLOR: Rgb = Rgb::BLACK;

    /// Make a new PpmBuilder with default configurations.
    pub fn new(height: usize, width: usize, color_depth: u16) -> Self {
        Self {
            height,
            width,
            color_depth,
            wrap_x: false,
            wrap_y: false,
            invert_y: true,
            data: vec![],
            bg_color: Self::DEFAULT_BG_COLOR,
        }
    }

    /// Set `wrap_x`.
    ///
    /// [`wrap_x`]: #structfield.wrap_y
    /// [`PpmBuilder`]: ./struct.PpmBuilder.html
    pub fn wrap_x(mut self, to_wrap: bool) -> Self {
        self.wrap_x = to_wrap;
        self
    }

    /// Set `wrap_y`.
    ///
    /// [`wrap_y`]: #structfield.wrap_y
    /// [`PpmBuilder`]: ./struct.PpmBuilder.html
    pub fn wrap_y(mut self, to_wrap: bool) -> Self {
        self.wrap_y = to_wrap;
        self
    }

    /// Set `invert_y`.
    ///
    /// [`invert_y`]: #structfield.invert_y
    /// [`PpmBuilder`]: ./struct.PpmBuilder.html
    pub fn invert_y(mut self, to_invert: bool) -> Self {
        self.invert_y = to_invert;
        self
    }

    /// Set initial data.
    ///
    /// If initial image data is provided with this method, `bg_color` will not be used.
    pub fn with_data(mut self, data: Vec<Rgb>) -> Self {
        self.data = data;
        self
    }

    /// Set background color.
    pub fn bg_color(mut self, bg_color: Rgb) -> Self {
        self.bg_color = bg_color;
        self
    }

    /// Build a [`Ppm`]. Always remember to call this method after configuring a [`PpmBuilder`].
    /// 
    /// [`Ppm`]: ./struct.Ppm.html
    /// [`PpmBuilder`]: ./struct.PpmBuilder.html
    pub fn build(self) -> Ppm {
        Ppm {
            height: self.height,
            width: self.width,
            color_depth: self.color_depth,
            wrap_x: self.wrap_x,
            wrap_y: self.wrap_y,
            invert_y: self.invert_y,
            data: if self.data.is_empty() {
                vec![self.bg_color; self.width * self.height]
            } else {
                self.data
            },
            zbuf: vec![f64::NEG_INFINITY; self.width * self.height],
        }
    }
}

/// Represent a ppm image.
pub struct Ppm {
    /// Height of the image (max y value).
    height: usize,
    /// Width of the image (max x value).
    width: usize,
    /// Max value of color_depth is 2^16, per ppm spec.
    color_depth: u16, // max = 2^16
    /// If true, x values outside image will be wrapped around. if `false`, the point will be ignored.
    pub wrap_x: bool,
    /// If true, y values outside image will be wrapped around. If `false`, the point will be ignored.
    pub wrap_y: bool,
    /// Whether y values will be inverted when plotting. Value of `true` will put origin on bottom left.
    pub invert_y: bool,
    /// Image data.
    /// 
    /// Image data is a 2D array. However here we use a 1D array to represent it, because Vec in Vec in Rust isn't so great. [`index`] is
    /// used to get the appropriate index of `data` based on a (x, y) coordinate. 
    /// 
    /// [`index`]: #method.index
    data: Vec<Rgb>,
    /// Z-buffer (depth buffer).
    #[allow(dead_code)]
    zbuf: Vec<f64>,
}

impl Ppm {
    /// Make a 500x500 [`Ppm`], with `color_depth` of 255 (default image configuration for class).
    ///
    /// Use [`PpmBuilder`] for other configurations.
    ///
    /// # Examples
    ///
    /// ```
    /// use graphics::prelude::*;
    ///
    /// let mut img = Ppm::new();
    ///
    /// assert_eq!(500, img.width());
    /// assert_eq!(500, img.height());
    /// ```
    ///
    /// [`Ppm`]: ./struct.Ppm.html
    /// [`PpmBuilder`]: ./struct.PpmBuilder.html
    pub fn new() -> Self {
        PpmBuilder::new(500, 500, 255).build()
    }

    /// Returns `Some(index)` if `index` is in the bounds of `self.data`. Otherwise `None`.
    ///
    /// `None` is useful because you might want to ignore points that are outside of the visible space. Note that there's no `null` in Rust,
    /// so this requires you to handle all cases.
    ///
    /// This method is used in the `plot` function for [`Screen` impl].
    ///
    /// [`Screen` impl]: #impl-Screen
    fn index(&self, x: i64, y: i64) -> Option<usize> {
        let (width, height) = (
            i64::try_from(self.width).unwrap(),
            i64::try_from(self.height).unwrap(),
        );

        if (!self.wrap_x && (x < 0 || x >= width)) || (!self.wrap_y && (y < 0 || y >= height)) {
            // wrapping is disabled AND (x or y is out of bounds)
            return None;
        }

        let x = wrap_index(x, width);
        let y = wrap_index(y, height);

        // invert y based on config
        let y = if self.invert_y {
            self.width as i64 - y - 1
        } else {
            y
        };

        // now we know that x and y are positive, we can cast without worry
        usize::try_from(y * self.width as i64 + x).ok()
    }

    /// Write ppm in binary format to the given `writer` without buffering.
    ///
    /// This is useful for writing image data to memory or if the `writer` is already buffered.
    pub fn write_binary_nobuffer<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writeln!(writer, "P6")?;
        writeln!(
            writer,
            "{} {} {}",
            self.width, self.height, self.color_depth
        )?;
        if self.color_depth < 256 {
            for t in self.data.iter() {
                writer.write_all(&[t.red, t.green, t.blue])?;
            }
        } else {
            for t in self.data.iter() {
                // content is in big endian, per ppm spec
                writer.write_all(&(t.red.to_be_bytes()))?;
                writer.write_all(&(t.green.to_be_bytes()))?;
                writer.write_all(&(t.blue.to_be_bytes()))?;
            }
        }
        writer.flush()?;
        Ok(())
    }

    /// Write ppm in binary format to the given `writer`.
    ///
    /// The `writer` will be buffered before being written to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use graphics::prelude::*;
    /// let mut img = Ppm::new();
    ///     
    /// let xmax = img.width() as f64;
    /// let ymax = img.height() as f64;
    /// let color = Rgb::WHITE;
    ///
    /// img.draw_line((xmax - 1., ymax - 1., 0.), (xmax / 2., 0., 0.), color);
    ///
    /// img.write_binary_to_buf(&mut File::create("binary.ppm").expect("error creating binary.ppm"))
    ///     .expect("error writing to file");
    /// ```
    ///
    /// Or, if you use `?` to handle errors:
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io;
    /// use graphics::prelude::*;
    ///
    /// fn main() -> io::Result<()> { // note that the return type has changed
    ///     let mut img = Ppm::new();
    ///     
    ///     let xmax = img.width() as f64;
    ///     let ymax = img.height() as f64;
    ///     let color = Rgb::WHITE;
    ///
    ///     img.draw_line((xmax - 1., ymax - 1., 0.), (xmax / 2., 0., 0.), color);
    ///     
    ///     img.write_binary_to_buf(&mut File::create("binary.ppm")?)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn write_binary_to_buf<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        self.write_binary_nobuffer(&mut BufWriter::new(writer))
    }

    /// Write ppm in ascii format to the given `writer`.
    ///
    /// The `writer` will be buffered before being written to.
    ///
    /// See [`write_binary_to_buf`] for examples.
    ///
    /// [`write_binary_to_buf`]: #method.write_binary_to_buf
    pub fn write_ascii_to_buf<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        let mut buf = BufWriter::new(writer);
        writeln!(buf, "P3")?;
        writeln!(buf, "{} {} {}", self.width, self.height, self.color_depth)?;
        for t in self.data.iter() {
            writeln!(buf, "{} {} {}", t.red, t.green, t.blue)?;
        }
        buf.flush()?;
        Ok(())
    }
}

/// Wraps an `index` to be an i64 in [0, index). Used in [`Ppm`]'s [`index`] method.
///
/// [`Ppm`]: ./struct.Ppm.html
/// [`index`]: ./struct.Ppm.html#method.index
fn wrap_index(value: i64, limit: i64) -> i64 {
    ((value % limit) + limit) % limit
}

impl Screen for Ppm {
    /// Plot a point on this PPMImg at (`x`, `y`, `z`).
    ///
    /// `z` is used for depth-buffer. Will only plot if `z` if `z` > existing `z` in buffer.
    fn plot(&mut self, x: i64, y: i64, z: f64, color: Rgb) {
        if let Some(index) = self.index(x, y) {
            if self.zbuf[index] < z {
                self.data[index] = color;
                self.zbuf[index] = z;
            }
        }
    }

    fn save(&self, file_path: &str) -> io::Result<()> {
        let mut cmd = magick::pipe_to_magick(&vec!["ppm:-", file_path])?;

        // This command should have a stdnin, so it's ok to unwrap
        let mut stdin = cmd.stdin.take().unwrap();
        self.write_binary_to_buf(&mut stdin)?;

        drop(stdin);

        let status = cmd.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Magick exited with non-zero status",
            ))
        }
    }

    fn write_to_buf<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        self.write_binary_nobuffer(writer)
    }

    fn display(&self) -> io::Result<()> {
        magick::display_ppm(self)
    }

    fn clear(&mut self, color: Rgb) {
        for d in self.data.iter_mut() {
            *d = color;
        }
        self.zbuf = vec![f64::NEG_INFINITY; self.height * self.width];
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn old_wrap_index(value: i64, limit: i64) -> i64 {
        if value >= limit {
            value % limit
        } else if value < 0 {
            let r = value % limit;
            if r != 0 {
                r + limit
            } else {
                r
            }
        } else {
            value
        }
    }
    #[test]
    #[ignore]
    fn test_wrap_index() {
        let cases = vec![
            (1, 2),
            (4, 5),
            (-2, 5),
            (-5, 10),
            (5, 5),
            (-5, 5),
            (i64::MIN, 2142432422),
        ];
        for (value, limit) in cases {
            assert_eq!(old_wrap_index(value, limit), wrap_index(value, limit))
        }
    }
}
