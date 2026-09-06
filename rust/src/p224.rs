use std::ops::{Add, Mul, Sub};

use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U256};
use pyo3::prelude::*;

const P224_LIMBS: usize = 7;

struct P224AddResult {
    x: [u32; P224_LIMBS + 1],
}
const P224_P_8: P224AddResult = P224AddResult {
    x: [
        0x00000001, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x0,
    ],
};

struct P224MulResult {
    x: [u32; P224_LIMBS << 1],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct P224Element {
    x: [u32; P224_LIMBS],
}
const P224_ONE: P224Element = P224Element {
    x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
};
const P224_P: P224Element = P224Element {
    x: [
        0x00000001, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    ],
};
const P224_A: P224Element = P224Element {
    x: [
        0xfffffffe, 0xffffffff, 0xffffffff, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff,
    ],
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct P224Point {
    x: P224Element,
    y: P224Element,
    z: P224Element,
}
const P224_G: P224Point = P224Point {
    x: P224Element {
        x: [
            0x115c1d21, 0x343280d6, 0x56c21122, 0x4a03c1d3, 0x321390b9, 0x6bb4bf7f, 0xb70e0cbd,
        ],
    },
    y: P224Element {
        x: [
            0x85007e34, 0x44d58199, 0x5a074764, 0xcd4375a0, 0x4c22dfe6, 0xb5f723fb, 0xbd376388,
        ],
    },
    z: P224_ONE,
};
const INFINITY: P224Point = P224Point {
    x: P224Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    y: P224Element {
        x: [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
    z: P224Element {
        x: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    },
};

const_monty_params!(
    P224Q,
    U256,
    "00000000ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d"
);

impl P224AddResult {
    fn less_than(&self, other: &P224AddResult) -> bool {
        for i in (0..P224_LIMBS + 1).rev() {
            if self.x[i] < other.x[i] {
                return true;
            } else if self.x[i] > other.x[i] {
                return false;
            }
        }

        false
    }

    fn reduce(&mut self) -> P224Element {
        let mut t: i64;
        let mut k: i64;

        while !self.less_than(&P224_P_8) {
            t = self.x[0] as i64 - P224_P_8.x[0] as i64;
            self.x[0] = t as u32;
            k = t >> 32;

            t = self.x[1] as i64 - P224_P_8.x[1] as i64 + k;
            self.x[1] = t as u32;
            k = t >> 32;

            t = self.x[2] as i64 - P224_P_8.x[2] as i64 + k;
            self.x[2] = t as u32;
            k = t >> 32;

            t = self.x[3] as i64 - P224_P_8.x[3] as i64 + k;
            self.x[3] = t as u32;
            k = t >> 32;

            t = self.x[4] as i64 - P224_P_8.x[4] as i64 + k;
            self.x[4] = t as u32;
            k = t >> 32;

            t = self.x[5] as i64 - P224_P_8.x[5] as i64 + k;
            self.x[5] = t as u32;
            k = t >> 32;

            t = self.x[6] as i64 - P224_P_8.x[6] as i64 + k;
            self.x[6] = t as u32;
            k = t >> 32;

            t = self.x[7] as i64 + k;
            self.x[7] = t as u32;
        }

        P224Element {
            x: [
                self.x[0], self.x[1], self.x[2], self.x[3], self.x[4], self.x[5], self.x[6],
            ],
        }
    }
}

impl P224MulResult {
    fn reduce(&self) -> P224Element {
        let mut somewhat_reduced: P224AddResult = P224AddResult {
            x: [0; P224_LIMBS + 1],
        };

        let mut sum: i64 = 0x2 + self.x[0] as i64 - self.x[7] as i64 - self.x[11] as i64;
        somewhat_reduced.x[0] = sum as u32;
        let mut carry: i64 = sum >> 32;

        sum = self.x[1] as i64 - self.x[8] as i64 - self.x[12] as i64 + carry;
        somewhat_reduced.x[1] = sum as u32;
        carry = sum >> 32;

        sum = self.x[2] as i64 - self.x[9] as i64 - self.x[13] as i64 + carry;
        somewhat_reduced.x[2] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[3] as i64 + self.x[7] as i64 + self.x[11] as i64
            - self.x[10] as i64
            + carry;
        somewhat_reduced.x[3] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[4] as i64 + self.x[8] as i64 + self.x[12] as i64
            - self.x[11] as i64
            + carry;
        somewhat_reduced.x[4] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[5] as i64 + self.x[9] as i64 + self.x[13] as i64
            - self.x[12] as i64
            + carry;
        somewhat_reduced.x[5] = sum as u32;
        carry = sum >> 32;

        sum = 0x1fffffffe + self.x[6] as i64 + self.x[10] as i64 - self.x[13] as i64 + carry;
        somewhat_reduced.x[6] = sum as u32;
        carry = sum >> 32;

        somewhat_reduced.x[7] = carry as u32;

        return somewhat_reduced.reduce();
    }

    fn sqr_helper(&mut self, k: usize, t: u64) {
        let lo = t as u32;
        let hi = (t >> 32) as u32;

        let (sumk, ck) = self.x[k].overflowing_add(lo);
        self.x[k] = sumk;

        let (sumk1, ck1a) = self.x[k + 1].overflowing_add(hi);
        let (sumk1, ck1b) = sumk1.overflowing_add(ck as u32);
        self.x[k + 1] = sumk1;

        let mut carry = (ck1a || ck1b) as u32;
        let mut l = k + 2;
        while carry != 0 {
            let (s, c) = self.x[l].overflowing_add(carry);
            self.x[l] = s;
            carry = c as u32;
            l += 1;
        }
    }
}

impl From<&[u8]> for P224Element {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self { x: [0; P224_LIMBS] };

        for i in 0..P224_LIMBS {
            for j in 0..4 {
                result.x[i] |= (x[i * 4 + j] as u32) << (j * 8);
            }
        }

        result
    }
}

impl From<P224Element> for Vec<u8> {
    fn from(x: P224Element) -> Vec<u8> {
        let mut result: [u8; 28] = [0; 28];

        for i in 0..P224_LIMBS {
            for j in 0..4 {
                result[i * 4 + j] = (x.x[i] >> (j * 8)) as u8;
            }
        }

        result.to_vec()
    }
}

impl Add for P224Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut unreduced: P224AddResult = P224AddResult {
            x: [0; P224_LIMBS + 1],
        };
        let mut t: u64;
        let mut k: u64 = 0;

        for i in 0..P224_LIMBS {
            t = self.x[i] as u64 + other.x[i] as u64 + k;
            unreduced.x[i] = t as u32;
            k = t >> 32;
        }

        unreduced.x[P224_LIMBS] = k as u32;
        unreduced.reduce()
    }
}

impl Sub for P224Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut unreduced: P224AddResult = P224AddResult {
            x: [0; P224_LIMBS + 1],
        };
        let mut t: i64;
        let mut k: i64 = 0;

