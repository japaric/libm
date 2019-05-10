/* origin: FreeBSD /usr/src/lib/msun/src/e_j0f.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

use crate::math::consts::*;

use super::{cosf, fabsf, logf, sinf, sqrtf};
use core::f32;

const INVSQRTPI: f32 = 5.641_896_128_7_e-01; /* 0x_3f10_6ebb */
const TPI: f32 = 6.366_197_466_9_e-01; /* 0x_3f22_f983 */

fn common(ix: u32, x: f32, y0: bool) -> f32 {
    let z: f32;
    let s: f32;
    let mut c: f32;
    let mut ss: f32;
    let mut cc: f32;
    /*
     * j0(x) = 1/sqrt(pi) * (P(0,x)*cc - Q(0,x)*ss) / sqrt(x)
     * y0(x) = 1/sqrt(pi) * (P(0,x)*ss + Q(0,x)*cc) / sqrt(x)
     */
    s = sinf(x);
    c = cosf(x);
    if y0 {
        c = -c;
    }
    cc = s + c;
    if ix < 0x_7f00_0000 {
        ss = s - c;
        z = -cosf(2. * x);
        if s * c < 0. {
            cc = z / ss;
        } else {
            ss = z / cc;
        }
        if ix < 0x_5880_0000 {
            if y0 {
                ss = -ss;
            }
            cc = pzerof(x) * cc - qzerof(x) * ss;
        }
    }
    INVSQRTPI * cc / sqrtf(x)
}

/* R0/S0 on [0, 2.00] */
const R02: f32 = 1.562_500_000_0_e-02; /* 0x_3c80_0000 */
const R03: f32 = -1.899_792_987_4_e-04; /* 0x_b947_352e */
const R04: f32 = 1.829_540_451_6_e-06; /* 0x_35f5_8e88 */
const R05: f32 = -4.618_327_054_1_e-09; /* 0x_b19e_af3c */
const S01: f32 = 1.561_910_286_5_e-02; /* 0x_3c7f_e744 */
const S02: f32 = 1.169_267_852_7_e-04; /* 0x_38f5_3697 */
const S03: f32 = 5.135_465_244_2_e-07; /* 0x_3509_daa6 */
const S04: f32 = 1.166_140_073_4_e-09; /* 0x_30a0_45e8 */

pub fn j0f(mut x: f32) -> f32 {
    let z: f32;
    let r: f32;
    let s: f32;
    let mut ix: u32;

    ix = x.to_bits();
    ix &= UF_ABS;
    if ix >= UF_INF {
        return 1. / (x * x);
    }
    x = fabsf(x);

    if ix >= 0x_4000_0000 {
        /* |x| >= 2 */
        /* large ulp error near zeros */
        return common(ix, x, false);
    }
    if ix >= 0x_3a00_0000 {
        /* |x| >= 2**-11 */
        /* up to 4ulp error near 2 */
        z = x * x;
        r = z * (R02 + z * (R03 + z * (R04 + z * R05)));
        s = 1. + z * (S01 + z * (S02 + z * (S03 + z * S04)));
        return (1. + x / 2.) * (1. - x / 2.) + z * (r / s);
    }
    if ix >= 0x_2180_0000 {
        /* |x| >= 2**-60 */
        x = 0.25 * x * x;
    }
    1. - x
}

const U00: f32 = -7.380_429_655_3_e-02; /* 0x_bd97_26b5 */
const U01: f32 = 1.766_664_534_8_e-01; /* 0x_3e34_e80d */
const U02: f32 = -1.381_856_761_9_e-02; /* 0x_bc62_6746 */
const U03: f32 = 3.474_534_314_6_e-04; /* 0x_39b6_2a69 */
const U04: f32 = -3.814_070_623_8_e-06; /* 0x_b67f_f53c */
const U05: f32 = 1.955_901_396_4_e-08; /* 0x_32a8_02ba */
const U06: f32 = -3.982_051_841_0_e-11; /* 0x_ae2f_21eb */
const V01: f32 = 1.273_048_389_7_e-02; /* 0x_3c50_9385 */
const V02: f32 = 7.600_686_512_9_e-05; /* 0x_389f_65e0 */
const V03: f32 = 2.591_508_518_9_e-07; /* 0x_348b_216c */
const V04: f32 = 4.411_103_149_4_e-10; /* 0x_2ff2_80c2 */

