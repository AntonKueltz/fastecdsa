use std::sync::OnceLock;

use crypto_bigint::{U192, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::curve::{AddResult, Curve, Field, MulResult, Point};

#[derive(Debug)]
pub struct Secp192k1;

const_monty_params!(
    Secp192k1Q,
    U192,
    "fffffffffffffffffffffffe26f2fc170f69466a74defd8d"
);

static SECP192K1_COMB: OnceLock<Comb<Secp192k1>> = OnceLock::new();

impl Curve for Secp192k1 {
    type Limbs = [u64; 3];
    type Wide = [u64; 4];
    type Double = [u64; 6];

    type Order = ConstMontyForm<Secp192k1Q, 3>;

    const LIMB_SZ: usize = 3;
    const WIDE_SZ: usize = 4;
    const FIELD_BYTES: usize = 24;

    const P: Field<Self> = Field {
        x: [0xfffffffeffffee37, 0xffffffffffffffff, 0xffffffffffffffff],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xfffffffeffffee37,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x0,
        ],
    };
    const A: Field<Self> = Field { x: [0x0, 0x0, 0x0] };
    const B: Field<Self> = Field { x: [0x3, 0x0, 0x0] };
    const B3: Field<Self> = Field { x: [0x9, 0x0, 0x0] };
    const ZERO: Field<Self> = Field { x: [0x0, 0x0, 0x0] };
    const ONE: Field<Self> = Field { x: [0x1, 0x0, 0x0] };
    const G: Point<Self> = Point {
        x: Field {
            x: [0x1da5d1b1eae06c7d, 0x26b07d0280b7f434, 0xdb4ff10ec057e9ae],
        },
        y: Field {
            x: [0x4082aa88d95e2f9d, 0x844163d015be8634, 0x9b2f2f6d9c5628a7],
        },
        z: Self::ONE,
    };
    const INFINITY: Point<Self> = Point {
        x: Self::ZERO,
        y: Self::ONE,
        z: Self::ZERO,
    };

    fn reduce_mul_result(unreduced: &MulResult<Self>) -> Field<Self> {
        let r: u64 = 0x1000011c9;

        let a = unreduced.x.as_ref();
        let mut reduced = AddResult::<Self> {
            x: [a[0], a[1], a[2], 0x0],
        };
        let c = reduced.x.as_mut();
        let hi = Field::<Self> {
            x: [a[3], a[4], a[5]],
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
            x: [overflow as u64, (overflow >> 64) as u64, 0x0, 0x0],
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
        let z2 = z.sqr() * z;
        let z4 = z2.sqr_n_times(2) * z2;
        let z8 = z4.sqr_n_times(4) * z4;
        let z9 = z8.sqr() * z;
        let z18 = z9.sqr_n_times(9) * z9;
        let z19 = z18.sqr() * z;
        let z38 = z19.sqr_n_times(19) * z19;
        let z39 = z38.sqr() * z;
        let z78 = z39.sqr_n_times(39) * z39;
        let z79 = z78.sqr() * z;
        let z158 = z79.sqr_n_times(79) * z79;
        let z159 = z158.sqr() * z;

        let mut zinv = z159.sqr_n_times(20) * z19;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr() * z;
        zinv = zinv.sqr() * z;
        zinv = zinv.sqr_n_times(4) * z;
        zinv = zinv.sqr() * z;
        zinv = zinv.sqr_n_times(2) * z;
        zinv = zinv.sqr_n_times(2) * z;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: Self::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Self>> {
        Some(SECP192K1_COMB.get_or_init(|| Comb::new(Self::G, 4)))
    }
}

#[cfg(test)]
#[path = "unit_tests/secp192k1_test.rs"]
mod secp192k1_test;
