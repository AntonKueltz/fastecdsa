use std::cmp::max;
use std::ops::{Add, Mul};
use std::sync::Arc;

use crypto_bigint::modular::{BoxedMontyForm, BoxedMontyParams};
use crypto_bigint::{BoxedUint, Encoding, Integer, Limb, Resize};
use crypto_primes::{Flavor, is_prime};
use num_bigint::BigUint;

#[derive(Clone, PartialEq, Debug)]
pub struct GenericCurve {
    pub name: String,

    pub p: BoxedUint,
    pub a: BoxedUint,
    pub b: BoxedUint,
    pub q: BoxedUint,

    field_params: BoxedMontyParams,
    scalar_params: BoxedMontyParams,

    gx_monty: BoxedMontyForm,
    gy_monty: BoxedMontyForm,

    a_monty: BoxedMontyForm,
    b_monty: BoxedMontyForm,
    b3_monty: BoxedMontyForm,

    zero_monty: BoxedMontyForm,
    one_monty: BoxedMontyForm,
}

#[derive(Clone)]
pub struct GenericPoint {
    pub x: BoxedMontyForm,
    pub y: BoxedMontyForm,
    pub z: BoxedMontyForm,
    pub curve: Arc<GenericCurve>,
}

#[derive(Debug)]
pub enum CurveError {
    EvenModulus,
    PNotPrime,
    SingularCurve,
    PointNotOnCurve,
    QNotPrime,
}

impl GenericCurve {
    pub fn new(
        name: String,
        p: BoxedUint,
        a: BoxedUint,
        b: BoxedUint,
        q: BoxedUint,
        gx: BoxedUint,
        gy: BoxedUint,
    ) -> Result<Self, CurveError> {
        if p.is_even().into() {
            return Err(CurveError::EvenModulus);
        }

        if p.is_even().into() {
            return Err(CurveError::EvenModulus);
        }
        if !is_prime(Flavor::Any, &p) {
            return Err(CurveError::PNotPrime);
        }

        let bits = p.bits();
        let two = BoxedUint::from(Limb::from(2u32)).resize(bits);
        let three = BoxedUint::from(Limb::from(3u32)).resize(bits);
        let four = BoxedUint::from(Limb::from(4u32)).resize(bits);
        let twenty_seven = BoxedUint::from(Limb::from(27u32)).resize(bits);

        let field_params = BoxedMontyParams::new(p.to_odd().unwrap());
        let a_m = BoxedMontyForm::new(a.clone(), &field_params);
        let b_m = BoxedMontyForm::new(b.clone(), &field_params);
        let b3_m = b_m.double() + &b_m;

        let disc = a_m.pow(&three) * BoxedMontyForm::new(four, &field_params)
            + b_m.pow(&two) * BoxedMontyForm::new(twenty_seven, &field_params);
        if disc.retrieve().is_zero().into() {
            return Err(CurveError::SingularCurve);
        }

        let gx_monty = BoxedMontyForm::new(gx.clone(), &field_params);
        let gy_monty = BoxedMontyForm::new(gy.clone(), &field_params);
        let lhs = gy_monty.square();
        let rhs = gx_monty.pow(&three) + &a_m * &gx_monty + &b_m;
        if lhs.retrieve() != rhs.retrieve() {
            return Err(CurveError::PointNotOnCurve);
        }

        if !is_prime(Flavor::Any, &q) {
            return Err(CurveError::QNotPrime);
        }

        let scalar_params = BoxedMontyParams::new(q.to_odd().unwrap());
        Ok(Self {
            name,
            p,
            a,
            b,
            q,
            field_params: field_params.clone(),
            scalar_params,
            gx_monty,
            gy_monty,
            a_monty: a_m,
            b_monty: b_m,
            b3_monty: b3_m,
            zero_monty: BoxedMontyForm::new(
                BoxedUint::from(Limb::from(0u32)).resize(bits),
                &field_params,
            ),
            one_monty: BoxedMontyForm::new(
                BoxedUint::from(Limb::from(1u32)).resize(bits),
                &field_params,
            ),
        })
    }

