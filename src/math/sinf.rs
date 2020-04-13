/* origin: FreeBSD /usr/src/lib/msun/src/s_sinf.c */

use super::{k_cosf, k_sinf, rem_pio2f};

use core::f64::consts::FRAC_PI_2;

const S1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x3FF921FB, 0x54442D18 */
const S2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x400921FB, 0x54442D18 */
const S3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x4012D97C, 0x7F3321D2 */
const S4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x401921FB, 0x54442D18 */

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sinf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    if ix <= 0x3f490fda {
        /* |x| ~<= pi/4 */
        if ix < 0x39800000 {
            /* |x| < 2**-12 */
            force_eval!(if ix < 0x00800000 {
                x / x1p120
            } else {
                x + x1p120
            });
            return x;
        }
        return k_sinf(x64);
    }
    if ix <= 0x407b53d1 {
        /* |x| ~<= 5*pi/4 */
        if ix <= 0x4016cbe3 {
            /* |x| ~<= 3pi/4 */
            if sign {
                return -k_cosf(x64 + S1_PIO2);
            } else {
                return k_cosf(x64 - S1_PIO2);
            }
        }
        return k_sinf(if sign {
            -(x64 + S2_PIO2)
        } else {
            -(x64 - S2_PIO2)
        });
    }
    if ix <= 0x40e231d5 {
        /* |x| ~<= 9*pi/4 */
        if ix <= 0x40afeddf {
            /* |x| ~<= 7*pi/4 */
            if sign {
                return k_cosf(x64 + S3_PIO2);
            } else {
                return -k_cosf(x64 - S3_PIO2);
            }
        }
        return k_sinf(if sign { x64 + S4_PIO2 } else { x64 - S4_PIO2 });
    }

    if ix >= 0x7f800000 {
        return x - x;
    }

    let (n, y) = rem_pio2f(x);
    match n & 3 {
        0 => k_sinf(y),
        1 => k_cosf(y),
        2 => k_sinf(-y),
        _ => -k_cosf(y),
    }
}
