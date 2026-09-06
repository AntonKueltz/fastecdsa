use std::ops::{Add, Mul, Sub};

use crypto_bigint::{const_monty_params, modular::ConstMontyForm, U192};
use pyo3::prelude::*;

const P192_LIMBS: usize = 3;

struct P192AddResult {
    x: [u64; P192_LIMBS + 1],
}
const P192_P_4: P192AddResult = P192AddResult {
    x: [
        0xffffffffffffffff,
        0xfffffffffffffffe,
        0xffffffffffffffff,
        0x0,
    ],
};

struct P192MulResult {
    x: [u64; P192_LIMBS << 1],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct P192Element {
    x: [u64; P192_LIMBS],
}
const P192_ONE: P192Element = P192Element { x: [0x1, 0x0, 0x0] };
const P192_P: P192Element = P192Element {
    x: [0xffffffffffffffff, 0xfffffffffffffffe, 0xffffffffffffffff],
};
const P192_A: P192Element = P192Element {
    x: [0xfffffffffffffffc, 0xfffffffffffffffe, 0xffffffffffffffff],
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct P192Point {
    x: P192Element,
    y: P192Element,
    z: P192Element,
}
const P192_G: P192Point = P192Point {
    x: P192Element {
        x: [0xf4ff0afd82ff1012, 0x7cbf20eb43a18800, 0x188da80eb03090f6],
    },
    y: P192Element {
        x: [0x73f977a11e794811, 0x631011ed6b24cdd5, 0x07192b95ffc8da78],
    },
    z: P192_ONE,
};
const INFINITY: P192Point = P192Point {
    x: P192Element { x: [0x0, 0x0, 0x0] },
    y: P192Element { x: [0x1, 0x0, 0x0] },
    z: P192Element { x: [0x0, 0x0, 0x0] },
};

const_monty_params!(
    P192Q,
    U192,
    "ffffffffffffffffffffffff99def836146bc9b1b4d22831"
);

impl P192AddResult {
    fn less_than(&self, other: &P192AddResult) -> bool {
        for i in (0..P192_LIMBS + 1).rev() {
            if self.x[i] < other.x[i] {
                return true;
            } else if self.x[i] > other.x[i] {
                return false;
            }
        }

        false
    }

    fn reduce(&mut self) -> P192Element {
        let mut t: i128;
        let mut k: i128;

        while !self.less_than(&P192_P_4) {
            t = self.x[0] as i128 - P192_P_4.x[0] as i128;
            self.x[0] = t as u64;
            k = t >> 64;

            t = self.x[1] as i128 - P192_P_4.x[1] as i128 + k;
            self.x[1] = t as u64;
            k = t >> 64;

            t = self.x[2] as i128 - P192_P_4.x[2] as i128 + k;
            self.x[2] = t as u64;
            k = t >> 64;

            t = self.x[3] as i128 + k;
            self.x[3] = t as u64;
        }

        P192Element {
            x: [self.x[0], self.x[1], self.x[2]],
        }
    }
}

impl P192MulResult {
    fn reduce(&self) -> P192Element {
        let mut somewhat_reduced: P192AddResult = P192AddResult { x: [0; P192_LIMBS + 1] };

        let mut sum: u128 = self.x[0] as u128 + self.x[3] as u128 + self.x[5] as u128;
        somewhat_reduced.x[0] = sum as u64;
        let mut carry: u64 = (sum >> 64) as u64;

        sum = self.x[1] as u128
            + self.x[3] as u128
            + self.x[4] as u128
            + self.x[5] as u128
            + carry as u128;
        somewhat_reduced.x[1] = sum as u64;
        carry = (sum >> 64) as u64;

        sum = self.x[2] as u128 + self.x[4] as u128 + self.x[5] as u128 + carry as u128;
        somewhat_reduced.x[2] = sum as u64;
        carry = (sum >> 64) as u64;
        somewhat_reduced.x[3] = carry;

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

impl From<&[u8]> for P192Element {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self { x: [0; P192_LIMBS] };

        for i in 0..P192_LIMBS {
            for j in 0..8 {
                result.x[i] |= (x[i * 8 + j] as u64) << (j * 8);
            }
        }

        result
    }
}

impl From<P192Element> for Vec<u8> {
    fn from(x: P192Element) -> Vec<u8> {
        let mut result: [u8; 24] = [0; 24];

        for i in 0..P192_LIMBS {
            for j in 0..8 {
                result[i * 8 + j] = (x.x[i] >> (j * 8)) as u8;
            }
        }

        result.to_vec()
    }
}

impl Add for P192Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut unreduced: P192AddResult = P192AddResult { x: [0; P192_LIMBS + 1] };

        let mut t = self.x[0] as u128 + other.x[0] as u128;
        unreduced.x[0] = t as u64;
        let mut k = t >> 64;

        t = self.x[1] as u128 + other.x[1] as u128 + k;
        unreduced.x[1] = t as u64;
        k = t >> 64;

        t = self.x[2] as u128 + other.x[2] as u128 + k;
        unreduced.x[2] = t as u64;
        k = t >> 64;

        unreduced.x[3] = k as u64;

        unreduced.reduce()
    }
}

impl Sub for P192Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut unreduced: P192AddResult = P192AddResult { x: [0; P192_LIMBS + 1] };
        let mut t: i128;
        let mut k: i128;

