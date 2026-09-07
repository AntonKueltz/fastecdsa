use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U192};
use pyo3::prelude::*;

use crate::curve::{AddResult, Curve, Field, MulResult, Point};

#[derive(Debug)]
pub struct P192;

impl Curve for P192 {
    type Limbs = [u64; 3];
    type Wide = [u64; 4];
    type Double = [u64; 6];

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

const P192_Q: U192 = U192::from_be_hex("ffffffffffffffffffffffff99def836146bc9b1b4d22831");
const_monty_params!(
    P192Q,
    U192,
    "ffffffffffffffffffffffff99def836146bc9b1b4d22831"
);

#[pyfunction]
pub fn p192_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = (P192::G * &k_bytes).normalize();
    let r_bytes: Vec<u8> = Field::<P192>::into(p.x);

    let k = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(k_bytes));
    let z = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(msg));
    let r = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(&r_bytes));
    let d = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(d_bytes));
    let kinv = k.invert().unwrap();
    let s = kinv * (z + r * d);

    (
        r.retrieve().to_le_bytes().to_vec(),
        s.retrieve().to_le_bytes().to_vec(),
    )
}

fn is_valid_sig(r_bytes: &[u8], s_bytes: &[u8]) -> bool {
    let r = U192::from_le_slice(r_bytes);
    let s = U192::from_le_slice(s_bytes);

    if r.is_zero().into() || r >= P192_Q || s.is_zero().into() || s >= P192_Q {
        return false;
    }

    true
}

#[pyfunction]
pub fn p192_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    if !is_valid_sig(r_bytes, s_bytes) {
        return false;
    }

    let q = Point::<P192> {
        x: Field::<P192>::from(qx_bytes),
        y: Field::<P192>::from(qy_bytes),
        z: P192::ONE,
    };
    let z = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(msg));
    let s = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(s_bytes));
    let r = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(r_bytes));
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = Point::<P192>::shamir(
        &P192::G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = Field::<P192>::into(p.x);
    let xq = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(&x_bytes));

    xq == r
}

#[cfg(test)]
#[path = "p192_test.rs"]
mod p192_test;
