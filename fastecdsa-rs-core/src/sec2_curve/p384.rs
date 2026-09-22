use std::sync::OnceLock;

use crypto_bigint::{U384, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::sec2_curve::{AddResult, Field, MulResult, Point, Sec2Curve};
use crate::wnaf::lookup_table;

#[derive(Debug)]
pub struct P384;

const_monty_params!(
    P384Q,
    U384,
    "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973"
);

static P384_COMB: OnceLock<Comb<Point<P384>>> = OnceLock::new();
static P384_G_TABLE: OnceLock<Vec<Point<P384>>> = OnceLock::new();

impl Sec2Curve for P384 {
    type Limbs = [u64; 6];
    type Wide = [u64; 7];
    type Double = [u64; 12];

    type Order = ConstMontyForm<P384Q, { U384::LIMBS }>;

    const LIMB_SZ: usize = 6;
    const WIDE_SZ: usize = 7;
    const FIELD_BYTES: usize = 48;

    const P: Field<Self> = Field {
        x: [
            0x00000000ffffffff,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0x00000000ffffffff,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x0,
        ],
    };
    const A: Field<Self> = Field {
        x: [
            0x00000000fffffffc,
            0xffffffff00000000,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    const B: Field<Self> = Field {
        x: [
            0x2a85c8edd3ec2aef,
            0xc656398d8a2ed19d,
            0x0314088f5013875a,
            0x181d9c6efe814112,
            0x988e056be3f82d19,
            0xb3312fa7e23ee7e4,
        ],
    };
    const B3: Field<Self> = Field {
        x: [
            0x7f915ac77bc480cf,
            0x5302acaa9e8c74d7,
            0x093c19adf03a9612,
            0x4858d54cfb83c336,
            0xc9aa1043abe8874b,
            0x19938ef7a6bcb7ad,
        ],
    };
    const ZERO: Field<Self> = Field {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    const ONE: Field<Self> = Field {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    const G: Point<Self> = Point {
        x: Field {
            x: [
                0x3a545e3872760ab7,
                0x5502f25dbf55296c,
                0x59f741e082542a38,
                0x6e1d3b628ba79b98,
                0x8eb1c71ef320ad74,
                0xaa87ca22be8b0537,
            ],
        },
        y: Field {
            x: [
                0x7a431d7c90ea0e5f,
                0x0a60b1ce1d7e819d,
                0xe9da3113b5f0b8c0,
                0xf8f41dbd289a147c,
                0x5d9e98bf9292dc29,
                0x3617de4a96262c6f,
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
            x: [a[0], a[1], a[2], a[3], a[4], a[5]],
        };
        let s2 = Field::<Self> {
            x: [
                0x0,
                0x0,
                (a[10] >> 32) | ((a[11] & 0xffffffff) << 32),
                a[11] >> 32,
                0x0,
                0x0,
            ],
        };
        let s3 = Field::<Self> {
            x: [a[6], a[7], a[8], a[9], a[10], a[11]],
        };
        let s4 = Field::<Self> {
            x: [
                (a[10] >> 32) | ((a[11] & 0xffffffff) << 32),
                (a[11] >> 32) | ((a[6] & 0xffffffff) << 32),
                (a[6] >> 32) | ((a[7] & 0xffffffff) << 32),
                (a[7] >> 32) | ((a[8] & 0xffffffff) << 32),
                (a[8] >> 32) | ((a[9] & 0xffffffff) << 32),
                (a[9] >> 32) | ((a[10] & 0xffffffff) << 32),
            ],
        };
        let s5 = Field::<Self> {
            x: [
                a[11] & 0xffffffff00000000,
                (a[10] & 0xffffffff) << 32,
                a[6],
                a[7],
                a[8],
                a[9],
            ],
        };
        let s6 = Field::<Self> {
            x: [0x0, 0x0, a[10], a[11], 0x0, 0x0],
        };
        let s7 = Field::<Self> {
            x: [
                a[10] & 0xffffffff,
                a[10] & 0xffffffff00000000,
                a[11],
                0x0,
                0x0,
                0x0,
            ],
        };
        let s8 = Field::<Self> {
            x: [
                (a[11] >> 32) | ((a[6] & 0xffffffff) << 32),
                (a[6] >> 32) | ((a[7] & 0xffffffff) << 32),
                (a[7] >> 32) | ((a[8] & 0xffffffff) << 32),
                (a[8] >> 32) | ((a[9] & 0xffffffff) << 32),
                (a[9] >> 32) | ((a[10] & 0xffffffff) << 32),
                (a[10] >> 32) | ((a[11] & 0xffffffff) << 32),
            ],
        };
        let s9 = Field::<Self> {
            x: [
                (a[10] & 0xffffffff) << 32,
                (a[10] >> 32) | ((a[11] & 0xffffffff) << 32),
                a[11] >> 32,
                0x0,
                0x0,
                0x0,
            ],
        };
        let s10 = Field::<Self> {
            x: [0x0, a[11] & 0xffffffff00000000, a[11] >> 32, 0x0, 0x0, 0x0],
        };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = ((p[j] as i128) << 2)
                + s1.x[j] as i128
                + s2.x[j] as i128
                + s2.x[j] as i128
                + s3.x[j] as i128
                + s4.x[j] as i128
                + s5.x[j] as i128
                + s6.x[j] as i128
                + s7.x[j] as i128
                - s8.x[j] as i128
                - s9.x[j] as i128
                - s10.x[j] as i128
                + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        let q = k as u64;
        c[Self::LIMB_SZ] = q;

        let scaled_p = Self::P.scale_wide(q);
        let (mut reduced, underflow) = somewhat_reduced - scaled_p;
        reduced.conditional_add_p(underflow);
        reduced.conditional_sub_p();

        Field::<Self>::from(reduced)
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
        let t0 = z.sqr();
        let t1 = t0 * z;
        let t2 = t1.sqr();
        let z3 = t2 * z;
        let z6 = z3.sqr_n_times(3) * z3;
        let z12 = z6.sqr_n_times(6) * z6;
        let z24 = z12.sqr_n_times(12) * z12;
        let z30 = z24.sqr_n_times(6) * z6;
        let z31 = z30.sqr() * z;
        let z32 = z31.sqr() * z;
        let z63 = z32.sqr_n_times(31) * z31;
        let z126 = z63.sqr_n_times(63) * z63;
        let z252 = z126.sqr_n_times(126) * z126;
        let z255 = z252.sqr_n_times(3) * z3;

        let mut zinv = z255;
        zinv = zinv.sqr_n_times(33) * z32;
        zinv = zinv.sqr_n_times(94) * z30;
        zinv = zinv.sqr();
        zinv = zinv.sqr() * z;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: P384::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Point<Self>>> {
        Some(P384_COMB.get_or_init(|| Comb::<Point<Self>>::new(Self::G, 4)))
    }

    fn g_table() -> &'static Vec<Point<Self>> {
        P384_G_TABLE.get_or_init(|| lookup_table(&Self::G, 8))
    }
}

#[cfg(test)]
#[path = "../unit_tests/p384_test.rs"]
mod p384_test;