        t = ((P192_P.x[0] as i128) << 1) - other.x[0] as i128 + self.x[0] as i128;
        unreduced.x[0] = t as u64;
        k = t >> 64;

        t = ((P192_P.x[1] as i128) << 1) - other.x[1] as i128 + self.x[1] as i128 + k;
        unreduced.x[1] = t as u64;
        k = t >> 64;

        t = ((P192_P.x[2] as i128) << 1) - other.x[2] as i128 + self.x[2] as i128 + k;
        unreduced.x[2] = t as u64;
        unreduced.x[3] = (t >> 64) as u64;

        unreduced.reduce()
    }
}

impl Mul for P192Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut unreduced: P192MulResult = P192MulResult { x: [0; P192_LIMBS << 1] };
        let mut k: usize;
        let mut t: u128;

        for i in 0..P192_LIMBS {
            let mut carry: u128 = 0;

            for j in 0..P192_LIMBS {
                k = i + j;
                t = self.x[i] as u128 * other.x[j] as u128 + unreduced.x[k] as u128 + carry;
                unreduced.x[k] = t as u64;
                carry = t >> 64;
            }

            unreduced.x[i + P192_LIMBS] = carry as u64;
        }

        unreduced.reduce()
    }
}

impl Mul<u64> for P192Element {
    type Output = Self;

    fn mul(self, y: u64) -> Self::Output {
        let mut unreduced: P192MulResult = P192MulResult { x: [0; P192_LIMBS << 1] };
        let mut t: u128;
        let mut k: u128 = 0;

        for i in 0..P192_LIMBS {
            t = self.x[i] as u128 * y as u128 + unreduced.x[i] as u128 + k;
            unreduced.x[i] = t as u64;
            k = t >> 64;
        }
        unreduced.x[P192_LIMBS] = k as u64;

        unreduced.reduce()
    }
}

impl P192Element {
    fn sqr(&self) -> Self {
        let mut unreduced: P192MulResult = P192MulResult { x: [0; P192_LIMBS << 1] };
        let mut t: u128;
        let mut k: usize;

        for i in 0..P192_LIMBS {
            t = self.x[i] as u128 * self.x[i] as u128;
            unreduced.sqr_helper(2 * i, t);

            for j in (i + 1)..P192_LIMBS {
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

impl Add for P192Point {
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

impl Mul<&[u8]> for P192Point {
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

impl P192Point {
    fn normalize(&self) -> P192Point {
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

        let mut zinv = z127.sqr();
        zinv = zinv.sqr_n_times(62) * z62;
        zinv = zinv.sqr().sqr() * z;

        let zinv2 = zinv.sqr();
        let zinv3 = zinv2 * zinv;

        Self {
            x: self.x * zinv2,
            y: self.y * zinv3,
            z: P192_ONE,
        }
    }

    fn double(&self) -> P192Point {
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
        let m = xx * 3 + P192_A * zz.sqr();
        let t = m.sqr() - s * 2;
        let y3 = m * (s - t) - yyyy * 8;
        let z3 = (y1 + z1).sqr() - yy - zz;

        Self { x: t, y: y3, z: z3 }
    }

    fn shamir(p: &P192Point, q: &P192Point, n: &[u8], m: &[u8]) -> P192Point {
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
pub fn p192_scale_point(n: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = P192_G.mul(n).normalize();

    (P192Element::into(p.x), P192Element::into(p.y))
}

#[pyfunction]
pub fn p192_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let p = P192_G.mul(&k_bytes).normalize();
    let r_bytes: Vec<u8> = P192Element::into(p.x);

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

#[pyfunction]
pub fn p192_verify(
    r_bytes: &[u8],
    s_bytes: &[u8],
    msg: &[u8],
    qx_bytes: &[u8],
    qy_bytes: &[u8],
) -> bool {
    let q: P192Point = P192Point {
        x: P192Element::from(qx_bytes),
        y: P192Element::from(qy_bytes),
        z: P192_ONE,
    };
    let z = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(msg));
    let s = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(s_bytes));
    let r = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(r_bytes));
    let sinv = s.invert().unwrap();
    let u1 = z * sinv;
    let u2 = r * sinv;

    let p = P192Point::shamir(
        &P192_G,
        &q,
        &u1.retrieve().to_le_bytes().to_vec(),
        &u2.retrieve().to_le_bytes().to_vec(),
    )
    .normalize();
    let x_bytes: Vec<u8> = P192Element::into(p.x);
    let xq = ConstMontyForm::<P192Q, 3>::new(&U192::from_le_slice(&x_bytes));

    xq == r
}

#[cfg(test)]
#[path = "p192_test.rs"]
mod p192_test;
