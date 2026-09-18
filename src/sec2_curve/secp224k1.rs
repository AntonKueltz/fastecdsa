use std::sync::OnceLock;

use crypto_bigint::{U256, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::sec2_curve::{AddResult, Field, MulResult, Point, Sec2Curve};

#[derive(Debug)]
pub struct Secp224k1;

const_monty_params!(
    Secp224k1Q,
    U256,
    "000000010000000000000000000000000001dce8d2ec6184caf0a971769fb1f7"
);

static SECP224K1_COMB: OnceLock<Comb<Point<Secp224k1>>> = OnceLock::new();

impl Sec2Curve for Secp224k1 {
    type Limbs = [u64; 4];
    type Wide = [u64; 5];
    type Double = [u64; 8];

    type Order = ConstMontyForm<Secp224k1Q, { U256::LIMBS }>;

    const LIMB_SZ: usize = 4;
    const WIDE_SZ: usize = 5;
    const FIELD_BYTES: usize = 29;

    const P: Field<Self> = Field {
        x: [
            0xfffffffeffffe56d,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x00000000ffffffff,
        ],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xfffffffeffffe56d,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x00000000ffffffff,
            0x0,
        ],
    };
    const A: Field<Self> = Field {
        x: [0x0, 0x0, 0x0, 0x0],
    };
    const B: Field<Self> = Field {
        x: [0x5, 0x0, 0x0, 0x0],
    };
    const B3: Field<Self> = Field {
        x: [0xf, 0x0, 0x0, 0x0],
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
                0x0f7e650eb6b7a45c,
                0x69a467e9e47075a9,
                0x4df099df30fc28a1,
                0x00000000a1455b33,
            ],
        },
        y: Field {
            x: [
                0xe2ca4bdb556d61a5,
                0xf7e319f7c0b0bd59,
                0x7fba344282cafbd6,
                0x000000007e089fed,
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
        let r: u64 = 0x100001a93;

        let a = unreduced.x.as_ref();
        let mut reduced = AddResult::<Self> {
            x: [a[0], a[1], a[2], a[3] & 0xffffffff, 0x0],
        };
        let c = reduced.x.as_mut();
        let hi = Field::<Self> {
            x: [
                (a[3] >> 32) | ((a[4] & 0xffffffff) << 32),
                (a[4] >> 32) | ((a[5] & 0xffffffff) << 32),
                (a[5] >> 32) | ((a[6] & 0xffffffff) << 32),
                a[6] >> 32,
            ],
        };
        let scaled = hi.scale_wide(r);
        let b = scaled.x.as_ref();

        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = c[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }
        k += b[Self::LIMB_SZ] as u128;

        let overflow = (c[Self::LIMB_SZ - 1] >> 32) as u128 | ((k as u128) << 32);
        let overflow = overflow * r as u128;
        let scaled = AddResult::<Self> {
            x: [overflow as u64, (overflow >> 64) as u64, 0x0, 0x0, 0x0],
        };
        let b = scaled.x.as_ref();
        c[Self::LIMB_SZ - 1] &= 0xffffffff;

        k = 0;
        for j in 0..Self::LIMB_SZ {
            t = c[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

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
        let z2 = z.sqr() * z;
        let z4 = z2.sqr_n_times(2) * z2;
        let z8 = z4.sqr_n_times(4) * z4;
        let z9 = z8.sqr() * z;
        let z18 = z9.sqr_n_times(9) * z9;
        let z19 = z18.sqr() * z;
        let z38 = z19.sqr_n_times(19) * z19;
        let z76 = z38.sqr_n_times(38) * z38;
        let z152 = z76.sqr_n_times(76) * z76;
        let z171 = z152.sqr_n_times(19) * z19;
        let z190 = z171.sqr_n_times(19) * z19;
        let z191 = z190.sqr() * z;

        let mut zinv = z191.sqr_n_times(20) * z19;
        zinv = zinv.sqr_n_times(3) * z;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr() * z;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr() * z;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: Self::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Point<Self>>> {
        Some(SECP224K1_COMB.get_or_init(|| Comb::<Point<Self>>::new(Self::G, 4)))
    }
}

#[cfg(test)]
#[path = "../unit_tests/secp224k1_test.rs"]
mod secp224k1_test;
