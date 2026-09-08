use std::ops::{Add, Mul, Sub};

use crate::scalar::ScalarField;

pub trait Curve: Sized {
    type Limbs: AsRef<[u64]> + AsMut<[u64]> + Copy + Default + PartialEq + std::fmt::Debug;
    type Wide: AsRef<[u64]> + AsMut<[u64]> + Copy + Default;
    type Double: AsRef<[u64]> + AsMut<[u64]> + Copy + Default;

    type Order: ScalarField;

    const LIMB_SZ: usize;
    const WIDE_SZ: usize;
    const FIELD_BYTES: usize;

    const P: Field<Self>;
    const P_WIDE: AddResult<Self>;
    const A: Field<Self>;
    const B: Field<Self>;
    const ZERO: Field<Self>;
    const ONE: Field<Self>;
    const G: Point<Self>;
    const INFINITY: Point<Self>;

    fn reduce_mul_result(_unreduced: &MulResult<Self>) -> Field<Self> {
        todo!()
    }

    fn normalize_point(_point: &Point<Self>) -> Point<Self> {
        todo!()
    }
}

pub struct AddResult<C: Curve> {
    pub x: C::Wide,
}

pub struct MulResult<C: Curve> {
    pub x: C::Double,
}

#[derive(Debug)]
pub struct Field<C: Curve> {
    pub x: C::Limbs,
}

#[derive(Debug)]
pub struct Point<C: Curve> {
    pub x: Field<C>,
    pub y: Field<C>,
    pub z: Field<C>,
}

impl<C: Curve> AddResult<C> {
    pub fn conditional_add_p(&mut self, apply: bool) {
        let mask = 0u64.wrapping_sub(apply as u64);
        let p = C::P_WIDE;
        let a = self.x.as_mut();

        let mut k: u128 = 0;

        for j in 0..C::WIDE_SZ {
            let q = p.x.as_ref();
            let t = a[j] as u128 + (q[j] & mask) as u128 + k;
            a[j] = t as u64;
            k = t >> 64;
        }
    }

    pub fn conditional_sub_p(&mut self) {
        let n = C::WIDE_SZ;
        let a = self.x.as_mut();
        let p = C::P_WIDE;

        let mut diff = C::Wide::default();
        let d = diff.as_mut();

        let mut k: i128 = 0;
        for j in 0..n {
            let q = p.x.as_ref();
            let t = a[j] as i128 - q[j] as i128 + k;
            d[j] = t as u64;
            k = t >> 64;
        }

        let mask = k as u64;

        for j in 0..n {
            a[j] = (a[j] & mask) | (d[j] & !mask);
        }
    }
}

impl<C: Curve> Sub for AddResult<C> {
    type Output = (Self, bool);

    fn sub(self, other: AddResult<C>) -> (AddResult<C>, bool) {
        let mut result = AddResult::<C> { x: C::Wide::default() };
        let x = self.x.as_ref();
        let y = other.x.as_ref();
        let c = result.x.as_mut();

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..C::WIDE_SZ {
            t = x[j] as i128 - y[j] as i128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        (result, k != 0)
    }
}

impl<C: Curve> MulResult<C> {
    pub fn reduce(&self) -> Field<C> {
        C::reduce_mul_result(self)
    }

