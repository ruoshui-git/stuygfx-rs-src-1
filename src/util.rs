//! Misc functions that can be used across modules but don't fall into other categories.

/// Calculate `(x, y)` from `mag` and `angle_deg`
pub(crate) fn polar_to_xy(mag: f64, angle_degrees: f64) -> (f64, f64) {
    let (dy, dx) = angle_degrees.to_radians().sin_cos();
    (dx * mag, dy * mag)
}
