//! Module to work with colors.

/// Represent an rgb triple.
///
/// The fields are of type `u8`, since that's the standard RGB space. But the actual upper bound of color defined by PPM file spec is 
/// 2^16. Just change all instance of u8 here if you want that instead. You would also need to change the fns that write a PPM to file.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub const WHITE: Rgb = Rgb::gray(255);

    pub const BLACK: Rgb = Rgb::gray(0);

    /// Make an [`Rgb`] on grayscale.
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
    pub const fn gray(depth: u8) -> Self {
        Rgb {
            red: depth,
            green: depth,
            blue: depth,
        }
    }

    /// Function constructor for [`Rgb`], probably more succinct than the struct notation.
    ///
    /// [`Rgb`]: ./struct.Rgb.html
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }
}
