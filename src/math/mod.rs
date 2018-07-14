macro_rules! force_eval {
    ($e:expr) => {
        unsafe {
            ::core::ptr::read_volatile(&$e);
        }
    };
}

mod atanf;
mod ceilf;
mod cosf;
mod expf;
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
mod sqrt;
mod sqrtf;
mod trunc;
mod truncf;

pub use self::{
    atanf::atanf, ceilf::ceilf, cosf::cosf, expf::expf, fabs::fabs, fabsf::fabsf, floor::floor,
    floorf::floorf, fmodf::fmodf, hypot::hypot, hypotf::hypotf, log::log, log10::log10,
    log10f::log10f, log1p::log1p, log1pf::log1pf, log2::log2, log2f::log2f, logf::logf, powf::powf,
    round::round, roundf::roundf, scalbn::scalbn, scalbnf::scalbnf, sqrt::sqrt, sqrtf::sqrtf,
    trunc::trunc, truncf::truncf,
};

mod k_cosf;
mod k_sinf;
mod rem_pio2_large;
mod rem_pio2f;

use self::{k_cosf::k_cosf, k_sinf::k_sinf, rem_pio2_large::rem_pio2_large, rem_pio2f::rem_pio2f};

fn isnanf(x: f32) -> bool {
    x.to_bits() & 0x7fffffff > 0x7f800000
}
