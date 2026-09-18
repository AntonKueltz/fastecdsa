use std::ops::{Add, Mul};

use num_bigint::BigUint;

use crate::comb::Comb;
use crate::scalar::ScalarField;

pub mod brainpoolp160r1;
pub mod brainpoolp192r1;
pub mod brainpoolp224r1;
pub mod brainpoolp256r1;
pub mod brainpoolp320r1;
pub mod brainpoolp384r1;
pub mod brainpoolp512r1;

pub trait BrainpoolCurve: Sized + 'static {
    type CurveField: ScalarField;
    type GroupField: ScalarField;

    const BITS: u32;
    const ONE: Self::CurveField;

    const A: Self::CurveField;
    const A_T: Self::CurveField;
    const B: Self::CurveField;
    const B_T: Self::CurveField;

    const Z2: Self::CurveField;
    const Z3: Self::CurveField;
    const Z2_INV: Self::CurveField;
    const Z3_INV: Self::CurveField;

    const G: BrainpoolPoint<Self>;
    const G_T: BrainpoolPoint<Self>;
    const INFINITY: BrainpoolPoint<Self>;

    fn sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let k = Self::GroupField::from_le_bytes(k_bytes);
        assert!(!k.is_zero());

        let p: BrainpoolPoint<Self> = match Self::comb() {
            Some(x) => x.mul(k_bytes).from_twist().normalize(),
            None => (Self::G_T * k).from_twist().normalize(),
        };
        let z = Self::GroupField::from_le_bytes(msg);
        let r = Self::GroupField::from_le_bytes(&p.x.to_le_bytes());
        assert!(!r.is_zero());
        let d = Self::GroupField::from_le_bytes(d_bytes);

        let kinv = k.invert();
        let s = kinv * (z + r * d);
        assert!(!s.is_zero());

        (r.to_le_bytes(), s.to_le_bytes())
    }

    fn verify(
        r_bytes: &[u8],
        s_bytes: &[u8],
        msg: &[u8],
        qx_bytes: &[u8],
        qy_bytes: &[u8],
    ) -> bool {
        let (Some(r), Some(s)) = (
            Self::GroupField::from_le_bytes_checked(r_bytes),
            Self::GroupField::from_le_bytes_checked(s_bytes),
        ) else {
            return false;
        };

        let q = (BrainpoolPoint::<Self> {
            x: Self::CurveField::from_le_bytes(qx_bytes),
            y: Self::CurveField::from_le_bytes(qy_bytes),
            z: Self::ONE,
        })
        .to_twist();
        let z = Self::GroupField::from_le_bytes(msg);

        let sinv = s.invert();
        let u1 = sinv * z;
        let u2 = sinv * r;

        let p = BrainpoolPoint::<Self>::shamir(&Self::G_T, &q, u1, u2)
            .from_twist()
            .normalize();
        let xq = Self::GroupField::from_le_bytes(&p.x.to_le_bytes());

        xq.eq(&r)
    }

    fn evaluate(x_bytes: &[u8]) -> BigUint {
        let x = Self::CurveField::from_le_bytes(x_bytes);
        let rhs = x * x * x + Self::A * x + Self::B;

        BigUint::from_bytes_le(&rhs.to_le_bytes())
    }

    fn point_from_affine(x_bytes: &[u8], y_bytes: &[u8]) -> BrainpoolPoint<Self> {
        let x = Self::CurveField::from_le_bytes(x_bytes);
        let y = Self::CurveField::from_le_bytes(y_bytes);

        BrainpoolPoint::<Self> {
            x: x,
            y: y,
            z: Self::ONE,
        }
    }

    fn scalar_from_le_bytes(x_bytes: &[u8]) -> Self::GroupField {
        Self::GroupField::from_le_bytes(x_bytes)
    }

    fn comb() -> Option<&'static Comb<BrainpoolPoint<Self>>> {
        None
    }
}

pub struct BrainpoolPoint<C: BrainpoolCurve> {
    pub x: C::CurveField,
    pub y: C::CurveField,
    pub z: C::CurveField,
}

impl<C: BrainpoolCurve> Clone for BrainpoolPoint<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: BrainpoolCurve> Copy for BrainpoolPoint<C> {}

impl<C: BrainpoolCurve> PartialEq for BrainpoolPoint<C> {
    fn eq(&self, other: &Self) -> bool {
        self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
    }
}

