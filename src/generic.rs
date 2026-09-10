use std::ops::{Add, Mul};
use std::sync::Arc;

use crypto_bigint::modular::{BoxedMontyForm, BoxedMontyParams};
use crypto_bigint::{BoxedUint, Encoding, Integer, Limb};
use crypto_primes::{is_prime, Flavor};
use num_bigint::BigUint;

pub struct GenericCurve {
    pub p: BoxedUint,
    pub a: BoxedUint,
    pub b: BoxedUint,
    pub q: BoxedUint,

    field_params: BoxedMontyParams,
    scalar_params: BoxedMontyParams,

    gx_monty: BoxedMontyForm,
    gy_monty: BoxedMontyForm,

    a_monty: BoxedMontyForm,
    b3_monty: BoxedMontyForm,
}

#[derive(Clone)]
pub struct GenericPoint {
    x: BoxedMontyForm,
    y: BoxedMontyForm,
    z: BoxedMontyForm,
    curve: Arc<GenericCurve>,
}

pub enum CurveError {
    EvenModulus,
    PNotPrime,
    SingularCurve,
    PointNotOnCurve,
    QNotPrime,
}

impl GenericCurve {
    pub fn new(
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

        let two = BoxedUint::from(Limb::from(2u32));
        let three = BoxedUint::from(Limb::from(3u32));
        let four = BoxedUint::from(Limb::from(4u32));
        let twenty_seven = BoxedUint::from(Limb::from(27u32));

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
            p,
            a,
            b,
            q,
            field_params,
            scalar_params,
            gx_monty,
            gy_monty,
            a_monty: a_m,
            b3_monty: b3_m,
        })
    }

    pub fn to_field(&self, x: u32) -> BoxedMontyForm {
        BoxedMontyForm::new(BoxedUint::from(Limb::from(x)), &self.field_params)
    }

    pub fn generator(self: &Arc<Self>) -> GenericPoint {
        GenericPoint {
            x: self.gx_monty.clone(),
            y: self.gy_monty.clone(),
            z: self.to_field(1),
            curve: self.clone(),
        }
    }

    pub fn sign(
        self: &Arc<Self>,
        msg: &[u8],
        d_bytes: &[u8],
        k_bytes: &[u8],
    ) -> (Vec<u8>, Vec<u8>) {
        let p = self.generator() * k_bytes;

        let k = BoxedMontyForm::new(
            BoxedUint::from_le_bytes(k_bytes.into()),
            &self.scalar_params,
        );
        let z = BoxedMontyForm::new(BoxedUint::from_le_bytes(msg.into()), &self.scalar_params);
        let r = &BoxedMontyForm::new(p.x.retrieve(), &self.scalar_params);
        let d = BoxedMontyForm::new(
            BoxedUint::from_le_bytes(d_bytes.into()),
            &self.scalar_params,
        );

        let kinv = k.invert().unwrap();
        let s = kinv.mul(z + r * d);

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
        let s = BoxedMontyForm::new(
            BoxedUint::from_le_bytes(s_bytes.into()),
            &self.scalar_params,
        );
        let r = &BoxedMontyForm::new(
            BoxedUint::from_le_bytes(r_bytes.into()),
            &self.scalar_params,
        );
        let q = GenericPoint {
            x: BoxedMontyForm::new(
                BoxedUint::from_le_bytes(qx_bytes.into()),
                &self.field_params,
            ),
            y: BoxedMontyForm::new(
                BoxedUint::from_le_bytes(qy_bytes.into()),
                &self.field_params,
            ),
            z: self.to_field(1),
            curve: self.clone(),
        };
        let z: BoxedMontyForm =
            BoxedMontyForm::new(BoxedUint::from_le_bytes(msg.into()), &self.scalar_params);

        let sinv = &s.invert().unwrap();
        let u1 = z * sinv;
        let u2 = r * sinv;

        let p = &(self.generator() * &u1.retrieve().to_le_bytes())
            + &(q * &u2.retrieve().to_be_bytes());
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
            x: self.curve.to_field(0),
            y: self.curve.to_field(1),
            z: self.curve.to_field(0),
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
