/// Fast f64 to int floor function.
#[inline]
pub fn fast_floor(x: f64) -> int {
    if x > 0.0 {
        x.to_int().unwrap()
    } else {
        (x.to_int().unwrap()) - 1
    }
}
