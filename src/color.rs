//! Module to work with colors.

/// Represent an rgb triple.
///
/// The fields are of type `u16` because the upper bound of color defined by PPM file spec is 2^16.
/// Since we're exclusively working with PPM files, we'll make our lives easy by using this type.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rgb {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl Rgb {
    /// This only works if the image's upper bound for color depth is `255`.
    pub const WHITE: Rgb = Rgb {
        red: 255,
        green: 255,
        blue: 255,
    };

    pub const BLACK: Rgb = Rgb {
        red: 0,
        green: 0,
        blue: 0,
    };

    /// Function that constructs an [`Rgb`] on grayscale.
    ///
    /// `depth` of `0` will give black, and the upper bound of color depth will give white.
    ///
    /// # Examples
    ///
    /// ```
    /// use graphics::prelude::*;
    /// 
    /// assert_eq!(Rgb::BLACK, Rgb::gray(0));
    /// assert_eq!(Rgb::WHITE, Rgb::gray(255));
    /// assert_eq!(Rgb {
    ///     red: 50,
    ///     green: 50,
    ///     blue: 50,
    /// }, Rgb::gray(50));
    /// 
    /// ```
    ///
    /// [`Rgb`]: ./struct.Rgb.html
    pub fn gray(depth: u16) -> Self {
        Rgb {
            red: depth,
            green: depth,
            blue: depth,
        }
    }

    /// Function constructor for [`Rgb`], probably more succinct than the struct notation.
    ///
    /// [`Rgb`]: ./struct.Rgb.html
    pub fn new(red: u16, green: u16, blue: u16) -> Self {
        Rgb { red, green, blue }
    }
}
