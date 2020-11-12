//! Functions to help run ImageMagick commands as subprocesses

use std::{
    fs::{self, File},
    io::{self, BufWriter},
    process::Command,
    process::{Child, Stdio},
};

use crate::ppm::Ppm;

/// Subprocess (and run) `(magick) convert` with a piped stdin with the given `args`
pub fn pipe_to_magick(args: Vec<&str>) -> io::Result<Child> {
    Command::new(if cfg!(windows) { "magick" } else { "convert" })
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
}

/// Write Ppm to a temporary file and display with ImageMagick
///
/// This method requires ImageMagick `display` to be installed as `imdisplay` on Windows and `display` on *nix
pub(crate) fn display_ppm(img: &Ppm) -> io::Result<()> {
    let tmpfile_name = "tmp.ppm";

    let mut tmp_file = BufWriter::new(File::create(tmpfile_name)?);

    img.write_binary_to_buf(&mut tmp_file)?;

    drop(tmp_file);

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