impl<C: BrainpoolCurve> Add for BrainpoolPoint<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x1 = self.x;
        let x2 = rhs.x;
        let y1 = self.y;
        let y2 = rhs.y;
        let z1 = self.z;
        let z2 = rhs.z;

        let t0 = x1 * x2;
        let t1 = y1 * y2;
        let t2 = z1 * z2;
        let t3 = x1 + y1;
        let t4 = x2 + y2;
        let t3 = t3 * t4;
        let t4 = t0 + t1;
        let t3 = t3 - t4;
        let t4 = y1 + z1;
        let x3 = y2 + z2;
        let t4 = t4 * x3;
        let x3 = t1 + t2;
        let t4 = t4 - x3;
        let x3 = x1 + z1;
        let y3 = x2 + z2;
        let x3 = x3 * y3;
        let y3 = t0 + t2;
        let y3 = x3 - y3;
        let z3 = C::B_T * t2;
        let x3 = y3 - z3;
        let z3 = x3 + x3;
        let x3 = x3 + z3;
        let z3 = t1 - x3;
        let x3 = t1 + x3;
        let y3 = C::B_T * y3;
        let t1 = t2 + t2;
        let t2 = t1 + t2;
        let y3 = y3 - t2;
        let y3 = y3 - t0;
        let t1 = y3 + y3;
        let y3 = t1 + y3;
        let t1 = t0 + t0;
        let t0 = t1 + t0;
        let t0 = t0 - t2;
        let t1 = t4 * y3;
        let t2 = t0 * y3;
        let y3 = x3 * z3;
        let y3 = y3 + t2;
        let x3 = t3 * x3;
        let x3 = x3 - t1;
        let z3 = t4 * z3;
        let t1 = t3 * t0;
        let z3 = z3 + t1;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}

impl<C: BrainpoolCurve> Mul<C::GroupField> for BrainpoolPoint<C> {
    type Output = Self;

    fn mul(self, rhs: C::GroupField) -> Self::Output {
        let mut r0: Self = C::INFINITY;
        let mut r1: Self = self.clone();

        for i in (0..C::BITS).rev() {
            if rhs.test_bit(i) {
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

impl<C: BrainpoolCurve> BrainpoolPoint<C> {
    pub fn is_point_at_infinity(&self) -> bool {
        self.z.is_zero()
    }

    pub fn normalize(&self) -> Self {
        if self.is_point_at_infinity() {
            return C::INFINITY;
        }

        let zinv = self.z.invert();

        Self {
            x: self.x * zinv,
            y: self.y * zinv,
            z: C::ONE,
        }
    }

    pub fn to_twist(&self) -> Self {
        if self.is_point_at_infinity() {
            return C::INFINITY;
        }

        Self {
            x: self.x * C::Z2,
            y: self.y * C::Z3,
            z: self.z,
        }
    }

    pub fn from_twist(&self) -> Self {
        if self.is_point_at_infinity() {
            return C::INFINITY;
        }

        Self {
            x: self.x * C::Z2_INV,
            y: self.y * C::Z3_INV,
            z: self.z,
        }
    }

    pub fn double(&self) -> Self {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let t0 = x1.sqr();
        let t1 = y1.sqr();
        let t2 = z1.sqr();
        let t3 = x1 * y1;
        let t3 = t3 + t3;
        let z3 = x1 * z1;
        let z3 = z3 + z3;
        let y3 = C::B_T * t2;
        let y3 = y3 - z3;
        let x3 = y3 + y3;
        let y3 = x3 + y3;
        let x3 = t1 - y3;
        let y3 = t1 + y3;
        let y3 = x3 * y3;
        let x3 = x3 * t3;
        let t3 = t2 + t2;
        let t2 = t2 + t3;
        let z3 = C::B_T * z3;
        let z3 = z3 - t2;
        let z3 = z3 - t0;
        let t3 = z3 + z3;
        let z3 = z3 + t3;
        let t3 = t0 + t0;
        let t0 = t3 + t0;
        let t0 = t0 - t2;
        let t0 = t0 * z3;
        let y3 = y3 + t0;
        let t0 = y1 * z1;
        let t0 = t0 + t0;
        let z3 = t0 * z3;
        let x3 = x3 - z3;
        let z3 = t0 * t1;
        let z3 = z3 + z3;
        let z3 = z3 + z3;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn shamir(p: &Self, q: &Self, n: C::GroupField, m: C::GroupField) -> Self {
        let pq = *p + *q;
        let mut r = C::INFINITY;

        for i in (0..C::BITS).rev() {
            r = r.double();

            if n.test_bit(i) && m.test_bit(i) {
                r = r + pq;
            } else if n.test_bit(i) {
                r = r + *p;
            } else if m.test_bit(i) {
                r = r + *q;
            }
        }

        r
    }
}