pub fn y0f(x: f32) -> f32 {
    let z: f32;
    let u: f32;
    let v: f32;
    let ix: u32;

    ix = x.to_bits();
    if ix.trailing_zeros() >= 31 {
        f32::NEG_INFINITY
    } else if (ix >> 31) != 0 {
        f32::NAN
    } else if ix >= UF_INF {
        1. / x
    } else if ix >= 0x_4000_0000 {
        /* |x| >= 2.0 */
        /* large ulp error near zeros */
        common(ix, x, true)
    } else if ix >= 0x_3900_0000 {
        /* x >= 2**-13 */
        /* large ulp error at x ~= 0.89 */
        z = x * x;
        u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06)))));
        v = 1. + z * (V01 + z * (V02 + z * (V03 + z * V04)));
        u / v + TPI * (j0f(x) * logf(x))
    } else {
        U00 + TPI * logf(x)
    }
}

/* The asymptotic expansions of pzero is
 *      1 - 9/128 s^2 + 11025/98304 s^4 - ...,  where s = 1/x.
 * For x >= 2, We approximate pzero by
 *      pzero(x) = 1 + (R/S)
 * where  R = pR0 + pR1*s^2 + pR2*s^4 + ... + pR5*s^10
 *        S = 1 + pS0*s^2 + ... + pS4*s^10
 * and
 *      | pzero(x)-1-R/S | <= 2  ** ( -60.26)
 */
const PR8: [f32; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                    /* 0x_0000_0000 */
    -7.031_250_000_0_e-02, /* 0x_bd90_0000 */
    -8.081_670_761_1,      /* 0x_c101_4e86 */
    -2.570_631_103_5_e+02, /* 0x_c380_8814 */
    -2.485_216_308_6_e+03, /* 0x_c51b_5376 */
    -5.253_043_945_3_e+03, /* 0x_c5a4_285a */
];
const PS8: [f32; 5] = [
    1.165_343_627_9_e+02, /* 0x_42e9_1198 */
    3.833_744_873_0_e+03, /* 0x_456f_9beb */
    4.059_785_546_9_e+04, /* 0x_471e_95db */
    1.167_529_687_5_e+05, /* 0x_47e4_087c */
    4.762_772_656_2_e+04, /* 0x_473a_0bba */
];
const PR5: [f32; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    -1.141_254_625_5_e-11, /* 0x_ad48_c58a */
    -7.031_249_254_9_e-02, /* 0x_bd8f_ffff */
    -4.159_610_748_3,      /* 0x_c085_1b88 */
    -6.767_476_654_1_e+01, /* 0x_c287_597b */
    -3.312_312_927_2_e+02, /* 0x_c3a5_9d9b */
    -3.464_333_801_3_e+02, /* 0x_c3ad_3779 */
];
const PS5: [f32; 5] = [
    6.075_393_676_8_e+01, /* 0x_4273_0408 */
    1.051_252_319_3_e+03, /* 0x_4483_6813 */
    5.978_970_703_1_e+03, /* 0x_45ba_d7c4 */
    9.625_445_312_5_e+03, /* 0x_4616_65c8 */
    2.406_058_105_5_e+03, /* 0x_4516_60ee */
];

