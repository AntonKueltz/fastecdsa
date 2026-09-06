use std::ops::{Add, Mul, Sub};

use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U256};
use pyo3::prelude::*;

const P256_LIMBS: usize = 4;

struct P256AddResult {
    x: [u64; P256_LIMBS + 1],
}
const P256_P_5: P256AddResult = P256AddResult {
    x: [
        0xffffffffffffffff,
        0x00000000ffffffff,
        0x0000000000000000,
        0xffffffff00000001,
        0x0,
    ],
};

struct P256MulResult {
    x: [u64; P256_LIMBS << 1],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct P256Element {
    x: [u64; P256_LIMBS],
}
const P256_ONE: P256Element = P256Element {
    x: [0x1, 0x0, 0x0, 0x0],
};
const P256_P: P256Element = P256Element {
    x: [
        0xffffffffffffffff,
        0x00000000ffffffff,
        0x0000000000000000,
        0xffffffff00000001,
    ],
};
const P256_A: P256Element = P256Element {
    x: [
        0xfffffffffffffffc,
        0x00000000ffffffff,
        0x0000000000000000,
        0xffffffff00000001,
    ],
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct P256Point {
    x: P256Element,
    y: P256Element,
    z: P256Element,
}
const P256_G: P256Point = P256Point {
    x: P256Element {
        x: [
            0xf4a13945d898c296,
            0x77037d812deb33a0,
            0xf8bce6e563a440f2,
            0x6b17d1f2e12c4247,
        ],
    },
    y: P256Element {
        x: [
            0xcbb6406837bf51f5,
            0x2bce33576b315ece,
            0x8ee7eb4a7c0f9e16,
            0x4fe342e2fe1a7f9b,
        ],
    },
    z: P256_ONE,
};
const INFINITY: P256Point = P256Point {
    x: P256Element {
        x: [0x0, 0x0, 0x0, 0x0],
    },
    y: P256Element {
        x: [0x1, 0x0, 0x0, 0x0],
    },
    z: P256Element {
        x: [0x0, 0x0, 0x0, 0x0],
    },
};

const_monty_params!(
    P256Q,
    U256,
    "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551"
);

impl P256AddResult {
    fn less_than(&self, other: &P256AddResult) -> bool {
        for i in (0..P256_LIMBS + 1).rev() {
            if self.x[i] < other.x[i] {
                return true;
            } else if self.x[i] > other.x[i] {
                return false;
            }
        }

        false
    }

    fn reduce(&mut self) -> P256Element {
        let mut t: i128;
        let mut k: i128;

        while !self.less_than(&P256_P_5) {
            t = self.x[0] as i128 - P256_P_5.x[0] as i128;
            self.x[0] = t as u64;
            k = t >> 64;

            t = self.x[1] as i128 - P256_P_5.x[1] as i128 + k;
            self.x[1] = t as u64;
            k = t >> 64;

            t = self.x[2] as i128 - P256_P_5.x[2] as i128 + k;
            self.x[2] = t as u64;
            k = t >> 64;

            t = self.x[3] as i128 - P256_P_5.x[3] as i128 + k;
            self.x[3] = t as u64;
            k = t >> 64;

            t = self.x[4] as i128 + k;
            self.x[4] = t as u64;
        }

        P256Element {
            x: [self.x[0], self.x[1], self.x[2], self.x[3]],
        }
    }
}

impl P256MulResult {
    fn reduce(&self) -> P256Element {
        let mut somewhat_reduced: P256AddResult = P256AddResult {
            x: [0; P256_LIMBS + 1],
        };

        let s1 = P256Element {
            x: [self.x[0], self.x[1], self.x[2], self.x[3]],
        };
        let s2 = P256Element {
            x: [0x0, self.x[5] & 0xffffffff00000000, self.x[6], self.x[7]],
        };
        let s3 = P256Element {
            x: [
                0x0,
                (self.x[6] & 0xffffffff) << 32,
                (self.x[6] >> 32) | ((self.x[7] & 0xffffffff) << 32),
                self.x[7] >> 32,
            ],
        };
        let s4 = P256Element {
            x: [self.x[4], self.x[5] & 0xffffffff, 0x0, self.x[7]],
        };
        let s5 = P256Element {
            x: [
                (self.x[4] >> 32) | ((self.x[5] & 0xffffffff) << 32),
                (self.x[5] >> 32) | (self.x[6] & 0xffffffff00000000),
                self.x[7],
                (self.x[6] >> 32) | ((self.x[4] & 0xffffffff) << 32),
            ],
        };
        let s6 = P256Element {
            x: [
                (self.x[5] >> 32) | ((self.x[6] & 0xffffffff) << 32),
                self.x[6] >> 32,
                0x0,
                (self.x[4] & 0xffffffff) | ((self.x[5] & 0xffffffff) << 32),
            ],
        };
        let s7 = P256Element {
            x: [
                self.x[6],
                self.x[7],
                0x0,
                (self.x[4] >> 32) | (self.x[5] & 0xffffffff00000000),
            ],
        };
        let s8 = P256Element {
            x: [
                (self.x[6] >> 32) | ((self.x[7] & 0xffffffff) << 32),
                (self.x[7] >> 32) | ((self.x[4] & 0xffffffff) << 32),
                (self.x[4] >> 32) | ((self.x[5] & 0xffffffff) << 32),
                (self.x[6] & 0xffffffff) << 32,
            ],
        };
        let s9 = P256Element {
            x: [
                self.x[7],
                self.x[4] & 0xffffffff00000000,
                self.x[5],
                self.x[6] & 0xffffffff00000000,
            ],
        };

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..P256_LIMBS {
            t = ((P256_P.x[j] as i128) << 2)
                + s1.x[j] as i128
                + s2.x[j] as i128
                + s2.x[j] as i128
                + s3.x[j] as i128
                + s3.x[j] as i128
                + s4.x[j] as i128
                + s5.x[j] as i128
                - s6.x[j] as i128
                - s7.x[j] as i128
                - s8.x[j] as i128
                - s9.x[j] as i128
                + k;
            somewhat_reduced.x[j] = t as u64;
            k = t >> 64;
        }

        somewhat_reduced.x[P256_LIMBS] = k as u64;
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

impl From<&[u8]> for P256Element {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self { x: [0; P256_LIMBS] };

        for i in 0..P256_LIMBS {
            for j in 0..8 {
                result.x[i] |= (x[i * 8 + j] as u64) << (j * 8);
            }
        }

        result
    }
}

impl From<P256Element> for Vec<u8> {
    fn from(x: P256Element) -> Vec<u8> {
        let mut result: [u8; 32] = [0; 32];

        for i in 0..P256_LIMBS {
            for j in 0..8 {
                result[i * 8 + j] = (x.x[i] >> (j * 8)) as u8;
            }
        }

        result.to_vec()
    }
}

impl Add for P256Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut unreduced: P256AddResult = P256AddResult {
            x: [0; P256_LIMBS + 1],
        };
        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..P256_LIMBS {
            t = self.x[j] as u128 + other.x[j] as u128 + k;
            unreduced.x[j] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P256_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl Sub for P256Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut unreduced: P256AddResult = P256AddResult {
            x: [0; P256_LIMBS + 1],
        };
        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..P256_LIMBS {
            t = ((P256_P.x[j] as i128) << 1) - other.x[j] as i128 + self.x[j] as i128 + k;
            unreduced.x[j] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P256_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl Mul for P256Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut unreduced: P256MulResult = P256MulResult {
            x: [0; P256_LIMBS << 1],
        };
        let mut k: usize;
        let mut t: u128;

        for i in 0..P256_LIMBS {
            let mut carry: u128 = 0;

            for j in 0..P256_LIMBS {
                k = i + j;
                t = self.x[i] as u128 * other.x[j] as u128 + unreduced.x[k] as u128 + carry;
                unreduced.x[k] = t as u64;
                carry = t >> 64;
            }

            unreduced.x[i + P256_LIMBS] = carry as u64;
        }

        unreduced.reduce()
    }
}

impl Mul<u64> for P256Element {
    type Output = Self;

    fn mul(self, y: u64) -> Self::Output {
        let mut unreduced: P256MulResult = P256MulResult {
            x: [0; P256_LIMBS << 1],
        };
        let mut t: u128;
        let mut k: u128 = 0;

        for i in 0..P256_LIMBS {
            t = self.x[i] as u128 * y as u128 + unreduced.x[i] as u128 + k;
            unreduced.x[i] = t as u64;
            k = t >> 64;
        }

        unreduced.x[P256_LIMBS] = k as u64;
        unreduced.reduce()
    }
}

impl P256Element {
    fn sqr(&self) -> Self {
        let mut unreduced: P256MulResult = P256MulResult {
            x: [0; P256_LIMBS << 1],
        };
        let mut t: u128;
        let mut k: usize;

        for i in 0..P256_LIMBS {
            t = self.x[i] as u128 * self.x[i] as u128;
            unreduced.sqr_helper(2 * i, t);

            for j in (i + 1)..P256_LIMBS {
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

impl Add for P256Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self == INFINITY {
            return other.clone();
        } else if other == INFINITY {
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

impl Mul<&[u8]> for P256Point {
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

impl P256Point {
    fn normalize(&self) -> P256Point {
        let z = self.z;
        let z2 = z.sqr() * z;
        let z4 = z2.sqr_n_times(2) * z2;
        let z8 = z4.sqr_n_times(4) * z4;
        let z16 = z8.sqr_n_times(8) * z8;
        let z32 = z16.sqr_n_times(16) * z16;
        let z64 = z32.sqr_n_times(32) * z32;
        let z24 = z16.sqr_n_times(8) * z8;
        let z28 = z24.sqr_n_times(4) * z4;
        let z30 = z28.sqr_n_times(2) * z2;
        let z94 = z64.sqr_n_times(30) * z30;

        let mut zinv = z32;
        zinv = zinv.sqr_n_times(32) * z;
        zinv = zinv.sqr_n_times(96);
        zinv = zinv.sqr_n_times(94) * z94;
        zinv = zinv.sqr();
        zinv = zinv.sqr() * z;

        let zinv2 = zinv.sqr();
        let zinv3 = zinv2 * zinv;

        Self {
            x: self.x * zinv2,
            y: self.y * zinv3,
            z: P256_ONE,
        }
    }

    fn double(&self) -> P256Point {
        if *self == INFINITY {
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
        let m = xx * 3 + P256_A * zz.sqr();
        let t = m.sqr() - s * 2;
        let y3 = m * (s - t) - yyyy * 8;
        let z3 = (y1 + z1).sqr() - yy - zz;

        Self { x: t, y: y3, z: z3 }
    }

    fn shamir(p: &P256Point, q: &P256Point, n: &[u8], m: &[u8]) -> P256Point {
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

#[inline]
fn test_bit(n: &[u8], j: usize) -> bool {
    let byte = j >> 3;
    let bit = j & 0b111;

    ((n[byte] >> bit) & 1) == 1
}

#[pyfunction]
pub fn p256_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = P256_G.mul(&k_bytes).normalize();
    let r_bytes: Vec<u8> = P256Element::into(p.x);

    let k = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(k_bytes));
    let z = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(msg));
    let r = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(&r_bytes));
    let d = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(d_bytes));
    let kinv = k.invert().unwrap();
    let s = kinv * (z + r * d);

    (
        r.retrieve().to_le_bytes().to_vec(),
        s.retrieve().to_le_bytes().to_vec(),
    )
}

#[pyfunction]
pub fn p256_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    let q: P256Point = P256Point {
        x: P256Element::from(qx_bytes),
        y: P256Element::from(qy_bytes),
        z: P256_ONE,
    };
    let z = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(msg));
    let s = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(s_bytes));
    let r = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(r_bytes));
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = P256Point::shamir(
        &P256_G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = P256Element::into(p.x);
    let xq = ConstMontyForm::<P256Q, 4>::new(&U256::from_le_slice(&x_bytes));

    xq == r
}

#[cfg(test)]
#[path = "p256_test.rs"]
mod p256_test;