    pub fn sqr_helper(&mut self, k: usize, t: u128) {
        let a = self.x.as_mut();

        let lo = t as u64;
        let hi = (t >> 64) as u64;

        let (sumk, ck) = a[k].overflowing_add(lo);
        a[k] = sumk;

        let (sumk1, ck1a) = a[k + 1].overflowing_add(hi);
        let (sumk1, ck1b) = sumk1.overflowing_add(ck as u64);
        a[k + 1] = sumk1;

        let mut carry = (ck1a || ck1b) as u64;
        let mut l = k + 2;
        while carry != 0 {
            let (s, c) = a[l].overflowing_add(carry);
            a[l] = s;
            carry = c as u64;
            l += 1;
        }
    }
}

impl<C: Curve> Clone for Field<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Curve> Copy for Field<C> {}

impl<C: Curve> PartialEq for Field<C> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl<C: Curve> From<&[u8]> for Field<C> {
    fn from(x: &[u8]) -> Self {
        let mut result: Self = Self {
            x: C::Limbs::default(),
        };
        let a = result.x.as_mut();

        for i in 0..C::FIELD_BYTES {
            let j = i >> 3;
            let k = i & 0b111;
            a[j] |= (x[i] as u64) << (k * 8);
        }

        result
    }
}

impl<C: Curve> From<Field<C>> for Vec<u8> {
    fn from(x: Field<C>) -> Vec<u8> {
        let mut result = vec![0; C::FIELD_BYTES];
        let a = x.x.as_ref();

        for i in 0..C::FIELD_BYTES {
            let j = i >> 3;
            let k = i & 0b111;
            result[i] = (a[j] >> (k * 8)) as u8;
        }

        result
    }
}

impl<C: Curve> From<AddResult<C>> for Field<C> {
    fn from (x: AddResult<C>) -> Field<C> {
        let mut result = C::Limbs::default();
        result
            .as_mut()
            .copy_from_slice(&x.x.as_ref()[..C::LIMB_SZ]);
        Field { x: result }
    }
}

impl<C: Curve> Add for Field<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut unreduced = AddResult::<C> {
            x: C::Wide::default(),
        };

        let a = self.x.as_ref();
        let b = other.x.as_ref();
        let c = unreduced.x.as_mut();

        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..C::LIMB_SZ {
            t = a[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        if C::WIDE_SZ != C::LIMB_SZ {
            c[C::LIMB_SZ] = k as u64;
        }

        unreduced.conditional_sub_p();
        Field::<C>::from(unreduced)
    }
}

impl<C: Curve> Sub for Field<C> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut unreduced = AddResult::<C> {
            x: C::Wide::default(),
        };

        let a = self.x.as_ref();
        let b = other.x.as_ref();
        let c = unreduced.x.as_mut();
        let p = C::P;

        let mut t: i128;
        let mut k: i128 = 0;

        for j in 0..C::LIMB_SZ {
            let q = p.x.as_ref();
            t = ((q[j] as i128) << 1) - b[j] as i128 + a[j] as i128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        if C::WIDE_SZ != C::LIMB_SZ {
            c[C::LIMB_SZ] = k as u64;
        }

        unreduced.conditional_sub_p();
        unreduced.conditional_sub_p();
        Field::<C>::from(unreduced)
    }
}

impl<C: Curve> Mul for Field<C> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut unreduced = MulResult::<C> {
            x: C::Double::default(),
        };

        let a = self.x.as_ref();
        let b = other.x.as_ref();
        let c = unreduced.x.as_mut();

        let mut k: usize;
        let mut t: u128;

        for i in 0..C::LIMB_SZ {
            let mut carry: u128 = 0;

            for j in 0..C::LIMB_SZ {
                k = i + j;
                t = a[i] as u128 * b[j] as u128 + c[k] as u128 + carry;
                c[k] = t as u64;
                carry = t >> 64;
            }

            c[i + C::LIMB_SZ] = carry as u64;
        }

        unreduced.reduce()
    }
}

impl<C: Curve> Mul<u64> for Field<C> {
    type Output = Self;

    fn mul(self, y: u64) -> Self::Output {
        let mut unreduced = MulResult::<C> {
            x: C::Double::default(),
        };

        let a = self.x.as_ref();
        let c = unreduced.x.as_mut();

        let mut t: u128;
        let mut k: u128 = 0;

        for i in 0..C::LIMB_SZ {
            t = a[i] as u128 * y as u128 + c[i] as u128 + k;
            c[i] = t as u64;
            k = t >> 64;
        }

        c[C::LIMB_SZ] = k as u64;
        unreduced.reduce()
    }
}

impl<C: Curve> Field<C> {
    pub fn scale_wide(&self, y: u64) -> AddResult<C> {
        let mut result = AddResult::<C> { x: C::Wide::default() };
        let a = self.x.as_ref();
        let c = result.x.as_mut();

        let mut t: u128;
        let mut k: u128 = 0;

        for i in 0..C::LIMB_SZ {
            t = a[i] as u128 * y as u128 + k;
            c[i] = t as u64;
            k = t >> 64;
        }

        c[C::LIMB_SZ] = k as u64;
        result
    }

