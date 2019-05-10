/* origin: FreeBSD /usr/src/lib/msun/src/e_j1.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* j1(x), y1(x)
 * Bessel function of the first and second kinds of order zero.
 * Method -- j1(x):
 *      1. For tiny x, we use j1(x) = x/2 - x^3/16 + x^5/384 - ...
 *      2. Reduce x to |x| since j1(x)=-j1(-x),  and
 *         for x in (0,2)
 *              j1(x) = x/2 + x*z*R0/S0,  where z = x*x;
 *         (precision:  |j1/x - 1/2 - R0/S0 |<2**-61.51 )
 *         for x in (2,inf)
 *              j1(x) = sqrt(2/(pi*x))*(p1(x)*cos(x1)-q1(x)*sin(x1))
 *              y1(x) = sqrt(2/(pi*x))*(p1(x)*sin(x1)+q1(x)*cos(x1))
 *         where x1 = x-3*pi/4. It is better to compute sin(x1),cos(x1)
 *         as follow:
 *              cos(x1) =  cos(x)cos(3pi/4)+sin(x)sin(3pi/4)
 *                      =  1/sqrt(2) * (sin(x) - cos(x))
 *              sin(x1) =  sin(x)cos(3pi/4)-cos(x)sin(3pi/4)
 *                      = -1/sqrt(2) * (sin(x) + cos(x))
 *         (To avoid cancellation, use
 *              sin(x) +- cos(x) = -cos(2x)/(sin(x) -+ cos(x))
 *          to compute the worse one.)
 *
 *      3 Special cases
 *              j1(nan)= nan
 *              j1(0) = 0
 *              j1(inf) = 0
 *
 * Method -- y1(x):
 *      1. screen out x<=0 cases: y1(0)=-inf, y1(x<0)=NaN
 *      2. For x<2.
 *         Since
 *              y1(x) = 2/pi*(j1(x)*(ln(x/2)+Euler)-1/x-x/2+5/64*x^3-...)
 *         therefore y1(x)-2/pi*j1(x)*ln(x)-1/x is an odd function.
 *         We use the following function to approximate y1,
 *              y1(x) = x*U(z)/V(z) + (2/pi)*(j1(x)*ln(x)-1/x), z= x^2
 *         where for x in [0,2] (abs err less than 2**-65.89)
 *              U(z) = U0[0] + U0[1]*z + ... + U0[4]*z^4
 *              V(z) = 1  + v0[0]*z + ... + v0[4]*z^5
 *         Note: For tiny x, 1/x dominate y1 and hence
 *              y1(tiny) = -2/pi/tiny, (choose tiny<2**-54)
 *      3. For x>=2.
 *              y1(x) = sqrt(2/(pi*x))*(p1(x)*sin(x1)+q1(x)*cos(x1))
 *         where x1 = x-3*pi/4. It is better to compute sin(x1),cos(x1)
 *         by method mentioned above.
 */

use super::{cos, fabs, get_high_word, get_low_word, log, sin, sqrt};
use crate::math::consts::*;
use core::f64;

const INVSQRTPI: f64 = 5.641_895_835_477_562_792_80_e-01; /* 0x_3FE2_0DD7, 0x_5042_9B6D */
const TPI: f64 = 6.366_197_723_675_813_824_33_e-01; /* 0x_3FE4_5F30, 0x_6DC9_C883 */

fn common(ix: u32, x: f64, y1: bool, sign: bool) -> f64 {
    let z: f64;
    let mut s: f64;
    let c: f64;
    let mut ss: f64;
    let mut cc: f64;

    /*
     * j1(x) = sqrt(2/(pi*x))*(p1(x)*cos(x-3pi/4)-q1(x)*sin(x-3pi/4))
     * y1(x) = sqrt(2/(pi*x))*(p1(x)*sin(x-3pi/4)+q1(x)*cos(x-3pi/4))
     *
     * sin(x-3pi/4) = -(sin(x) + cos(x))/sqrt(2)
     * cos(x-3pi/4) = (sin(x) - cos(x))/sqrt(2)
     * sin(x) +- cos(x) = -cos(2x)/(sin(x) -+ cos(x))
     */
    s = sin(x);
    if y1 {
        s = -s;
    }
    c = cos(x);
    cc = s - c;
    if ix < 0x_7fe0_0000 {
        /* avoid overflow in 2*x */
        ss = -s - c;
        z = cos(2. * x);
        if s * c > 0. {
            cc = z / ss;
        } else {
            ss = z / cc;
        }
        if ix < 0x_4800_0000 {
            if y1 {
                ss = -ss;
            }
            cc = pone(x) * cc - qone(x) * ss;
        }
    }
    if sign {
        cc = -cc;
    }
    INVSQRTPI * cc / sqrt(x)
}

