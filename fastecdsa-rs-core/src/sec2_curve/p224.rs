use std::sync::OnceLock;

use crypto_bigint::{U256, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::sec2_curve::{AddResult, Field, MulResult, Point, Sec2Curve};
use crate::wnaf::lookup_table;

#[derive(Debug)]
pub struct P224;

const_monty_params!(
    P224Q,
    U256,
    "00000000ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d"
);

static P224_COMB: OnceLock<Comb<Point<P224>>> = OnceLock::new();
static P224_G_TABLE: OnceLock<Vec<Point<P224>>> = OnceLock::new();

impl Sec2Curve for P224 {
    type Limbs = [u64; 4];
    type Wide = [u64; 4];
    type Double = [u64; 8];

    type Order = ConstMontyForm<P224Q, { U256::LIMBS }>;

    const LIMB_SZ: usize = 4;
    const WIDE_SZ: usize = 4;
    const FIELD_BYTES: usize = 28;

    const P: Field<Self> = Field {
        x: [0x1, 0xffffffff00000000, 0xffffffffffffffff, 0xffffffff],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [0x1, 0xffffffff00000000, 0xffffffffffffffff, 0xffffffff],
    };
    const A: Field<Self> = Field {
        x: [
            0xfffffffffffffffe,
            0xfffffffeffffffff,
            0xffffffffffffffff,
            0xffffffff,
        ],
    };
    const B: Field<Self> = Field {
        x: [
            0x270b39432355ffb4,
            0x5044b0b7d7bfd8ba,
            0x0c04b3abf5413256,
            0xb4050a85,
        ],
    };
    const B3: Field<Self> = Field {
        x: [
            0x7521abc96a01ff1a,
            0xf0ce1229873f8a2e,
            0x240e1b03dfc39702,
            0x1c0f1f8f,
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
                0x343280d6115c1d21,
                0x4a03c1d356c21122,
                0x6bb4bf7f321390b9,
                0xb70e0cbd,
            ],
        },
        y: Field {
            x: [
                0x44d5819985007e34,
                0xcd4375a05a074764,
                0xb5f723fb4c22dfe6,
                0xbd376388,
            ],
        },
        z: Self::ONE,
    };
    const INFINITY: Point<Self> = Point {
        x: Self::ZERO,
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
            x: [a[0], a[1], a[2], a[3] & 0xffffffff],
        };
        let s2 = Field::<Self> {
            x: [0x0, a[3] & 0xffffffff00000000, a[4], a[5] & 0xffffffff],
        };
        let s3 = Field::<Self> {
            x: [0x0, a[5] & 0xffffffff00000000, a[6], 0x0],
        };
        let s4 = Field::<Self> {
            x: [
                (a[3] >> 32) | ((a[4] & 0xffffffff) << 32),
                (a[4] >> 32) | ((a[5] & 0xffffffff) << 32),
                (a[5] >> 32) | ((a[6] & 0xffffffff) << 32),
                a[6] >> 32,
            ],
        };
        let s5 = Field::<Self> {
            x: [
                (a[5] >> 32) | ((a[6] & 0xffffffff) << 32),
                a[6] >> 32,
                0x0,
                0x0,
            ],
        };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = ((p[j] as i128) << 1) + s1.x[j] as i128 + s2.x[j] as i128 + s3.x[j] as i128
                - s4.x[j] as i128
                - s5.x[j] as i128
                + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        somewhat_reduced.conditional_sub_p();
        somewhat_reduced.conditional_sub_p();
        somewhat_reduced.conditional_sub_p();

        Field::<Self>::from(somewhat_reduced)
    }

    fn add_point(p: &Point<Self>, q: &Point<Self>) -> Point<Self> {
        p.add_a_is_neg3(q)
    }

    fn double_point(p: &Point<Self>) -> Point<Self> {
        p.double_a_is_neg3()
    }

    fn normalize_point(point: &Point<Self>) -> Point<Self> {
        if point.is_point_at_infinity() {
            return Self::INFINITY;
        }
        let z = point.z;
        let z2 = z.sqr() * z;
        let z3 = z2.sqr() * z;
        let z6 = z3.sqr_n_times(3) * z3;

        let z7 = z6.sqr() * z;
        let z14 = z7.sqr_n_times(7) * z7;
        let z15 = z14.sqr() * z;
        let z30 = z15.sqr_n_times(15) * z15;
        let z31 = z30.sqr() * z;
        let z62 = z31.sqr_n_times(31) * z31;
        let z63 = z62.sqr() * z;
        let z126 = z63.sqr_n_times(63) * z63;
        let z127 = z126.sqr() * z;

        let z12 = z6.sqr_n_times(6) * z6;
        let z24 = z12.sqr_n_times(12) * z12;
        let z48 = z24.sqr_n_times(24) * z24;
        let z96 = z48.sqr_n_times(48) * z48;

        let zinv = z127.sqr_n_times(97) * z96;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: Self::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Point<Self>>> {
        Some(P224_COMB.get_or_init(|| Comb::<Point<Self>>::new(Self::G, 4)))
    }

    fn g_table() -> &'static Vec<Point<Self>> {
        P224_G_TABLE.get_or_init(|| lookup_table(&Self::G, 8))
    }
}

#[cfg(test)]
#[path = "../unit_tests/p224_test.rs"]
mod p224_test;
