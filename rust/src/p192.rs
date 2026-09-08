use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U192};

use crate::curve::{AddResult, Curve, Field, MulResult, Point};

#[derive(Debug)]
pub struct P192;

const_monty_params!(
    P192Q,
    U192,
    "ffffffffffffffffffffffff99def836146bc9b1b4d22831"
);

impl Curve for P192 {
    type Limbs = [u64; 3];
    type Wide = [u64; 4];
    type Double = [u64; 6];

    type Order = ConstMontyForm<P192Q, 3>;

    const LIMB_SZ: usize = 3;
    const WIDE_SZ: usize = 4;
    const FIELD_BYTES: usize = 24;

    const P: Field<Self> = Field {
        x: [0xffffffffffffffff, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    const P_WIDE: AddResult<Self> = AddResult {
        x: [
            0xffffffffffffffff,
            0xfffffffffffffffe,
            0xffffffffffffffff,
            0x0,
        ],
    };
    const A: Field<Self> = Field {
        x: [0xfffffffffffffffc, 0xfffffffffffffffe, 0xffffffffffffffff],
    };
    const ZERO: Field<Self> = Field { x: [0x0, 0x0, 0x0] };
    const ONE: Field<Self> = Field { x: [0x1, 0x0, 0x0] };
    const G: Point<Self> = Point {
        x: Field {
            x: [0xf4ff0afd82ff1012, 0x7cbf20eb43a18800, 0x188da80eb03090f6],
        },
        y: Field {
            x: [0x73f977a11e794811, 0x631011ed6b24cdd5, 0x07192b95ffc8da78],
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

        let mut t: u128 = a[0] as u128 + a[3] as u128 + a[5] as u128;
        c[0] = t as u64;
        let mut k: u64 = (t >> 64) as u64;

        t = a[1] as u128 + a[3] as u128 + a[4] as u128 + a[5] as u128 + k as u128;
        c[1] = t as u64;
        k = (t >> 64) as u64;

        t = a[2] as u128 + a[4] as u128 + a[5] as u128 + k as u128;
        c[2] = t as u64;
        k = (t >> 64) as u64;
        c[3] = k;

        somewhat_reduced.reduce()
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

        let mut zinv = z127.sqr();
        zinv = zinv.sqr_n_times(62) * z62;
        zinv = zinv.sqr().sqr() * z;

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
#[path = "unit_tests/p192_test.rs"]
mod p192_test;
