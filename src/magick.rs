//! Functions to help run ImageMagick commands as subprocesses.

use std::{
    fs::{self, File},
    io,
    process::Command,
    process::{Child, Stdio},
};

use crate::ppm::Ppm;

/// Subprocess (and run) `(magick) convert` with a piped stdin with the given `args`.
///
/// This function will be very useful later on when we deal with animations
/// (by piping all the image data to ImageMagick and letting it make a gif out of it).
pub fn pipe_to_magick(args: &[&str]) -> io::Result<Child> {
    Command::new(if cfg!(windows) { "magick" } else { "convert" })
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
}

/// Write Ppm to a temporary file and display with ImageMagick.
///
/// This method requires ImageMagick `display` to be installed as `imdisplay` on Windows and `display` on *nix.
pub(crate) fn display_ppm(img: &Ppm) -> io::Result<()> {
    let tmpfile_name = "tmp.ppm";

    img.write_binary_to_buf(&mut File::create(tmpfile_name)?)?;

    let mut cmd = Command::new(if cfg!(windows) {
        "imdisplay"
    } else {
        "display"
    });

    let mut display = cmd
        // .arg("-flip")
        .arg(tmpfile_name)
        .spawn()
        .unwrap();

    let _result = display.wait().unwrap();
    fs::remove_file(tmpfile_name)?;

    Ok(())
}
