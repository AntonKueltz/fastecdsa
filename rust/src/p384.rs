use std::ops::{Add, Mul, Sub};

use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U384};
use pyo3::prelude::*;

const P384_LIMBS: usize = 6;

struct P384AddResult {
    x: [u64; P384_LIMBS + 1],
}
const P384_P_7: P384AddResult = P384AddResult {
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

struct P384MulResult {
    x: [u64; P384_LIMBS << 1],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct P384Element {
    x: [u64; P384_LIMBS],
}
const P384_ONE: P384Element = P384Element {
    x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0],
};
const P384_P: P384Element = P384Element {
    x: [
        0x00000000ffffffff,
        0xffffffff00000000,
        0xfffffffffffffffe,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
    ],
};
const P384_A: P384Element = P384Element {
    x: [
        0x00000000fffffffc,
        0xffffffff00000000,
        0xfffffffffffffffe,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
    ],
};

#[derive(Clone, Copy, Debug)]
struct P384Point {
    x: P384Element,
    y: P384Element,
    z: P384Element,
}
const P384_G: P384Point = P384Point {
    x: P384Element {
        x: [
            0x3a545e3872760ab7,
            0x5502f25dbf55296c,
            0x59f741e082542a38,
            0x6e1d3b628ba79b98,
            0x8eb1c71ef320ad74,
            0xaa87ca22be8b0537,
        ],
    },
    y: P384Element {
        x: [
            0x7a431d7c90ea0e5f,
            0x0a60b1ce1d7e819d,
            0xe9da3113b5f0b8c0,
            0xf8f41dbd289a147c,
            0x5d9e98bf9292dc29,
            0x3617de4a96262c6f,
        ],
    },
    z: P384_ONE,
};
const INFINITY: P384Point = P384Point {
    x: P384Element {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    y: P384Element {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    z: P384Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
};

const P384_Q: U384 =
    U384::from_be_hex("ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973");
const_monty_params!(
    P384Q,
    U384,
    "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973"
);

impl P384AddResult {
    fn less_than(&self, other: &P384AddResult) -> bool {
        for i in (0..P384_LIMBS + 1).rev() {
            if self.x[i] < other.x[i] {
                return true;
            } else if self.x[i] > other.x[i] {
                return false;
            }
        }

        false
    }

    fn reduce(&mut self) -> P384Element {
        let mut t: i128;
        let mut k: i128;

        while !self.less_than(&P384_P_7) {
            k = 0;

            for j in 0..P384_LIMBS {
                t = self.x[j] as i128 - P384_P_7.x[j] as i128 + k;
                self.x[j] = t as u64;
                k = t >> 64;
            }

            t = self.x[P384_LIMBS] as i128 + k;
            self.x[P384_LIMBS] = t as u64;
        }

        P384Element {
            x: [
                self.x[0], self.x[1], self.x[2], self.x[3], self.x[4], self.x[5],
            ],
        }
    }
}

impl P384MulResult {
    fn reduce(&self) -> P384Element {
        let mut somewhat_reduced: P384AddResult = P384AddResult {
            x: [0; P384_LIMBS + 1],
        };

        let s1 = P384Element {
            x: [
                self.x[0], self.x[1], self.x[2], self.x[3], self.x[4], self.x[5],
            ],
        };
        let s2 = P384Element {
            x: [
                0x0,
                0x0,
                (self.x[10] >> 32) | ((self.x[11] & 0xffffffff) << 32),
                self.x[11] >> 32,
                0x0,
                0x0,
            ],
        };
        let s3 = P384Element {
            x: [
                self.x[6], self.x[7], self.x[8], self.x[9], self.x[10], self.x[11],
            ],
        };
        let s4 = P384Element {
            x: [
                (self.x[10] >> 32) | ((self.x[11] & 0xffffffff) << 32),
                (self.x[11] >> 32) | ((self.x[6] & 0xffffffff) << 32),
                (self.x[6] >> 32) | ((self.x[7] & 0xffffffff) << 32),
                (self.x[7] >> 32) | ((self.x[8] & 0xffffffff) << 32),
                (self.x[8] >> 32) | ((self.x[9] & 0xffffffff) << 32),
                (self.x[9] >> 32) | ((self.x[10] & 0xffffffff) << 32),
            ],
        };
        let s5 = P384Element {
            x: [
                self.x[11] & 0xffffffff00000000,
                (self.x[10] & 0xffffffff) << 32,
                self.x[6],
                self.x[7],
                self.x[8],
                self.x[9],
            ],
        };
        let s6 = P384Element {
            x: [0x0, 0x0, self.x[10], self.x[11], 0x0, 0x0],
        };
        let s7 = P384Element {
            x: [
                self.x[10] & 0xffffffff,
                self.x[10] & 0xffffffff00000000,
                self.x[11],
                0x0,
                0x0,
                0x0,
            ],
        };
        let s8 = P384Element {
            x: [
                (self.x[11] >> 32) | ((self.x[6] & 0xffffffff) << 32),
                (self.x[6] >> 32) | ((self.x[7] & 0xffffffff) << 32),
                (self.x[7] >> 32) | ((self.x[8] & 0xffffffff) << 32),
                (self.x[8] >> 32) | ((self.x[9] & 0xffffffff) << 32),
                (self.x[9] >> 32) | ((self.x[10] & 0xffffffff) << 32),
                (self.x[10] >> 32) | ((self.x[11] & 0xffffffff) << 32),
            ],
        };
        let s9 = P384Element {
            x: [
                (self.x[10] & 0xffffffff) << 32,
                (self.x[10] >> 32) | ((self.x[11] & 0xffffffff) << 32),
                self.x[11] >> 32,
                0x0,
                0x0,
                0x0,
            ],
        };
        let s10 = P384Element {
            x: [
                0x0,
                self.x[11] & 0xffffffff00000000,
                self.x[11] >> 32,
                0x0,
                0x0,
                0x0,
            ],
        };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..P384_LIMBS {
            t = ((P384_P.x[j] as i128) << 2)
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
            somewhat_reduced.x[j] = t as u64;
            k = t >> 64;
        }

        somewhat_reduced.x[P384_LIMBS] = k as u64;
        somewhat_reduced.reduce()
    }

    fn sqr_helper(&mut self, k: usize, t: u128) {
        let lo = t as u64;
        let hi = (t >> 64) as u64;

        let (sumk, ck) = self.x[k].overflowing_add(lo);
        self.x[k] = sumk;

        let (sumk1, ck1a) = self.x[k + 1].overflowing_add(hi);
        let (sumk1, ck1b) = sumk1.overflowing_add(ck as u64);
        self.x[k + 1] = sumk1;

        let mut carry = (ck1a || ck1b) as u64;
        let mut l = k + 2;
        while carry != 0 {
            let (s, c) = self.x[l].overflowing_add(carry);
            self.x[l] = s;
            carry = c as u64;
            l += 1;
        }
    }
}

impl From<&[u8]> for P384Element {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self { x: [0; P384_LIMBS] };

        for i in 0..P384_LIMBS {
            for j in 0..8 {
                result.x[i] |= (x[i * 8 + j] as u64) << (j * 8);
            }
        }

        result
    }
}

impl From<P384Element> for Vec<u8> {
    fn from(x: P384Element) -> Vec<u8> {
        let mut result: [u8; 48] = [0; 48];

        for i in 0..P384_LIMBS {
            for j in 0..8 {
                result[i * 8 + j] = (x.x[i] >> (j * 8)) as u8;
            }
        }

        result.to_vec()
    }
}

impl Add for P384Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut unreduced: P384AddResult = P384AddResult {
            x: [0; P384_LIMBS + 1],
        };
        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..P384_LIMBS {
            t = self.x[j] as u128 + other.x[j] as u128 + k;
            unreduced.x[j] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P384_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl Sub for P384Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut unreduced: P384AddResult = P384AddResult {
            x: [0; P384_LIMBS + 1],
        };
        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..P384_LIMBS {
            t = ((P384_P.x[j] as i128) << 1) - other.x[j] as i128 + self.x[j] as i128 + k;
            unreduced.x[j] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P384_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl Mul for P384Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut unreduced: P384MulResult = P384MulResult {
            x: [0; P384_LIMBS << 1],
        };
        let mut k: usize;
        let mut t: u128;

        for i in 0..P384_LIMBS {
            let mut carry: u128 = 0;

            for j in 0..P384_LIMBS {
                k = i + j;
                t = self.x[i] as u128 * other.x[j] as u128 + unreduced.x[k] as u128 + carry;
                unreduced.x[k] = t as u64;
                carry = t >> 64;
            }

            unreduced.x[i + P384_LIMBS] = carry as u64;
        }

        unreduced.reduce()
    }
}

impl Mul<u64> for P384Element {
    type Output = Self;

    fn mul(self, y: u64) -> Self::Output {
        let mut unreduced: P384MulResult = P384MulResult {
            x: [0; P384_LIMBS << 1],
        };
        let mut t: u128;
        let mut k: u128 = 0;

        for i in 0..P384_LIMBS {
            t = self.x[i] as u128 * y as u128 + unreduced.x[i] as u128 + k;
            unreduced.x[i] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P384_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl P384Element {
    fn sqr(&self) -> Self {
        let mut unreduced: P384MulResult = P384MulResult {
            x: [0; P384_LIMBS << 1],
        };
        let mut t: u128;
        let mut k: usize;

        for i in 0..P384_LIMBS {
            t = self.x[i] as u128 * self.x[i] as u128;
            unreduced.sqr_helper(2 * i, t);

            for j in (i + 1)..P384_LIMBS {
                t = self.x[i] as u128 * self.x[j] as u128;
                k = i + j;

                unreduced.sqr_helper(k, t);
                unreduced.sqr_helper(k, t);
            }
        }

        unreduced.reduce()
    }

    fn sqr_n_times(&self, n: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.sqr();
        }
        result
    }
}

impl PartialEq for P384Point {
    fn eq(&self, other: &Self) -> bool {
        self.x * other.z * other.z == other.x * self.z * self.z
            && self.y * other.z * other.z * other.z == other.y * self.z * self.z * self.z
    }
}

impl Eq for P384Point {}

impl Add for P384Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.is_point_at_infinity() {
            return other.clone();
        } else if other.is_point_at_infinity() {
            return self.clone();
        } else if self == other {
            return self.double();
        }

        let x1 = self.x;
        let x2 = other.x;
        let y1 = self.y;
        let y2 = other.y;
        let z1 = self.z;
        let z2 = other.z;

        // https://hyperelliptic.org/EFD/g1p/auto-code/shortw/jacobian-3/addition/add-2007-bl.op3
        let z1z1 = z1.sqr();
        let z2z2 = z2.sqr();
        let u1 = x1 * z2z2;
        let u2 = x2 * z1z1;
        let s1 = y1 * z2 * z2z2;
        let s2 = y2 * z1 * z1z1;
        let h = u2 - u1;
        let i = (h * 2).sqr();
        let j = h * i;
        let r = (s2 - s1) * 2;
        let v = u1 * i;
        let x3 = r.sqr() - j - v * 2;
        let y3 = r * (v - x3) - s1 * j * 2;
        let z3 = ((z1 + z2).sqr() - z1z1 - z2z2) * h;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

impl Mul<&[u8]> for P384Point {
    type Output = Self;

    fn mul(self, n: &[u8]) -> Self::Output {
        if n.len() == 0 {
            return INFINITY;
        }

        let mut j = n.len() * 8 - 1;
        while !test_bit(n, j) {
            j -= 1;
        }

        let mut r0: Self = INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..j + 1).rev() {
            if test_bit(n, i) {
                r0 = r0 + r1;
                r1 = r1.double();
            } else {
                r1 = r1 + r0;
                r0 = r0.double();
            }
        }

        r0
    }
}

impl P384Point {
    fn is_point_at_infinity(&self) -> bool {
        self.z.x == [0x0, 0x0, 0x0, 0x0, 0x0, 0x0]
    }

    fn normalize(&self) -> P384Point {
        if self.is_point_at_infinity() {
            return INFINITY;
        }

        let z = self.z;
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

        let zinv2 = zinv.sqr();
        let zinv3 = zinv2 * zinv;

        Self {
            x: self.x * zinv2,
            y: self.y * zinv3,
            z: P384_ONE,
        }
    }

    fn double(&self) -> P384Point {
        if self.is_point_at_infinity() {
            return INFINITY;
        }

        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        // https://hyperelliptic.org/EFD/g1p/auto-code/shortw/jacobian-3/doubling/dbl-2007-bl.op3
        let xx = x1.sqr();
        let yy = y1.sqr();
        let yyyy = yy.sqr();
        let zz = z1.sqr();
        let s = ((x1 + yy).sqr() - xx - yyyy) * 2;
        let m = xx * 3 + P384_A * zz.sqr();
        let t = m.sqr() - s * 2;
        let y3 = m * (s - t) - yyyy * 8;
        let z3 = (y1 + z1).sqr() - yy - zz;

        Self { x: t, y: y3, z: z3 }
    }

    fn shamir(p: &P384Point, q: &P384Point, n: &[u8], m: &[u8]) -> P384Point {
        let mut j = n.len() * 8 - 1;
        while !test_bit(n, j) && !test_bit(m, j) {
            j -= 1;
        }

        let pq = *p + *q;
        let mut r = INFINITY;

        for i in (0..j + 1).rev() {
            r = r.double();

            if test_bit(n, i) && test_bit(m, i) {
                r = r + pq;
            } else if test_bit(n, i) {
                r = r + *p;
            } else if test_bit(m, i) {
                r = r + *q;
            }
        }

        r
    }
}

#[pyfunction]
pub fn p384_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = P384_G.mul(&k_bytes).normalize();
    let r_bytes: Vec<u8> = P384Element::into(p.x);

    let k = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(k_bytes));
    let z = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(msg));
    let r = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(&r_bytes));
    let d = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(d_bytes));
    let kinv = k.invert().unwrap();
    let s = kinv * (z + r * d);

    (
        r.retrieve().to_le_bytes().to_vec(),
        s.retrieve().to_le_bytes().to_vec(),
    )
}

fn is_valid_sig(r_bytes: &[u8], s_bytes: &[u8]) -> bool {
    let r = U384::from_le_slice(r_bytes);
    let s = U384::from_le_slice(s_bytes);

    if r.is_zero().into() || r >= P384_Q || s.is_zero().into() || s >= P384_Q {
        return false;
    }

    true
}

#[pyfunction]
pub fn p384_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    if !is_valid_sig(r_bytes, s_bytes) {
        return false;
    }

    let q: P384Point = P384Point {
        x: P384Element::from(qx_bytes),
        y: P384Element::from(qy_bytes),
        z: P384_ONE,
    };
    let z = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(msg));
    let s = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(s_bytes));
    let r = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(r_bytes));
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = P384Point::shamir(
        &P384_G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = P384Element::into(p.x);
    let xq = ConstMontyForm::<P384Q, 6>::new(&U384::from_le_slice(&x_bytes));

    xq == r
}

#[inline]
fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;

    ((n[byte] >> bit) & 1) == 1
}

#[cfg(test)]
#[path = "p384_test.rs"]
mod p384_test;