/* R0/S0 on [0,2] */
const R00: f64 = -6.250_000_000_000_000_000_00_e-02; /* 0x_BFB0_0000, 0x_0000_0000 */
const R01: f64 = 1.407_056_669_551_897_060_48_e-03; /* 0x_3F57_0D9F, 0x_9847_2C61 */
const R02: f64 = -1.599_556_310_840_355_975_20_e-05; /* 0x_BEF0_C5C6, 0x_BA16_9668 */
const R03: f64 = 4.967_279_996_095_844_484_12_e-08; /* 0x_3E6A_AAFA, 0x_46CA_0BD9 */
const S01: f64 = 1.915_375_995_383_634_608_05_e-02; /* 0x_3F93_9D0B, 0x_1263_7E53 */
const S02: f64 = 1.859_467_855_886_309_155_60_e-04; /* 0x_3F28_5F56, 0x_B9CD_F664 */
const S03: f64 = 1.177_184_640_426_236_832_63_e-06; /* 0x_3EB3_BFF8, 0x_333F_8498 */
const S04: f64 = 5.046_362_570_762_170_427_15_e-09; /* 0x_3E35_AC88, 0x_C97D_FF2C */
const S05: f64 = 1.235_422_744_261_379_139_08_e-11; /* 0x_3DAB_2ACF, 0x_CFB9_7ED8 */

pub fn j1(x: f64) -> f64 {
    let mut z: f64;
    let r: f64;
    let s: f64;
    let mut ix: u32;
    let sign: bool;

    ix = get_high_word(x);
    sign = (ix >> 31) != 0;
    ix &= UF_ABS;
    if ix >= 0x_7ff0_0000 {
        return 1. / (x * x);
    }
    if ix >= 0x_4000_0000 {
        /* |x| >= 2 */
        return common(ix, fabs(x), false, sign);
    }
    if ix >= 0x_3800_0000 {
        /* |x| >= 2**-127 */
        z = x * x;
        r = z * (R00 + z * (R01 + z * (R02 + z * R03)));
        s = 1. + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05))));
        z = r / s;
    } else {
        /* avoid underflow, raise inexact if x!=0 */
        z = x;
    }
    (0.5 + z) * x
}

const U0: [f64; 5] = [
    -1.960_570_906_462_389_406_68_e-01, /* 0x_BFC9_1866, 0x_143C_BC8A */
    5.044_387_166_398_112_826_16_e-02,  /* 0x_3FA9_D3C7, 0x_7629_2CD1 */
    -1.912_568_958_757_635_472_98_e-03, /* 0x_BF5F_55E5, 0x_4844_F50F */
    2.352_526_005_616_104_959_28_e-05,  /* 0x_3EF8_AB03, 0x_8FA6_B88E */
    -9.190_991_580_398_788_745_04_e-08, /* 0x_BE78_AC00, 0x_5691_05B8 */
];
const V0: [f64; 5] = [
    1.991_673_182_366_499_039_73_e-02, /* 0x_3F94_650D, 0x_3F4D_A9F0 */
    2.025_525_810_251_351_714_96_e-04, /* 0x_3F2A_8C89, 0x_6C25_7764 */
    1.356_088_010_975_162_294_04_e-06, /* 0x_3EB6_C05A, 0x_894E_8CA6 */
    6.227_414_523_646_215_012_95_e-09, /* 0x_3E3A_BF1D, 0x_5BA6_9A86 */
    1.665_592_462_079_920_791_14_e-11, /* 0x_3DB2_5039, 0x_DACA_772A */
];

pub fn y1(x: f64) -> f64 {
    let z: f64;
    let u: f64;
    let v: f64;
    let ix: u32;
    let lx: u32;

    ix = get_high_word(x);
    lx = get_low_word(x);

    /* y1(nan)=nan, y1(<0)=nan, y1(0)=-inf, y1(inf)=0 */
    if (ix << 1 | lx) == 0 {
        return f64::NEG_INFINITY;
    }
    if (ix >> 31) != 0 {
        return f64::NAN;
    }
    if ix >= 0x_7ff0_0000 {
        return 1. / x;
    }

    if ix >= 0x_4000_0000 {
        /* x >= 2 */
        return common(ix, x, true, false);
    }
    if ix < 0x_3c90_0000 {
        /* x < 2**-54 */
        return -TPI / x;
    }
    z = x * x;
    u = U0[0] + z * (U0[1] + z * (U0[2] + z * (U0[3] + z * U0[4])));
    v = 1. + z * (V0[0] + z * (V0[1] + z * (V0[2] + z * (V0[3] + z * V0[4]))));
    x * (u / v) + TPI * (j1(x) * log(x) - 1. / x)
}

