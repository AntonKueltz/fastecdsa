use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U256};
use pyo3::prelude::*;

use crate::curve::{AddResult, Curve, Field, MulResult, Point};

#[derive(Debug)]
pub struct P224;

impl Curve for P224 {
    type Limbs = [u64; 4];
    type Wide = [u64; 4];
    type Double = [u64; 8];

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
        x: Self::ONE,
        y: Self::ONE,
        z: Self::ZERO,
    };

    fn reduce_add_result(unreduced: &mut AddResult<Self>) -> Field<Self> {
        let n = Self::WIDE_SZ;
        let p = Self::P_WIDE;

        let mut t: i128;
        let mut k: i128;

        while !unreduced.less_than(&Self::P_WIDE) {
            let a = unreduced.x.as_mut();
            let q = p.x.as_ref();
            k = 0;

            for j in 0..n {
                t = a[j] as i128 - q[j] as i128 + k;
                a[j] = t as u64;
                k = t >> 64;
            }
        }

        Field {
            x: unreduced.x.clone(),
        }
    }

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

        somewhat_reduced.reduce()
    }

    fn field_add(op1: &Field<Self>, op2: &Field<Self>) -> Field<Self> {
        let mut unreduced = AddResult::<Self> {
            x: Self::Wide::default(),
        };

        let a: &[u64] = op1.x.as_ref();
        let b = op2.x.as_ref();
        let c = unreduced.x.as_mut();

        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = a[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        unreduced.reduce()
    }

    fn field_sub(op1: &Field<Self>, op2: &Field<Self>) -> Field<Self> {
        let mut unreduced = AddResult::<Self> {
            x: Self::Wide::default(),
        };

        let a = op1.x.as_ref();
        let b = op2.x.as_ref();
        let c = unreduced.x.as_mut();
        let p = Self::P;

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..Self::LIMB_SZ {
            let q = p.x.as_ref();
            t = ((q[j] as i128) << 1) - b[j] as i128 + a[j] as i128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        unreduced.reduce()
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
        let zinv2 = zinv.sqr();
        let zinv3 = zinv2 * zinv;

        Point::<Self> {
            x: point.x * zinv2,
            y: point.y * zinv3,
            z: Self::ONE,
        }
    }
}

const P224_Q: U256 =
    U256::from_be_hex("00000000ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d");
const_monty_params!(
    P224Q,
    U256,
    "00000000ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d"
);

#[inline]
fn monty_form(n: &[u8]) -> ConstMontyForm<P224Q, 4> {
    let mut bytes: [u8; 32] = [0; 32];
    bytes[..n.len()].copy_from_slice(n);

    ConstMontyForm::<P224Q, 4>::new(&U256::from_le_slice(&bytes))
}

#[pyfunction]
pub fn p224_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = (P224::G * &k_bytes).normalize();
    let r_bytes: Vec<u8> = Field::<P224>::into(p.x);

    let k = monty_form(k_bytes);
    let z = monty_form(msg);
    let r = monty_form(&r_bytes);
    let d = monty_form(d_bytes);
    let kinv = k.invert().unwrap();
    let s = kinv * (z + r * d);

    (
        r.retrieve().to_le_bytes().to_vec(),
        s.retrieve().to_le_bytes().to_vec(),
    )
}

fn is_valid_sig(r_bytes: &[u8], s_bytes: &[u8]) -> bool {
    let mut r_padded: [u8; 32] = [0; 32];
    let mut s_padded: [u8; 32] = [0; 32];
    r_padded[..28].copy_from_slice(&r_bytes[..28]);
    s_padded[..28].copy_from_slice(&s_bytes[..28]);

    let r = U256::from_le_slice(&r_padded);
    let s = U256::from_le_slice(&s_padded);

    if r.is_zero().into() || r >= P224_Q || s.is_zero().into() || s >= P224_Q {
        return false;
    }

    true
}

#[pyfunction]
pub fn p224_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    if !is_valid_sig(r_bytes, s_bytes) {
        return false;
    }

    let q = Point::<P224> {
        x: Field::<P224>::from(qx_bytes),
        y: Field::<P224>::from(qy_bytes),
        z: P224::ONE,
    };
    let z = monty_form(msg);
    let s = monty_form(s_bytes);
    let r = monty_form(r_bytes);
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = Point::<P224>::shamir(
        &P224::G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = Field::<P224>::into(p.x);
    let xq = monty_form(&x_bytes);

    xq == r
}

#[cfg(test)]
#[path = "p224_test.rs"]
mod p224_test;