const PR3: [f32; 6] = [
    /* for x in [4.547,2.8571]=1/[0.2199,0.35001] */
    -2.547_045_907_5_e-09, /* 0x_b12f_081b */
    -7.031_196_355_8_e-02, /* 0x_bd8f_ffb8 */
    -2.409_032_106_4,      /* 0x_c01a_2d95 */
    -2.196_597_671_5_e+01, /* 0x_c1af_ba52 */
    -5.807_917_022_7_e+01, /* 0x_c268_5112 */
    -3.144_794_654_8_e+01, /* 0x_c1fb_9565 */
];
const PS3: [f32; 5] = [
    3.585_603_332_5_e+01, /* 0x_420f_6c94 */
    3.615_139_770_5_e+02, /* 0x_43b4_c1ca */
    1.193_607_788_1_e+03, /* 0x_4495_3373 */
    1.127_996_826_2_e+03, /* 0x_448c_ffe6 */
    1.735_809_326_2_e+02, /* 0x_432d_94b8 */
];

const PR2: [f32; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    -8.875_343_127_1_e-08, /* 0x_b3be_98b7 */
    -7.030_309_736_7_e-02, /* 0x_bd8f_fb12 */
    -1.450_738_43,         /* 0x_bfb9_b1cc */
    -7.635_695_934_3,      /* 0x_c0f4_579f */
    -1.119_316_673_3_e+01, /* 0x_c133_1736 */
    -3.233_645_677_6,      /* 0x_c04e_f40d */
];
const PS2: [f32; 5] = [
    2.222_030_067_4_e+01, /* 0x_41b1_c32d */
    1.362_067_871_1_e+02, /* 0x_4308_34f0 */
    2.704_702_758_8_e+02, /* 0x_4387_3c32 */
    1.538_753_967_3_e+02, /* 0x_4319_e01a */
    1.465_761_756_9_e+01, /* 0x_416a_859a */
];

fn pzerof(x: f32) -> f32 {
    let p: &[f32; 6];
    let q: &[f32; 5];
    let z: f32;
    let r: f32;
    let s: f32;
    let mut ix: u32;

    ix = x.to_bits();
    ix &= UF_ABS;
    if ix >= 0x_4100_0000 {
        p = &PR8;
        q = &PS8;
    } else if ix >= 0x_4091_73eb {
        p = &PR5;
        q = &PS5;
    } else if ix >= 0x_4036_d917 {
        p = &PR3;
        q = &PS3;
    } else {
        /*ix >= 0x_4000_0000*/
        p = &PR2;
        q = &PS2;
    }
    z = 1. / (x * x);
    r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    s = 1. + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * q[4]))));
    1. + r / s
}

/* For x >= 8, the asymptotic expansions of qzero is
 *      -1/8 s + 75/1024 s^3 - ..., where s = 1/x.
 * We approximate pzero by
 *      qzero(x) = s*(-1.25 + (R/S))
 * where  R = qR0 + qR1*s^2 + qR2*s^4 + ... + qR5*s^10
 *        S = 1 + qS0*s^2 + ... + qS5*s^12
 * and
 *      | qzero(x)/s +1.25-R/S | <= 2  ** ( -61.22)
 */
const QR8: [f32; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                   /* 0x_0000_0000 */
    7.324_218_750_0_e-02, /* 0x_3d96_0000 */
    1.176_820_659_6_e+01, /* 0x_413c_4a93 */
    5.576_734_008_8_e+02, /* 0x_440b_6b19 */
    8.859_197_265_6_e+03, /* 0x_460a_6cca */
    3.701_462_500_0_e+04, /* 0x_4710_96a0 */
];
const QS8: [f32; 6] = [
    1.637_760_314_9_e+02,  /* 0x_4323_c6aa */
    8.098_344_726_6_e+03,  /* 0x_45fd_12c2 */
    1.425_382_968_8_e+05,  /* 0x_480b_3293 */
    8.033_092_500_0_e+05,  /* 0x_4944_1ed4 */
    8.405_015_625_0_e+05,  /* 0x_494d_3359 */
    -3.438_992_812_5_e+05, /* 0x_c8a7_eb69 */
];