/* For x >= 8, the asymptotic expansions of pone is
 *      1 + 15/128 s^2 - 4725/2^15 s^4 - ...,   where s = 1/x.
 * We approximate pone by
 *      pone(x) = 1 + (R/S)
 * where  R = pr0 + pr1*s^2 + pr2*s^4 + ... + pr5*s^10
 *        S = 1 + ps0*s^2 + ... + ps4*s^10
 * and
 *      | pone(x)-1-R/S | <= 2  ** ( -60.06)
 */

const PR8: [f64; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                                /* 0x_0000_0000, 0x_0000_0000 */
    1.171_874_999_999_886_479_70_e-01, /* 0x_3FBD_FFFF, 0x_FFFF_FCCE */
    1.323_948_065_930_735_751_29_e+01, /* 0x_402A_7A9D, 0x_357F_7FCE */
    4.120_518_543_073_785_622_25_e+02, /* 0x_4079_C0D4, 0x_652E_A590 */
    3.874_745_389_139_605_322_27_e+03, /* 0x_40AE_457D, 0x_A3A5_32CC */
    7.914_479_540_318_917_315_74_e+03, /* 0x_40BE_EA7A, 0x_C327_82DD */
];
const PS8: [f64; 5] = [
    1.142_073_703_756_784_084_36_e+02, /* 0x_405C_8D45, 0x_8E65_6CAC */
    3.650_930_834_208_534_633_94_e+03, /* 0x_40AC_85DC, 0x_964D_274F */
    3.695_620_602_690_334_635_55_e+04, /* 0x_40E2_0B86, 0x_97C5_BB7F */
    9.760_279_359_349_508_013_11_e+04, /* 0x_40F7_D42C, 0x_B28F_17BB */
    3.080_427_206_278_888_115_78_e+04, /* 0x_40DE_1511, 0x_697A_0B2D */
];

const PR5: [f64; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    1.319_905_195_562_435_227_49_e-11, /* 0x_3DAD_0667, 0x_DAE1_CA7D */
    1.171_874_931_906_140_976_38_e-01, /* 0x_3FBD_FFFF, 0x_E2C1_0043 */
    6.802_751_278_684_328_717_36,      /* 0x_401B_3604, 0x_6E63_15E3 */
    1.083_081_829_901_891_097_73_e+02, /* 0x_405B_13B9, 0x_4526_02ED */
    5.176_361_395_331_997_528_05_e+02, /* 0x_4080_2D16, 0x_D052_D649 */
    5.287_152_013_633_375_418_07_e+02, /* 0x_4080_85B8, 0x_BB7E_0CB7 */
];
const PS5: [f64; 5] = [
    5.928_059_872_211_313_319_21_e+01, /* 0x_404D_A3EA, 0x_A8AF_633D */
    9.914_014_187_336_143_777_43_e+02, /* 0x_408E_FB36, 0x_1B06_6701 */
    5.353_266_952_914_879_766_47_e+03, /* 0x_40B4_E944, 0x_5706_B6FB */
    7.844_690_317_495_512_317_69_e+03, /* 0x_40BE_A4B0, 0x_B8A5_BB15 */
    1.504_046_888_103_610_626_79_e+03, /* 0x_4097_8030, 0x_036F_5E51 */
];

const PR3: [f64; 6] = [
    3.025_039_161_373_736_180_24_e-09, /* 0x_3E29_FC21, 0x_A7AD_9EDD */
    1.171_868_655_672_535_924_91_e-01, /* 0x_3FBD_FFF5, 0x_5B21_D17B */
    3.932_977_500_333_156_406_5,       /* 0x_400F_76BC, 0x_E85E_AD8A */
    3.511_940_355_916_369_327_36_e+01, /* 0x_4041_8F48, 0x_9DA6_D129 */
    9.105_501_107_507_812_719_18_e+01, /* 0x_4056_C385, 0x_4D2C_1837 */
    4.855_906_851_973_649_196_45_e+01, /* 0x_4048_478F, 0x_8EA8_3EE5 */
];
const PS3: [f64; 5] = [
    3.479_130_950_012_515_199_89_e+01, /* 0x_4041_6549, 0x_A134_069C */
    3.367_624_587_478_257_467_41_e+02, /* 0x_4075_0C33, 0x_07F1_A75F */
    1.046_871_399_757_751_305_51_e+03, /* 0x_4090_5B7C, 0x_5037_D523 */
    8.908_113_463_982_564_326_22_e+02, /* 0x_408B_D67D, 0x_A32E_31E9 */
    1.037_879_324_396_392_775_04_e+02, /* 0x_4059_F26D, 0x_7C2E_ED53 */
];