    pub fn bytes_to_field(&self, x: &[u8]) -> BoxedMontyForm {
        let xbits = x.len() * 8;
        let pbits = self.p.bits();

        if xbits < pbits as usize {
            BoxedMontyForm::new(
                BoxedUint::from_le_bytes(x.into()).resize(pbits),
                &self.field_params,
            )
        } else {
            BoxedMontyForm::new(BoxedUint::from_le_bytes(x.into()), &self.field_params)
        }
    }

    pub fn bytes_to_scalar(&self, x: &[u8]) -> BoxedMontyForm {
        let xbits = x.len() * 8;
        let qbits = self.p.bits();

        if xbits < qbits as usize {
            BoxedMontyForm::new(
                BoxedUint::from_le_bytes(x.into()).resize(qbits),
                &self.scalar_params,
            )
        } else {
            BoxedMontyForm::new(BoxedUint::from_le_bytes(x.into()), &self.scalar_params)
        }
    }

    pub fn generator(self: &Arc<Self>) -> GenericPoint {
        GenericPoint {
            x: self.gx_monty.clone(),
            y: self.gy_monty.clone(),
            z: self.one_monty.clone(),
            curve: self.clone(),
        }
    }

    pub fn is_point_on_curve(self: &Arc<Self>, point: &GenericPoint) -> bool {
        let y = &point.y;
        let x = &point.x;
        let lhs = y * y;
        let rhs = x * x * x + &self.a_monty * x + &self.b_monty;

        lhs == rhs
    }

    pub fn evaluate(self: &Arc<Self>, x_bytes: &[u8]) -> BigUint {
        let x = &self.bytes_to_field(x_bytes);
        let r = x * x * x + &self.a_monty * x + &self.b_monty;

        BigUint::from_bytes_le(&r.retrieve().to_le_bytes())
    }

    pub fn point_from_affine(self: &Arc<Self>, x_bytes: &[u8], y_bytes: &[u8]) -> GenericPoint {
        GenericPoint {
            x: self.bytes_to_field(x_bytes),
            y: self.bytes_to_field(y_bytes),
            z: self.one_monty.clone(),
            curve: self.clone(),
        }
    }

    pub fn sign(
        self: &Arc<Self>,
        msg: &[u8],
        d_bytes: &[u8],
        k_bytes: &[u8],
    ) -> (Vec<u8>, Vec<u8>) {
        let p = (self.generator() * k_bytes).normalize();

        let k = self.bytes_to_scalar(k_bytes);
        assert!(!k.is_zero().to_bool());
        let z = self.bytes_to_scalar(msg);
        let r = &BoxedMontyForm::new(p.x.retrieve(), &self.scalar_params);
        assert!(!r.is_zero().to_bool());
        let d = self.bytes_to_scalar(d_bytes);

        let kinv = k.invert().unwrap();
        let s = kinv.mul(z + r * d);
        assert!(!s.is_zero().to_bool());

        (
            r.retrieve().to_le_bytes().to_vec(),
            s.retrieve().to_le_bytes().to_vec(),
        )
    }

    pub fn verify(
        self: &Arc<Self>,
        r_bytes: &[u8],
        s_bytes: &[u8],
        msg: &[u8],
        qx_bytes: &[u8],
        qy_bytes: &[u8],
    ) -> bool {
        let s = self.bytes_to_scalar(s_bytes);
        let r = &self.bytes_to_scalar(r_bytes);
        if r.is_zero().to_bool() || s.is_zero().to_bool() {
            return false;
        }

        let q = GenericPoint {
            x: self.bytes_to_field(qx_bytes),
            y: self.bytes_to_field(qy_bytes),
            z: self.one_monty.clone(),
            curve: self.clone(),
        };
        let z = self.bytes_to_scalar(msg);

        let sinv = &s.invert().unwrap();
        let u1 = z * sinv;
        let u2 = r * sinv;

        let p = GenericPoint::shamir(
            &self.generator(),
            &q,
            &u1.retrieve().to_le_bytes(),
            &u2.retrieve().to_le_bytes(),
        )
        .normalize();
        let xq = &BoxedMontyForm::new(p.x.retrieve(), &self.scalar_params);

        xq == r
    }
}