    pub fn sqr(&self) -> Self {
        let mut unreduced = MulResult::<C> {
            x: C::Double::default(),
        };

        let a = self.x.as_ref();

        let mut t: u128;
        let mut k: usize;

        for i in 0..C::LIMB_SZ {
            t = a[i] as u128 * a[i] as u128;
            unreduced.sqr_helper(2 * i, t);

            for j in (i + 1)..C::LIMB_SZ {
                t = a[i] as u128 * a[j] as u128;
                k = i + j;

                unreduced.sqr_helper(k, t);
                unreduced.sqr_helper(k, t);
            }
        }

        unreduced.reduce()
    }

    pub fn sqr_n_times(&self, n: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.sqr();
        }
        result
    }
}

impl<C: Curve> Clone for Point<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Curve> Copy for Point<C> {}

impl<C: Curve> PartialEq for Point<C> {
    fn eq(&self, other: &Self) -> bool {
        self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
    }
}

impl<C: Curve> Eq for Point<C> {}

impl<C: Curve> Add for Point<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let x1 = self.x;
        let x2 = other.x;
        let y1 = self.y;
        let y2 = other.y;
        let z1 = self.z;
        let z2 = other.z;
        let b3 = C::B * 3;

        // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective-3.html#addition-add-2015-rcb
        let t0 = x1 * x2;
        let t1 = y1 * y2;
        let t2 = z1 * z2;
        let t3 = x1 + y1;
        let t4 = x2 + y2;
        let t3 = t3 * t4;
        let t4 = t0 + t1;
        let t3 = t3 - t4;
        let t4 = x1 + z1;
        let t5 = x2 + z2;
        let t4 = t4 * t5;
        let t5 = t0 + t2;
        let t4 = t4 - t5;
        let t5 = y1 + z1;
        let x3 = y2 + z2;
        let t5 = t5 * x3;
        let x3 = t1 + t2;
        let t5 = t5 - x3;
        let z3 = C::A * t4;
        let x3 = b3 * t2;
        let z3 = x3 + z3;
        let x3 = t1 - z3;
        let z3 = t1 + z3;
        let y3 = x3 * z3;
        let t1 = t0 + t0;
        let t1 = t1 + t0;
        let t2 = C::A * t2;
        let t4 = b3 * t4;
        let t1 = t1 + t2;
        let t2 = t0 - t2;
        let t2 = C::A * t2;
        let t4 = t4 + t2;
        let t0 = t1 * t4;
        let y3 = y3 + t0;
        let t0 = t5 * t4;
        let x3 = t3 * x3;
        let x3 = x3 - t0;
        let t0 = t3 * t1;
        let z3 = t5 * z3;
        let z3 = z3 + t0;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

impl<C: Curve> Mul<&[u8]> for Point<C> {
    type Output = Self;

    fn mul(self, n: &[u8]) -> Self::Output {
        let mut padded = vec![0u8; C::FIELD_BYTES];
        padded[..n.len()].copy_from_slice(n);

        let j = C::FIELD_BYTES * 8 - 1;
        let mut r0: Self = C::INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..j + 1).rev() {
            if test_bit(&padded, i) {
                r0 = r0 + r1;
                r1 = r1 + r1;
            } else {
                r1 = r1 + r0;
                r0 = r0 + r0;
            }
        }

        r0
    }
}

impl<C: Curve> Point<C> {
    pub fn is_point_at_infinity(&self) -> bool {
        self.z == C::ZERO
    }

    pub fn normalize(&self) -> Point<C> {
        C::normalize_point(self)
    }

    pub fn shamir(p: &Point<C>, q: &Point<C>, n: &[u8], m: &[u8]) -> Point<C> {
        let mut j = n.len() * 8 - 1;
        while !test_bit(n, j) && !test_bit(m, j) {
            j -= 1;
        }

        let pq = *p + *q;
        let mut r = C::INFINITY;

        for i in (0..j + 1).rev() {
            r = r + r;

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
