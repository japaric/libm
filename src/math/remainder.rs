#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn remainder(x: f64, y: f64) -> (f64, i32) {
    super::remquo(x, y)
}
