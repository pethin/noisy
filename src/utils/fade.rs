/// C(2) continuous interpolant
#[inline]
pub fn fade(t: f64) -> f64 {
    t * t * t * ( t * ( t * 6.0 - 15.0 ) + 10.0 )
}
