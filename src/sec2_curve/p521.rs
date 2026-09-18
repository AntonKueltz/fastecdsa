use std::sync::OnceLock;

use crypto_bigint::{U576, const_monty_params, modular::ConstMontyForm};

use crate::comb::Comb;
use crate::sec2_curve::{AddResult, Field, MulResult, Point, Sec2Curve};

#[derive(Debug)]
pub struct P521;

const_monty_params!(
    P521Q,
    U576,
    "00000000000001fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409"
);

static P521_COMB: OnceLock<Comb<Point<P521>>> = OnceLock::new();

impl Sec2Curve for P521 {
    type Limbs = [u64; 9];
    type Wide = [u64; 9];
    type Double = [u64; 18];

    type Order = ConstMontyForm<P521Q, { U576::LIMBS }>;

    const LIMB_SZ: usize = 9;
    const WIDE_SZ: usize = 9;
    const FIELD_BYTES: usize = 66;

    const P: Field<Self> = Field {
        x: [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x00000000000001ff,
        ],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x00000000000001ff,
        ],
    };
    const A: Field<Self> = Field {
        x: [
            0xfffffffffffffffc,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x00000000000001ff,
        ],
    };
    const B: Field<Self> = Field {
        x: [
            0xef451fd46b503f00,
            0x3573df883d2c34f1,
            0x1652c0bd3bb1bf07,
            0x56193951ec7e937b,
            0xb8b489918ef109e1,
            0xa2da725b99b315f3,
            0x929a21a0b68540ee,
            0x953eb9618e1c9a1f,
            0x0000000000000051,
        ],
    };
    const B3: Field<Self> = Field {
        x: [
            0xcdcf5f7d41f0bd00,
            0xa05b9e98b7849ed5,
            0x42f84237b3153d15,
            0x024babf5c57bba71,
            0x2a1d9cb4acd31da4,
            0xe88f5712cd1941db,
            0xb7ce64e2238fc2cb,
            0xbfbc2c24aa55ce5e,
            0x00000000000000f4,
        ],
    };
    const ZERO: Field<Self> = Field {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    const ONE: Field<Self> = Field {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    };
    const G: Point<Self> = Point {
        x: Field {
            x: [
                0xf97e7e31c2e5bd66,
                0x3348b3c1856a429b,
                0xfe1dc127a2ffa8de,
                0xa14b5e77efe75928,
                0xf828af606b4d3dba,
                0x9c648139053fb521,
                0x9e3ecb662395b442,
                0x858e06b70404e9cd,
                0x00000000000000c6,
            ],
        },
        y: Field {
            x: [
                0x88be94769fd16650,
                0x353c7086a272c240,
                0xc550b9013fad0761,
                0x97ee72995ef42640,
                0x17afbd17273e662c,
                0x98f54449579b4468,
                0x5c8a5fb42c7d1bd9,
                0x39296a789a3bc004,
                0x0000000000000118,
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
        let a = unreduced.x.as_ref();
        let mask: u64 = 0b111111111;

        let s1 = Field::<Self> {
            x: [
                (a[8] >> 9) | (a[9] & mask) << 55,
                (a[9] >> 9) | (a[10] & mask) << 55,
                (a[10] >> 9) | (a[11] & mask) << 55,
                (a[11] >> 9) | (a[12] & mask) << 55,
                (a[12] >> 9) | (a[13] & mask) << 55,
                (a[13] >> 9) | (a[14] & mask) << 55,
                (a[14] >> 9) | (a[15] & mask) << 55,
                (a[15] >> 9) | (a[16] & mask) << 55,
                (a[16] >> 9) | (a[17] & mask) << 55,
            ],
        };
        let s2 = Field::<Self> {
            x: [a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8] & mask],
        };

        s1 + s2
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
        let z2 = z.sqr();
        let z3 = z2 * z;
        let z4 = z3.sqr_n_times(2) * z3;
        let z8 = z4.sqr_n_times(4) * z4;
        let z16 = z8.sqr_n_times(8) * z8;
        let z32 = z16.sqr_n_times(16) * z16;
        let z64 = z32.sqr_n_times(32) * z32;
        let z65 = z64.sqr() * z;
        let z129 = z65.sqr_n_times(64) * z64;
        let z130 = z129.sqr() * z;
        let z259 = z130.sqr_n_times(129) * z129;
        let z260 = z259.sqr() * z;
        let z519 = z260.sqr_n_times(259) * z259;
        let zinv = z519.sqr_n_times(2) * z;

        Point::<Self> {
            x: point.x * zinv,
            y: point.y * zinv,
            z: Self::ONE,
        }
    }

    fn comb() -> Option<&'static Comb<Point<Self>>> {
        Some(P521_COMB.get_or_init(|| Comb::<Point<Self>>::new(Self::G, 5)))
    }
}

#[cfg(test)]
#[path = "../unit_tests/p521_test.rs"]
mod p521_test;