impl Add for &GenericPoint {
    type Output = GenericPoint;

    fn add(self, other: Self) -> GenericPoint {
        let curve = self.curve.clone();
        let (x1, y1, z1) = (&self.x, &self.y, &self.z);
        let (x2, y2, z2) = (&other.x, &other.y, &other.z);
        let a = &curve.a_monty;
        let b3 = &curve.b3_monty;

        let t0 = x1 * x2;
        let t1 = y1 * y2;
        let t2 = z1 * z2;
        let t3 = x1 + y1;
        let t4 = x2 + y2;
        let t3 = &t3 * &t4;
        let t4 = &t0 + &t1;
        let t3 = &t3 - &t4;
        let t4 = x1 + z1;
        let t5 = x2 + z2;
        let t4 = &t4 * &t5;
        let t5 = &t0 + &t2;
        let t4 = &t4 - &t5;
        let t5 = y1 + z1;
        let x3 = y2 + z2;
        let t5 = &t5 * &x3;
        let x3 = &t1 + &t2;
        let t5 = &t5 - &x3;
        let z3 = a * &t4;
        let x3 = b3 * &t2;
        let z3 = &x3 + &z3;
        let x3 = &t1 - &z3;
        let z3 = &t1 + &z3;
        let y3 = &x3 * &z3;
        let t1 = &t0 + &t0;
        let t1 = &t1 + &t0;
        let t2 = a * &t2;
        let t4 = b3 * &t4;
        let t1 = &t1 + &t2;
        let t2 = &t0 - &t2;
        let t2 = a * &t2;
        let t4 = &t4 + &t2;
        let t0 = &t1 * &t4;
        let y3 = &y3 + &t0;
        let t0 = &t5 * &t4;
        let x3 = &t3 * &x3;
        let x3 = &x3 - &t0;
        let t0 = &t3 * &t1;
        let z3 = &t5 * &z3;
        let z3 = &z3 + &t0;

        GenericPoint {
            x: x3,
            y: y3,
            z: z3,
            curve,
        }
    }
}

impl Mul<&[u8]> for GenericPoint {
    type Output = Self;

    fn mul(self, other: &[u8]) -> Self {
        let scalar = BigUint::from_bytes_le(other);

        let mut r0 = Self {
            x: self.curve.zero_monty.clone(),
            y: self.curve.one_monty.clone(),
            z: self.curve.zero_monty.clone(),
            curve: self.curve.clone(),
        };
        let mut r1 = self;

        for i in (0..scalar.bits()).rev() {
            if scalar.bit(i) {
                r0 = &r0 + &r1;
                r1 = &r1 + &r1;
            } else {
                r1 = &r1 + &r0;
                r0 = &r0 + &r0;
            }
        }

        r0
    }
}

impl GenericPoint {
    pub fn normalize(&self) -> Self {
        if self.z == self.curve.zero_monty {
            return Self {
                x: self.curve.zero_monty.clone(),
                y: self.curve.one_monty.clone(),
                z: self.curve.zero_monty.clone(),
                curve: self.curve.clone(),
            };
        }

        let zinv = &self.z.invert().unwrap();

        Self {
            x: &self.x * zinv,
            y: &self.y * zinv,
            z: self.curve.one_monty.clone(),
            curve: self.curve.clone(),
        }
    }

    pub fn shamir(p: &Self, q: &Self, n_bytes: &[u8], m_bytes: &[u8]) -> Self {
        let n = BigUint::from_bytes_le(n_bytes);
        let m = BigUint::from_bytes_le(m_bytes);
        let j = max(n.bits(), m.bits());

        let pq = p + q;
        let mut r = Self {
            x: p.curve.zero_monty.clone(),
            y: p.curve.one_monty.clone(),
            z: p.curve.zero_monty.clone(),
            curve: p.curve.clone(),
        };

        for i in (0..j).rev() {
            r = &r + &r;

            if n.bit(i) && m.bit(i) {
                r = &r + &pq;
            } else if n.bit(i) {
                r = &r + p;
            } else if m.bit(i) {
                r = &r + q;
            }
        }

        r
    }
}