const PR2: [f64; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    1.077_108_301_068_737_430_82_e-07, /* 0x_3E7C_E9D4, 0x_F655_44F4 */
    1.171_762_194_626_833_480_94_e-01, /* 0x_3FBD_FF42, 0x_BE76_0D83 */
    2.368_514_966_676_087_851_74,      /* 0x_4002_F2B7, 0x_F98F_AEC0 */
    1.224_261_091_482_612_329_17_e+01, /* 0x_4028_7C37, 0x_7F71_A964 */
    1.769_397_112_716_877_273_90_e+01, /* 0x_4031_B1A8, 0x_177F_8EE2 */
    5.073_523_125_888_184_992_5,       /* 0x_4014_4B49, 0x_A574_C1FE */
];
const PS2: [f64; 5] = [
    2.143_648_593_638_214_094_88_e+01, /* 0x_4035_6FBD, 0x_8AD5_ECDC */
    1.252_902_271_684_027_510_90_e+02, /* 0x_405F_5293, 0x_14F9_2CD5 */
    2.322_764_690_571_628_136_69_e+02, /* 0x_406D_08D8, 0x_D5A2_DBD9 */
    1.176_793_732_871_471_007_68_e+02, /* 0x_405D_6B7A, 0x_DA18_84A9 */
    8.364_638_933_716_182_833_68,      /* 0x_4020_BAB1, 0x_F44E_5192 */
];