        for i in 0..P224_LIMBS {
            t = ((P224_P.x[i] as i64) << 1) - other.x[i] as i64 + self.x[i] as i64 + k;
            unreduced.x[i] = t as u32;
            k = t >> 32;
        }

        unreduced.x[P224_LIMBS] = k as u32;
        unreduced.reduce()
    }
}

impl Mul for P224Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut unreduced: P224MulResult = P224MulResult {
            x: [0; P224_LIMBS << 1],
        };
        let mut k: usize;
        let mut t: u64;

        for i in 0..P224_LIMBS {
            let mut carry: u64 = 0;

            for j in 0..P224_LIMBS {
                k = i + j;
                t = self.x[i] as u64 * other.x[j] as u64 + unreduced.x[k] as u64 + carry;
                unreduced.x[k] = t as u32;
                carry = t >> 32;
            }

            unreduced.x[i + P224_LIMBS] = carry as u32;
        }

        unreduced.reduce()
    }
}

impl Mul<u32> for P224Element {
    type Output = Self;

    fn mul(self, y: u32) -> Self::Output {
        let mut unreduced: P224MulResult = P224MulResult {
            x: [0; P224_LIMBS << 1],
        };
        let mut t: u64;
        let mut k: u64 = 0;

        for i in 0..P224_LIMBS {
            t = self.x[i] as u64 * y as u64 + unreduced.x[i] as u64 + k;
            unreduced.x[i] = t as u32;
            k = t >> 32;
        }

        unreduced.x[P224_LIMBS] = k as u32;
        unreduced.reduce()
    }
}

impl P224Element {
    fn sqr(&self) -> Self {
        let mut unreduced: P224MulResult = P224MulResult {
            x: [0; P224_LIMBS << 1],
        };
        let mut t: u64;
        let mut k: usize;

        for i in 0..P224_LIMBS {
            t = self.x[i] as u64 * self.x[i] as u64;
            unreduced.sqr_helper(2 * i, t);

            for j in (i + 1)..P224_LIMBS {
                t = self.x[i] as u64 * self.x[j] as u64;
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

impl Add for P224Point {
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

impl Mul<&[u8]> for P224Point {
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

impl P224Point {
    fn normalize(&self) -> P224Point {
        let z = self.z;
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

        Self {
            x: self.x * zinv2,
            y: self.y * zinv3,
            z: P224_ONE,
        }
    }

    fn double(&self) -> P224Point {
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
        let m = xx * 3 + P224_A * zz.sqr();
        let t = m.sqr() - s * 2;
        let y3 = m * (s - t) - yyyy * 8;
        let z3 = (y1 + z1).sqr() - yy - zz;

        Self { x: t, y: y3, z: z3 }
    }

    fn shamir(p: &P224Point, q: &P224Point, n: &[u8], m: &[u8]) -> P224Point {
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

#[inline]
fn monty_form(n: &[u8]) -> ConstMontyForm<P224Q, 4> {
    let mut bytes: [u8; 32] = [0; 32];
    bytes[..n.len()].copy_from_slice(n);

    ConstMontyForm::<P224Q, 4>::new(&U256::from_le_slice(&bytes))
}

#[pyfunction]
pub fn p224_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = P224_G.mul(&k_bytes).normalize();
    let r_bytes: Vec<u8> = P224Element::into(p.x);

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

#[pyfunction]
pub fn p224_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    let q: P224Point = P224Point {
        x: P224Element::from(qx_bytes),
        y: P224Element::from(qy_bytes),
        z: P224_ONE,
    };
    let z = monty_form(msg);
    let s = monty_form(s_bytes);
    let r = monty_form(r_bytes);
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = P224Point::shamir(
        &P224_G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = P224Element::into(p.x);
    let xq = monty_form(&x_bytes);

    xq == r
}

#[cfg(test)]
#[path = "p224_test.rs"]
mod p224_test;
