use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U256};

use crate::curve::{AddResult, Curve, Field, MulResult, Point};

#[derive(Debug)]
pub struct P256;

const_monty_params!(
    P256Q,
    U256,
    "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551"
);

impl Curve for P256 {
    type Limbs = [u64; 4];
    type Wide = [u64; 5];
    type Double = [u64; 8];

    type Order = ConstMontyForm<P256Q, 4>;

    const LIMB_SZ: usize = 4;
    const WIDE_SZ: usize = 5;
    const FIELD_BYTES: usize = 32;

    const P: Field<Self> = Field {
        x: [
            0xffffffffffffffff,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xffffffffffffffff,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
            0x0,
        ],
    };
    const A: Field<Self> = Field {
        x: [
            0xfffffffffffffffc,
            0x00000000ffffffff,
            0x0000000000000000,
            0xffffffff00000001,
        ],
    };
    const ZERO: Field<Self> = Field {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    const ONE: Field<Self> = Field {
        x: [0x1, 0x0, 0x0, 0x0],
    };
    const G: Point<Self> = Point {
        x: Field {
            x: [
                0xf4a13945d898c296,
                0x77037d812deb33a0,
                0xf8bce6e563a440f2,
                0x6b17d1f2e12c4247,
            ],
        },
        y: Field {
            x: [
                0xcbb6406837bf51f5,
                0x2bce33576b315ece,
                0x8ee7eb4a7c0f9e16,
                0x4fe342e2fe1a7f9b,
            ],
        },
        z: Self::ONE,
    };
    const INFINITY: Point<Self> = Point {
        x: Self::ONE,
        y: Self::ONE,
        z: Self::ZERO,
    };

    fn reduce_mul_result(unreduced: &MulResult<Self>) -> Field<Self> {
        let mut somewhat_reduced = AddResult::<Self> {
            x: Self::Wide::default(),
        };
        let a = unreduced.x.as_ref();
        let c = somewhat_reduced.x.as_mut();
        let p = Self::P.x.as_ref();

        let s1 = Field::<Self> {
            x: [a[0], a[1], a[2], a[3]],
        };
        let s2 = Field::<Self> {
            x: [0x0, a[5] & 0xffffffff00000000, a[6], a[7]],
        };
        let s3 = Field::<Self> {
            x: [
                0x0,
                (a[6] & 0xffffffff) << 32,
                (a[6] >> 32) | ((a[7] & 0xffffffff) << 32),
                a[7] >> 32,
            ],
        };
        let s4 = Field::<Self> {
            x: [a[4], a[5] & 0xffffffff, 0x0, a[7]],
        };
        let s5 = Field::<Self> {
            x: [
                (a[4] >> 32) | ((a[5] & 0xffffffff) << 32),
                (a[5] >> 32) | (a[6] & 0xffffffff00000000),
                a[7],
                (a[6] >> 32) | ((a[4] & 0xffffffff) << 32),
            ],
        };
        let s6 = Field::<Self> {
            x: [
                (a[5] >> 32) | ((a[6] & 0xffffffff) << 32),
                a[6] >> 32,
                0x0,
                (a[4] & 0xffffffff) | ((a[5] & 0xffffffff) << 32),
            ],
        };
        let s7 = Field::<Self> {
            x: [a[6], a[7], 0x0, (a[4] >> 32) | (a[5] & 0xffffffff00000000)],
        };
        let s8 = Field::<Self> {
            x: [
                (a[6] >> 32) | ((a[7] & 0xffffffff) << 32),
                (a[7] >> 32) | ((a[4] & 0xffffffff) << 32),
                (a[4] >> 32) | ((a[5] & 0xffffffff) << 32),
                (a[6] & 0xffffffff) << 32,
            ],
        };
        let s9 = Field::<Self> {
            x: [
                a[7],
                a[4] & 0xffffffff00000000,
                a[5],
                a[6] & 0xffffffff00000000,
            ],
        };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = ((p[j] as i128) << 2)
                + s1.x[j] as i128
                + s2.x[j] as i128
                + s2.x[j] as i128
                + s3.x[j] as i128
                + s3.x[j] as i128
                + s4.x[j] as i128
                + s5.x[j] as i128
                - s6.x[j] as i128
                - s7.x[j] as i128
                - s8.x[j] as i128
                - s9.x[j] as i128
                + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        c[Self::LIMB_SZ] = k as u64;
        somewhat_reduced.reduce()
    }

    fn normalize_point(point: &Point<Self>) -> Point<Self> {
        if point.is_point_at_infinity() {
            return Self::INFINITY;
        }

        let z = point.z;
        let t0 = z.sqr();
        let z2 = t0 * z;
        let z4 = z2.sqr_n_times(2) * z2;
        let z6 = z4.sqr_n_times(2) * z2;
        let z12 = z6.sqr_n_times(6) * z6;
        let z24 = z12.sqr_n_times(12) * z12;
        let z30 = z24.sqr_n_times(6) * z6;
        let z32 = z30.sqr_n_times(2) * z2;

        let mut zinv = z32.sqr_n_times(32) * z;
        zinv = zinv.sqr_n_times(128) * z32;
        zinv = zinv.sqr_n_times(32);
        zinv = (z32 * zinv).sqr_n_times(30) * z30;
        zinv = zinv.sqr_n_times(2);
        zinv = zinv * z;

        let zinv2 = zinv.sqr();
        let zinv3 = zinv2 * zinv;

        Point::<Self> {
            x: point.x * zinv2,
            y: point.y * zinv3,
            z: Self::ONE,
        }
    }
}

#[cfg(test)]
#[path = "unit_tests/p256_test.rs"]
mod p256_test;
