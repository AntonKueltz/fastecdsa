use std::sync::OnceLock;

use crypto_bigint::{U256, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::sec2_curve::{AddResult, Field, MulResult, Point, Sec2Curve};
use crate::wnaf::lookup_table;

#[derive(Debug)]
pub struct Secp256k1;

const_monty_params!(
    Secp256k1Q,
    U256,
    "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141"
);

static SECP256K1_COMB: OnceLock<Comb<Point<Secp256k1>>> = OnceLock::new();
static SECP256K1_G_TABLE: OnceLock<Vec<Point<Secp256k1>>> = OnceLock::new();

impl Sec2Curve for Secp256k1 {
    type Limbs = [u64; 4];
    type Wide = [u64; 5];
    type Double = [u64; 8];

    type Order = ConstMontyForm<Secp256k1Q, { U256::LIMBS }>;

    const LIMB_SZ: usize = 4;
    const WIDE_SZ: usize = 5;
    const FIELD_BYTES: usize = 32;

    const P: Field<Self> = Field {
        x: [
            0xfffffffefffffc2f,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xfffffffefffffc2f,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x0,
        ],
    };
    const A: Field<Self> = Field {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    const B: Field<Self> = Field {
        x: [0x7, 0x0, 0x0, 0x0],
    };
    const B3: Field<Self> = Field {
        x: [0x15, 0x0, 0x0, 0x0],
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
                0x59f2815b16f81798,
                0x029bfcdb2dce28d9,
                0x55a06295ce870b07,
                0x79be667ef9dcbbac,
            ],
        },
        y: Field {
            x: [
                0x9c47d08ffb10d4b8,
                0xfd17b448a6855419,
                0x5da4fbfc0e1108a8,
                0x483ada7726a3c465,
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
        let r: u64 = 0x1000003d1;

        let a = unreduced.x.as_ref();
        let mut reduced = AddResult::<Self> {
            x: [a[0], a[1], a[2], a[3], 0x0],
        };
        let c = reduced.x.as_mut();
        let hi = Field::<Self> {
            x: [a[4], a[5], a[6], a[7]],
        };
        let scaled = hi.scale_wide(r);
        let b = scaled.x.as_ref();

        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..Self::WIDE_SZ {
            t = c[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        let overflow = c[Self::LIMB_SZ] as u128 * r as u128;
        let scaled = AddResult::<Self> {
            x: [overflow as u64, (overflow >> 64) as u64, 0x0, 0x0, 0x0],
        };
        let b = scaled.x.as_ref();
        c[Self::LIMB_SZ] = 0x0;

        for j in 0..Self::LIMB_SZ {
            t = c[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        c[Self::LIMB_SZ] = k as u64;
        reduced.conditional_sub_p();

        Field::<Self>::from(reduced)
    }

    fn add_point(p: &Point<Self>, q: &Point<Self>) -> Point<Self> {
        p.add_a_is_0(q)
    }

    fn double_point(p: &Point<Self>) -> Point<Self> {
        p.double_a_is_0()
    }

    fn normalize_point(point: &Point<Self>) -> Point<Self> {
        if point.is_point_at_infinity() {
            return Self::INFINITY;
        }

        let z = point.z;
        let t0 = z.sqr();
        let z2 = t0 * z;
        let z4 = z2.sqr_n_times(2) * z2;
        let t1 = z4.sqr();
        let z5 = t1 * z;
        let z7 = z5.sqr_n_times(2) * z2;
        let z11 = z7.sqr_n_times(4) * z4;
        let z22 = z11.sqr_n_times(11) * z11;
        let z27 = z22.sqr_n_times(5) * z5;
        let z54 = z27.sqr_n_times(27) * z27;
        let z108 = z54.sqr_n_times(54) * z54;
        let z216 = z108.sqr_n_times(108) * z108;
        let z223 = z216.sqr_n_times(7) * z7;

        let mut zinv = z223.sqr_n_times(23) * z22;
        zinv = zinv.sqr_n_times(5) * z;
        zinv = zinv.sqr_n_times(3);
        zinv = z2 * zinv;
        zinv = zinv.sqr_n_times(2);
        zinv = zinv * z;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: Self::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Point<Self>>> {
        Some(SECP256K1_COMB.get_or_init(|| Comb::<Point<Self>>::new(Self::G, 4)))
    }

    fn g_table() -> &'static Vec<Point<Self>> {
        SECP256K1_G_TABLE.get_or_init(|| lookup_table(&Self::G, 8))
    }
}

#[cfg(test)]
#[path = "../unit_tests/secp256k1_test.rs"]
mod secp256k1_test;
