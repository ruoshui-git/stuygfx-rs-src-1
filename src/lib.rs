//! # Graphics ❤️ Rust, Even Just for Class
//!
//! Welcome!
//!
//! So now you'll be using [Rust] for your Graphics (mks66) class. Rust is a fascinating language with interesting features. It is very
//! fast and eliminates a lot of unexpected errors at compile time. A lot of times the code you write will "just run"
//! (especially if you are just coming from systems (mks65) and got tired of C's segmentation faults.)
//! What is left for you to do is to take care of your logical stuff.
//!
//! Rust also has very great tooling! You have probably generated this document yourself, thanks to Cargo. Cargo will also manage all your
//! compilation and all the libraries you'll ever need.
//!
//! But Rust isn't only roses and flowers. You'll potentially need to invest more of your time into learning about the language than
//! just opting for one that you're familiar with. But part of the point of this template is to reduce that friction.
//!
//! Some people criticize Rust for long compile times. That is probably very true, but you won't really feel until towards the end of this course,
//! when using a parser library is required.
//!
//! Rust has the ["unsafe"] feature, but you shouldn't need any of that for this class.
//!
//! You should know some Rust basics by now. It would be great if you can (or have) read through [the book]. But in case you don't have the time,
//! [Rust By Example] should also get you started. Various topics that show up might be addressed in this doc too, and link you to the
//! appropriate resources.
//!
//!
//! # ImageMagick
//! You should have ImageMagick installed on your computer as a command.
//!
//! On *nix, they are `convert` and `display`.
//! On Windows, they are `magick` and `imdisplay`.
//!
//! Both of these commands should be available in the shell that you run your Rust program.
//! For Windows, if your ImageMagick commands are invoked by a different set of names, you should set the program to use
//! the appropriate values in the [`magick`] module (magick.rs file).
//!
//!
//! # Structure Overview
//! Refer to the [module chapter] of the [the book] to learn more about Rust's module system.
//!
//! This template is structured as both a library crate and a binary crate. In your "main.rs", you can use the library's code with the name
//! `graphics`. The "main.rs" file right now includes an example of how you can do that.
//!
//! ## (Picture) Graphics Engine
//! The provided C templates for the class use just functions and arrays to model your engine. Same in the Python templates, even though
//! you have access to a lot more constructs, like classes. But here you'll use Rust structs and enums and implement methods and them, kind of
//! like defining your own classes and methods in Python.
//!
//! To begin drawing, you shoud do these things
//! 1. Import [`prelude::*`] from `graphics`.
//! 2. Construct a [`Ppm`] through [`Ppm::new()`] or [`PpmBuilder`].
//! 3. Call various drawing functions on [`Ppm`].
//!     - Most of those functions are actually (or will be) implemented on the [`Screen`] trait, and [`Ppm`] implements the trait, so you can use them
//!         on [`Ppm`] when [`Screen`] is in scope. Importing from [`prelude`] gives you all of that as a convenience.
//! 4. Call [`display`] or [`save`] on your [`Ppm`] to see the final image. (Provided as part of [`Screen`] trait)
//!     - Alternatively, you can call [`write_ascii_to_buf`] or [`write_binary_to_buf`] on [`Ppm`] and pass in a [`Write`]r.
//!         - The [`Write`]r passed will be wrapped in a [`BufWriter`], so you should pass in a [`File`] directly.
//!         - There is a variant that does not buffer the output. Check the [`Ppm`] struct for more information.
//!
//! - Eventually, we will have other ways of doing things, but for now, this is the process.
//!
//!
//!
//! # Get Ready
//! You should be comfortable poking around this documentation. Reading docs is an invaluable skill everyone should have in programming.
//!
//! Look at main.rs to see how to draw stuff. Or go straight into work 1.
//!
//! # Work 1
//! Head over to "screen.rs", ([`screen`] module), and implement the [`draw_line`] method. Read the docs for more info.
//!
//! After you implement that, run main.rs to see an example. For your creative work (gallery submission), feel free to explore
//! other functionalities ([`draw_line_degrees`] and [`Turtle`]) provided to you for free! They are not part of the class material, but
//! they depend on a correct implementation of `draw_line`.
//!
//! # Running Binary
//! If you haven't figured out by now, `cargo run` will compile and run "main.rs". Subsequent `cargo run` won't compile again
//! if you don't modify any of your source files.
//!
//! Use `cargo run --release` to run the optimized binary. Optimization isn't necessary now, but it will make the program a lot faster later on.
//!
//! [module chapter]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
//! [Rust]: https://www.rust-lang.org/
//! [the book]: https://doc.rust-lang.org/book/
//! [Rust By Example]: https://doc.rust-lang.org/stable/rust-by-example/
//! ["unsafe"]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
//! [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
//! [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
//! [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [`prelude`]: ./prelude/index.html
//! [`prelude::*`]: ./prelude/index.html
//! [`Ppm`]: ./ppm/struct.Ppm.html
//! [`Ppm::new()`]: ./ppm/struct.Ppm.html#method.new
//! [`PpmBuilder`]: ./ppm/struct.PpmBuilder.html
//! [`Screen`]: ./screen/trait.Screen.html
//! [`screen`]: ./screen/index.html
//! [`draw_line`]: ./screen/trait.Screen.html#method.draw_line
//! [`display`]: ./screen/trait.Screen.html#tymethod.display
//! [`save`]: ./screen/trait.Screen.html#tymethod.save
//! [`write_ascii_to_buf`]: ./ppm/struct.Ppm.html#method.write_ascii_to_buf
//! [`write_binary_to_buf`]: ./ppm/struct.Ppm.html#method.write_binary_to_buf
//! [`magick`]: ./magick/index.html
//! [`draw_line_degrees`]: ./screen/trait.Screen.html#method.draw_line_degrees
//! [`Turtle`]: ./turtle/struct.Turtle.html

pub mod color;
pub mod magick;
pub mod ppm;
pub mod screen;
pub mod turtle;
pub mod util;

/// Re-export common structs and traits for drawing.
pub mod prelude {
    pub use crate::{
        color::Rgb,
        ppm::{Ppm, PpmBuilder},
        screen::Screen,
    };
}
