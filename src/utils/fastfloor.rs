/// Fast f64 to int floor function.
#[inline]
pub fn fastfloor(x: f64) -> int {
    if x > 0.0 {
        x as int
    } else {
        (x as int) - 1
    }
}