fn pone(x: f64) -> f64 {
    let p: &[f64; 6];
    let q: &[f64; 5];
    let z: f64;
    let r: f64;
    let s: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;
    if ix >= 0x_4020_0000 {
        p = &PR8;
        q = &PS8;
    } else if ix >= 0x_4012_2E8B {
        p = &PR5;
        q = &PS5;
    } else if ix >= 0x_4006_DB6D {
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

/* For x >= 8, the asymptotic expansions of qone is
 *      3/8 s - 105/1024 s^3 - ..., where s = 1/x.
 * We approximate pone by
 *      qone(x) = s*(0.375 + (R/S))
 * where  R = qr1*s^2 + qr2*s^4 + ... + qr5*s^10
 *        S = 1 + qs1*s^2 + ... + qs6*s^12
 * and
 *      | qone(x)/s -0.375-R/S | <= 2  ** ( -61.13)
 */

const QR8: [f64; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                                 /* 0x_0000_0000, 0x_0000_0000 */
    -1.025_390_624_999_927_141_61_e-01, /* 0x_BFBA_3FFF, 0x_FFFF_FDF3 */
    -1.627_175_345_445_899_878_88_e+01, /* 0x_C030_4591, 0x_A267_79F7 */
    -7.596_017_225_139_501_078_96_e+02, /* 0x_C087_BCD0, 0x_53E4_B576 */
    -1.184_980_667_024_295_871_67_e+04, /* 0x_C0C7_24E7, 0x_40F8_7415 */
    -4.843_851_242_857_503_530_10_e+04, /* 0x_C0E7_A6D0, 0x_65D0_9C6A */
];
const QS8: [f64; 6] = [
    1.613_953_697_007_229_095_56_e+02,  /* 0x_4064_2CA6, 0x_DE5B_CDE5 */
    7.825_385_999_233_484_653_81_e+03,  /* 0x_40BE_9162, 0x_D0D8_8419 */
    1.338_753_362_872_495_781_63_e+05,  /* 0x_4100_579A, 0x_B0B7_5E98 */
    7.196_577_236_832_409_398_63_e+05,  /* 0x_4125_F653, 0x_7286_9C19 */
    6.666_012_326_177_763_752_64_e+05,  /* 0x_4124_57D2, 0x_7719_AD5C */
    -2.944_902_643_038_346_432_15_e+05, /* 0x_C111_F969, 0x_0EA5_AA18 */
];

const QR5: [f64; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    -2.089_799_311_417_641_042_97_e-11, /* 0x_BDB6_FA43, 0x_1AA1_A098 */
    -1.025_390_502_413_754_262_31_e-01, /* 0x_BFBA_3FFF, 0x_CB59_7FEF */
    -8.056_448_281_239_360_298_4,       /* 0x_C020_1CE6, 0x_CA03_AD4B */
    -1.836_696_074_748_883_802_39_e+02, /* 0x_C066_F56D, 0x_6CA7_B9B0 */
    -1.373_193_760_655_081_632_65_e+03, /* 0x_C095_74C6, 0x_6931_734F */
    -2.612_444_404_532_156_568_17_e+03, /* 0x_C0A4_68E3, 0x_88FD_A79D */
];
const QS5: [f64; 6] = [
    8.127_655_013_843_357_778_57_e+01,  /* 0x_4054_51B2, 0x_FF5A_11B2 */
    1.991_798_734_604_859_646_42_e+03,  /* 0x_409F_1F31, 0x_E77B_F839 */
    1.746_848_519_249_089_076_77_e+04,  /* 0x_40D1_0F1F, 0x_0D64_CE29 */
    4.985_142_709_103_522_793_16_e+04,  /* 0x_40E8_576D, 0x_AABA_D197 */
    2.794_807_516_389_181_182_60_e+04,  /* 0x_40DB_4B04, 0x_CF7C_364B */
    -4.719_183_547_951_284_708_69_e+03, /* 0x_C0B2_6F2E, 0x_FCFF_A004 */
];

const QR3: [f64; 6] = [
    -5.078_312_264_617_665_613_69_e-09, /* 0x_BE35_CFA9, 0x_D38F_C84F */
    -1.025_378_298_208_370_897_45_e-01, /* 0x_BFBA_3FEB, 0x_51AE_ED54 */
    -4.610_115_811_394_734_031_13,      /* 0x_C012_70C2, 0x_3302_D9FF */
    -5.784_722_165_627_836_432_12_e+01, /* 0x_C04C_EC71, 0x_C25D_16DA */
    -2.282_445_407_376_316_950_38_e+02, /* 0x_C06C_87D3, 0x_4718_D55F */
    -2.192_101_284_789_093_256_22_e+02, /* 0x_C06B_66B9, 0x_5F5C_1BF6 */
];
const QS3: [f64; 6] = [
    4.766_515_503_237_295_092_73_e+01,  /* 0x_4047_D523, 0x_CCD3_67E4 */
    6.738_651_126_766_997_094_82_e+02,  /* 0x_4085_0EEB, 0x_C031_EE3E */
    3.380_152_866_795_263_435_05_e+03,  /* 0x_40AA_684E, 0x_448E_7C9A */
    5.547_729_097_207_227_823_67_e+03,  /* 0x_40B5_ABBA, 0x_A61D_54A6 */
    1.903_119_193_388_107_987_63_e+03,  /* 0x_409D_BC7A, 0x_0DD4_DF4B */
    -1.352_011_914_443_073_408_17_e+02, /* 0x_C060_E670, 0x_290A_311F */
];

const QR2: [f64; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    -1.783_817_275_109_588_655_72_e-07, /* 0x_BE87_F126, 0x_44C6_26D2 */
    -1.025_170_426_079_855_534_60_e-01, /* 0x_BFBA_3E8E, 0x_9148_B010 */
    -2.752_205_682_781_874_607_20,      /* 0x_C006_0484, 0x_69BB_4EDA */
    -1.966_361_626_437_037_202_21_e+01, /* 0x_C033_A9E2, 0x_C168_907F */
    -4.232_531_333_728_304_900_89_e+01, /* 0x_C045_29A3, 0x_DE10_4AAA */
    -2.137_192_117_037_040_617_33_e+01, /* 0x_C035_5F36, 0x_39CF_6E52 */
];
const QS2: [f64; 6] = [
    2.953_336_290_605_238_545_48_e+01, /* 0x_403D_888A, 0x_78AE_64FF */
    2.529_815_499_821_905_291_36_e+02, /* 0x_406F_9F68, 0x_DB82_1CBA */
    7.575_028_348_686_454_364_72_e+02, /* 0x_4087_AC05, 0x_CE49_A0F7 */
    7.393_932_053_204_672_456_56_e+02, /* 0x_4087_1B25, 0x_48D4_C029 */
    1.559_490_033_366_661_236_87_e+02, /* 0x_4063_7E5E, 0x_3C3E_D8D4 */
    -4.959_498_988_226_282_101_27,     /* 0x_C013_D686, 0x_E71B_E86B */
];

fn qone(x: f64) -> f64 {
    let p: &[f64; 6];
    let q: &[f64; 6];
    let s: f64;
    let r: f64;
    let z: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;
    if ix >= 0x_4020_0000 {
        p = &QR8;
        q = &QS8;
    } else if ix >= 0x_4012_2E8B {
        p = &QR5;
        q = &QS5;
    } else if ix >= 0x_4006_DB6D {
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
    (0.375 + r / s) / x
}
