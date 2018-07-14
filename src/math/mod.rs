macro_rules! force_eval {
    ($e:expr) => {
        unsafe {
            ::core::ptr::read_volatile(&$e);
        }
    };
}

mod acosf;
mod asinf;
mod atan2f;
mod atanf;
mod ceil;
mod ceilf;
mod cosf;
mod coshf;
mod expf;
mod expm1f;
mod fabs;
mod fabsf;
mod floor;
mod floorf;
mod fmodf;
mod hypot;
mod hypotf;
mod log;
mod log10;
mod log10f;
mod log1p;
mod log1pf;
mod log2;
mod log2f;
mod logf;
mod powf;
mod round;
mod roundf;
mod scalbn;
mod scalbnf;
mod sinf;
mod sinhf;
mod sqrt;
mod sqrtf;
mod tanf;
mod tanhf;
mod trunc;
mod truncf;

pub use self::acosf::acosf;
pub use self::asinf::asinf;
pub use self::atan2f::atan2f;
pub use self::atanf::atanf;
pub use self::ceil::ceil;
pub use self::ceilf::ceilf;
pub use self::cosf::cosf;
pub use self::coshf::coshf;
pub use self::expf::expf;
pub use self::expm1f::expm1f;
pub use self::fabs::fabs;
pub use self::fabsf::fabsf;
pub use self::floor::floor;
pub use self::floorf::floorf;
pub use self::fmodf::fmodf;
pub use self::hypot::hypot;
pub use self::hypotf::hypotf;
pub use self::log::log;
pub use self::log10::log10;
pub use self::log10f::log10f;
pub use self::log1p::log1p;
pub use self::log1pf::log1pf;
pub use self::log2::log2;
pub use self::log2f::log2f;
pub use self::logf::logf;
pub use self::powf::powf;
pub use self::round::round;
pub use self::roundf::roundf;
pub use self::scalbn::scalbn;
pub use self::scalbnf::scalbnf;
pub use self::sinf::sinf;
pub use self::sinhf::sinhf;
pub use self::sqrt::sqrt;
pub use self::sqrtf::sqrtf;
pub use self::tanf::tanf;
pub use self::tanhf::tanhf;
pub use self::trunc::trunc;
pub use self::truncf::truncf;

mod k_cosf;
mod k_expo2f;
mod k_sinf;
mod k_tanf;
mod rem_pio2_large;
mod rem_pio2f;

use self::{
    k_cosf::k_cosf, k_expo2f::k_expo2f, k_sinf::k_sinf, k_tanf::k_tanf,
    rem_pio2_large::rem_pio2_large, rem_pio2f::rem_pio2f,
};

fn isnanf(x: f32) -> bool {
    x.to_bits() & 0x7fffffff > 0x7f800000
}
