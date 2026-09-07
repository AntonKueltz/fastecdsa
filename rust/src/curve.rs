use std::ops::{Add, Mul, Sub};

pub trait Curve: Sized {
    type Limbs: AsRef<[u64]> + AsMut<[u64]> + Copy + Default + PartialEq + std::fmt::Debug;
    type Wide: AsRef<[u64]> + AsMut<[u64]> + Copy + Default;
    type Double: AsRef<[u64]> + AsMut<[u64]> + Copy + Default;

    const LIMB_SZ: usize;
    const WIDE_SZ: usize;
    const FIELD_BYTES: usize;

    const P: Field<Self>;
    const P_WIDE: AddResult<Self>;
    const A: Field<Self>;
    const ZERO: Field<Self>;
    const ONE: Field<Self>;
    const G: Point<Self>;
    const INFINITY: Point<Self>;

    fn reduce_add_result(_unreduced: &mut AddResult<Self>) -> Field<Self> {
        let n = Self::WIDE_SZ - 1;
        let p = Self::P_WIDE;

        let mut t: i128;
        let mut k: i128;

        while !_unreduced.less_than(&Self::P_WIDE) {
            let a = _unreduced.x.as_mut();
            let q = p.x.as_ref();
            k = 0;

            for j in 0..n {
                t = a[j] as i128 - q[j] as i128 + k;
                a[j] = t as u64;
                k = t >> 64;
            }

            t = a[n] as i128 + k;
            a[n] = t as u64;
        }

        let mut result = Self::Limbs::default();
        result
            .as_mut()
            .copy_from_slice(&_unreduced.x.as_ref()[..Self::LIMB_SZ]);
        Field { x: result }
    }

    fn reduce_mul_result(_unreduced: &MulResult<Self>) -> Field<Self> {
        todo!()
    }

    fn field_add(op1: &Field<Self>, op2: &Field<Self>) -> Field<Self> {
        let mut unreduced = AddResult::<Self> {
            x: Self::Wide::default(),
        };

        let a = op1.x.as_ref();
        let b = op2.x.as_ref();
        let c = unreduced.x.as_mut();

        let mut t: u128;
        let mut k: u128 = 0;

        for j in 0..Self::LIMB_SZ {
            t = a[j] as u128 + b[j] as u128 + k;
            c[j] = t as u64;
            k = t >> 64;
        }

        c[Self::LIMB_SZ] = k as u64;
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

        c[Self::LIMB_SZ] = k as u64;
        unreduced.reduce()
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
    pub fn less_than(&self, other: &AddResult<C>) -> bool {
        let a = self.x.as_ref();
        let b = other.x.as_ref();

        for i in (0..C::WIDE_SZ).rev() {
            if a[i] != b[i] {
                return a[i] < b[i];
            }
        }

        false
    }

    pub fn reduce(&mut self) -> Field<C> {
        C::reduce_add_result(self)
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

impl<C: Curve> Add for Field<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        C::field_add(&self, &other)
    }
}

impl<C: Curve> Sub for Field<C> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        C::field_sub(&self, &other)
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
        self.x * other.z * other.z == other.x * self.z * self.z
            && self.y * other.z * other.z * other.z == other.y * self.z * self.z * self.z
    }
}

impl<C: Curve> Eq for Point<C> {}

impl<C: Curve> Add for Point<C> {
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

impl<C: Curve> Mul<&[u8]> for Point<C> {
    type Output = Self;

    fn mul(self, n: &[u8]) -> Self::Output {
        if n.len() == 0 {
            return C::INFINITY;
        }

        let mut j = n.len() * 8 - 1;
        while !test_bit(n, j) {
            j -= 1;
        }

        let mut r0: Self = C::INFINITY;
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

impl<C: Curve> Point<C> {
    pub fn is_point_at_infinity(&self) -> bool {
        self.z == C::ZERO
    }

    pub fn normalize(&self) -> Point<C> {
        C::normalize_point(self)
    }

    pub fn double(&self) -> Point<C> {
        if self.is_point_at_infinity() {
            return C::INFINITY;
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
        let m = xx * 3 + C::A * zz.sqr();
        let t = m.sqr() - s * 2;
        let y3 = m * (s - t) - yyyy * 8;
        let z3 = (y1 + z1).sqr() - yy - zz;

        Self { x: t, y: y3, z: z3 }
    }

    pub fn shamir(p: &Point<C>, q: &Point<C>, n: &[u8], m: &[u8]) -> Point<C> {
        let mut j = n.len() * 8 - 1;
        while !test_bit(n, j) && !test_bit(m, j) {
            j -= 1;
        }

        let pq = *p + *q;
        let mut r = C::INFINITY;

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
