/// A function to condense an ifelse function.
#[inline]
pub fn if_else(cond: bool, if_true: f64, if_false: f64) -> f64 {
    if cond {
        if_true
    } else {
        if_false
    }
}