const QR5: [f32; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    1.840_859_582_8_e-11, /* 0x_2da1_ec79 */
    7.324_218_004_9_e-02, /* 0x_3d95_ffff */
    5.835_635_185_2,      /* 0x_40ba_bd86 */
    1.351_115_722_7_e+02, /* 0x_4307_1c90 */
    1.027_243_774_4_e+03, /* 0x_4480_67cd */
    1.989_977_905_3_e+03, /* 0x_44f8_bf4b */
];
const QS5: [f32; 6] = [
    8.277_661_132_8_e+01,  /* 0x_42a5_8da0 */
    2.077_814_209_0_e+03,  /* 0x_4501_dd07 */
    1.884_728_906_2_e+04,  /* 0x_4693_3e94 */
    5.675_111_328_1_e+04,  /* 0x_475d_af1d */
    3.597_675_390_6_e+04,  /* 0x_470c_88c1 */
    -5.354_342_773_4_e+03, /* 0x_c5a7_52be */
];

const QR3: [f32; 6] = [
    /* for x in [4.547,2.8571]=1/[0.2199,0.35001] */
    4.377_409_990_0_e-09, /* 0x_3196_681b */
    7.324_111_461_6_e-02, /* 0x_3d95_ff70 */
    3.344_231_367_1,      /* 0x_4056_07e3 */
    4.262_184_524_5_e+01, /* 0x_422a_7cc5 */
    1.708_080_902_1_e+02, /* 0x_432a_cedf */
    1.667_339_477_5_e+02, /* 0x_4326_bbe4 */
];
const QS3: [f32; 6] = [
    4.875_887_298_6_e+01,  /* 0x_4243_0916 */
    7.096_892_089_8_e+02,  /* 0x_4431_6c1c */
    3.704_148_193_4_e+03,  /* 0x_4567_825f */
    6.460_425_293_0_e+03,  /* 0x_45c9_e367 */
    2.516_333_740_2_e+03,  /* 0x_451d_4557 */
    -1.492_474_517_8_e+02, /* 0x_c315_3f59 */
];

const QR2: [f32; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    1.504_444_497_9_e-07, /* 0x_3421_89db */
    7.322_342_693_8_e-02, /* 0x_3d95_f62a */
    1.998_191_714_3,      /* 0x_3fff_c4bf */
    1.449_560_260_8_e+01, /* 0x_4167_edfd */
    3.166_623_115_5_e+01, /* 0x_41fd_5471 */
    1.625_270_843_5_e+01, /* 0x_4182_058c */
];
const QS2: [f32; 6] = [
    3.036_558_532_7_e+01, /* 0x_41f2_ecb8 */
    2.693_481_140_1_e+02, /* 0x_4386_ac8f */
    8.447_837_524_4_e+02, /* 0x_4453_3229 */
    8.829_358_520_5_e+02, /* 0x_445c_bbe5 */
    2.126_663_818_4_e+02, /* 0x_4354_aa98 */
    -5.310_955_047_6,     /* 0x_c0a9_f358 */
];

fn qzerof(x: f32) -> f32 {
    let p: &[f32; 6];
    let q: &[f32; 6];
    let s: f32;
    let r: f32;
    let z: f32;
    let mut ix: u32;

    ix = x.to_bits();
    ix &= UF_ABS;
    if ix >= 0x_4100_0000 {
        p = &QR8;
        q = &QS8;
    } else if ix >= 0x_4091_73eb {
        p = &QR5;
        q = &QS5;
    } else if ix >= 0x_4036_d917 {
        p = &QR3;
        q = &QS3;
    } else {
        /*ix >= 0x_4000_0000*/
        p = &QR2;
        q = &QS2;
    }
    z = 1. / (x * x);
    r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    s = 1. + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * (q[4] + z * q[5])))));
    (-0.125 + r / s) / x
}
