use fastecdsa_rs_core::brainpool_curve::brainpoolp160r1::Brainpoolp160r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp192r1::Brainpoolp192r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp224r1::Brainpoolp224r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp256r1::Brainpoolp256r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp320r1::Brainpoolp320r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp384r1::Brainpoolp384r1;
use fastecdsa_rs_core::brainpool_curve::brainpoolp512r1::Brainpoolp512r1;
use fastecdsa_rs_core::brainpool_curve::{BrainpoolCurve, BrainpoolPoint};
use fastecdsa_rs_core::generic_curve::GenericPoint;
use fastecdsa_rs_core::scalar::ScalarField;
use fastecdsa_rs_core::sec2_curve::p192::P192;
use fastecdsa_rs_core::sec2_curve::p224::P224;
use fastecdsa_rs_core::sec2_curve::p256::P256;
use fastecdsa_rs_core::sec2_curve::p384::P384;
use fastecdsa_rs_core::sec2_curve::p521::P521;
use fastecdsa_rs_core::sec2_curve::secp192k1::Secp192k1;
use fastecdsa_rs_core::sec2_curve::secp224k1::Secp224k1;
use fastecdsa_rs_core::sec2_curve::secp256k1::Secp256k1;
use fastecdsa_rs_core::sec2_curve::{Point, Sec2Curve};
use num_bigint::BigUint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::curve_kind::CurveKind;

#[derive(Clone)]
pub enum PointKind {
    Brainpoolp160r1(BrainpoolPoint<Brainpoolp160r1>),
    Brainpoolp192r1(BrainpoolPoint<Brainpoolp192r1>),
    Brainpoolp224r1(BrainpoolPoint<Brainpoolp224r1>),
    Brainpoolp256r1(BrainpoolPoint<Brainpoolp256r1>),
    Brainpoolp320r1(BrainpoolPoint<Brainpoolp320r1>),
    Brainpoolp384r1(BrainpoolPoint<Brainpoolp384r1>),
    Brainpoolp512r1(BrainpoolPoint<Brainpoolp512r1>),
    P192(Point<P192>),
    P224(Point<P224>),
    P256(Point<P256>),
    P384(Point<P384>),
    P521(Point<P521>),
    Secp192k1(Point<Secp192k1>),
    Secp224k1(Point<Secp224k1>),
    Secp256k1(Point<Secp256k1>),
    Generic(GenericPoint),
}

impl PointKind {
    pub fn curve_kind(&self) -> CurveKind {
        match self {
            PointKind::Brainpoolp160r1(_) => CurveKind::Brainpoolp160r1,
            PointKind::Brainpoolp192r1(_) => CurveKind::Brainpoolp192r1,
            PointKind::Brainpoolp224r1(_) => CurveKind::Brainpoolp224r1,
            PointKind::Brainpoolp256r1(_) => CurveKind::Brainpoolp256r1,
            PointKind::Brainpoolp320r1(_) => CurveKind::Brainpoolp320r1,
            PointKind::Brainpoolp384r1(_) => CurveKind::Brainpoolp384r1,
            PointKind::Brainpoolp512r1(_) => CurveKind::Brainpoolp512r1,
            PointKind::P192(_) => CurveKind::P192,
            PointKind::P224(_) => CurveKind::P224,
            PointKind::P256(_) => CurveKind::P256,
            PointKind::P384(_) => CurveKind::P384,
            PointKind::P521(_) => CurveKind::P521,
            PointKind::Secp192k1(_) => CurveKind::Secp192k1,
            PointKind::Secp224k1(_) => CurveKind::Secp224k1,
            PointKind::Secp256k1(_) => CurveKind::Secp256k1,
            PointKind::Generic(point) => CurveKind::Generic(point.curve.clone()),
        }
    }

    pub fn add(&self, other: &PointKind, normalize: bool) -> PyResult<PointKind> {
        let c = match (self, other) {
            (PointKind::Brainpoolp160r1(a), PointKind::Brainpoolp160r1(b)) => Ok(
                PointKind::Brainpoolp160r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp192r1(a), PointKind::Brainpoolp192r1(b)) => Ok(
                PointKind::Brainpoolp192r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp224r1(a), PointKind::Brainpoolp224r1(b)) => Ok(
                PointKind::Brainpoolp224r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp256r1(a), PointKind::Brainpoolp256r1(b)) => Ok(
                PointKind::Brainpoolp256r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp320r1(a), PointKind::Brainpoolp320r1(b)) => Ok(
                PointKind::Brainpoolp320r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp384r1(a), PointKind::Brainpoolp384r1(b)) => Ok(
                PointKind::Brainpoolp384r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::Brainpoolp512r1(a), PointKind::Brainpoolp512r1(b)) => Ok(
                PointKind::Brainpoolp512r1((a.to_twist() + b.to_twist()).from_twist()),
            ),
            (PointKind::P192(a), PointKind::P192(b)) => Ok(PointKind::P192(*a + *b)),
            (PointKind::P224(a), PointKind::P224(b)) => Ok(PointKind::P224(*a + *b)),
            (PointKind::P256(a), PointKind::P256(b)) => Ok(PointKind::P256(*a + *b)),
            (PointKind::P384(a), PointKind::P384(b)) => Ok(PointKind::P384(*a + *b)),
            (PointKind::P521(a), PointKind::P521(b)) => Ok(PointKind::P521(*a + *b)),
            (PointKind::Secp192k1(a), PointKind::Secp192k1(b)) => Ok(PointKind::Secp192k1(*a + *b)),
            (PointKind::Secp224k1(a), PointKind::Secp224k1(b)) => Ok(PointKind::Secp224k1(*a + *b)),
            (PointKind::Secp256k1(a), PointKind::Secp256k1(b)) => Ok(PointKind::Secp256k1(*a + *b)),
            (PointKind::Generic(a), PointKind::Generic(b)) => Ok(PointKind::Generic(a + b)),
            _ => Err(PyValueError::new_err(
                "cannot add points from different curves",
            )),
        };

        if c.is_err() || !normalize {
            c
        } else {
            Ok(c.unwrap().normalize())
        }
    }

    pub fn mul(&self, scalar_bytes: &[u8], normalize: bool) -> PointKind {
        let c = match self {
            PointKind::Brainpoolp160r1(p) => PointKind::Brainpoolp160r1(
                (p.to_twist() * Brainpoolp160r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp192r1(p) => PointKind::Brainpoolp192r1(
                (p.to_twist() * Brainpoolp192r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp224r1(p) => PointKind::Brainpoolp224r1(
                (p.to_twist() * Brainpoolp224r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp256r1(p) => PointKind::Brainpoolp256r1(
                (p.to_twist() * Brainpoolp256r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp320r1(p) => PointKind::Brainpoolp320r1(
                (p.to_twist() * Brainpoolp320r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp384r1(p) => PointKind::Brainpoolp384r1(
                (p.to_twist() * Brainpoolp384r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::Brainpoolp512r1(p) => PointKind::Brainpoolp512r1(
                (p.to_twist() * Brainpoolp512r1::scalar_from_le_bytes(scalar_bytes)).from_twist(),
            ),
            PointKind::P192(p) => PointKind::P192(*p * scalar_bytes),
            PointKind::P224(p) => PointKind::P224(*p * scalar_bytes),
            PointKind::P256(p) => PointKind::P256(*p * scalar_bytes),
            PointKind::P384(p) => PointKind::P384(*p * scalar_bytes),
            PointKind::P521(p) => PointKind::P521(*p * scalar_bytes),
            PointKind::Secp192k1(p) => PointKind::Secp192k1(*p * scalar_bytes),
            PointKind::Secp224k1(p) => PointKind::Secp224k1(*p * scalar_bytes),
            PointKind::Secp256k1(p) => PointKind::Secp256k1(*p * scalar_bytes),
            PointKind::Generic(p) => PointKind::Generic(p.clone() * scalar_bytes),
        };

        if normalize { c.normalize() } else { c }
    }

    pub fn neg(&self) -> PointKind {
        match self {
            PointKind::Brainpoolp160r1(p) => {
                PointKind::Brainpoolp160r1(BrainpoolPoint::<Brainpoolp160r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp192r1(p) => {
                PointKind::Brainpoolp192r1(BrainpoolPoint::<Brainpoolp192r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp224r1(p) => {
                PointKind::Brainpoolp224r1(BrainpoolPoint::<Brainpoolp224r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp256r1(p) => {
                PointKind::Brainpoolp256r1(BrainpoolPoint::<Brainpoolp256r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp320r1(p) => {
                PointKind::Brainpoolp320r1(BrainpoolPoint::<Brainpoolp320r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp384r1(p) => {
                PointKind::Brainpoolp384r1(BrainpoolPoint::<Brainpoolp384r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::Brainpoolp512r1(p) => {
                PointKind::Brainpoolp512r1(BrainpoolPoint::<Brainpoolp512r1> {
                    x: p.x,
                    y: p.y.neg(),
                    z: p.z,
                })
            }
            PointKind::P192(p) => PointKind::P192(Point::<P192> {
                x: p.x,
                y: P192::P - p.y,
                z: p.z,
            }),
            PointKind::P224(p) => PointKind::P224(Point::<P224> {
                x: p.x,
                y: P224::P - p.y,
                z: p.z,
            }),
            PointKind::P256(p) => PointKind::P256(Point::<P256> {
                x: p.x,
                y: P256::P - p.y,
                z: p.z,
            }),
            PointKind::P384(p) => PointKind::P384(Point::<P384> {
                x: p.x,
                y: P384::P - p.y,
                z: p.z,
            }),
            PointKind::P521(p) => PointKind::P521(Point::<P521> {
                x: p.x,
                y: P521::P - p.y,
                z: p.z,
            }),
            PointKind::Secp192k1(p) => PointKind::Secp192k1(Point::<Secp192k1> {
                x: p.x,
                y: Secp192k1::P - p.y,
                z: p.z,
            }),
            PointKind::Secp224k1(p) => PointKind::Secp224k1(Point::<Secp224k1> {
                x: p.x,
                y: Secp224k1::P - p.y,
                z: p.z,
            }),
            PointKind::Secp256k1(p) => PointKind::Secp256k1(Point::<Secp256k1> {
                x: p.x,
                y: Secp256k1::P - p.y,
                z: p.z,
            }),
            PointKind::Generic(p) => PointKind::Generic(GenericPoint {
                x: p.x.clone(),
                y: p.y.neg(),
                z: p.z.clone(),
                curve: p.curve.clone(),
            }),
        }
    }

    pub fn sub(&self, other: &PointKind, normalize: bool) -> PyResult<PointKind> {
        self.add(&other.neg(), normalize)
    }

    pub fn xyz(&self) -> (BigUint, BigUint, BigUint) {
        match self {
            PointKind::Brainpoolp160r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp192r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp224r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp256r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp320r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp384r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::Brainpoolp512r1(p) => (
                BigUint::from_bytes_le(&p.x.to_le_bytes()),
                BigUint::from_bytes_le(&p.y.to_le_bytes()),
                BigUint::from_bytes_le(&p.z.to_le_bytes()),
            ),
            PointKind::P192(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::P224(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::P256(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::P384(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::P521(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::Secp192k1(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::Secp224k1(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::Secp256k1(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.z)),
            ),
            PointKind::Generic(p) => (
                BigUint::from_bytes_le(&p.x.retrieve().to_le_bytes()),
                BigUint::from_bytes_le(&p.y.retrieve().to_le_bytes()),
                BigUint::from_bytes_le(&p.z.retrieve().to_le_bytes()),
            ),
        }
    }

    pub fn normalize(&self) -> PointKind {
        match self {
            PointKind::Brainpoolp160r1(p) => PointKind::Brainpoolp160r1(p.normalize()),
            PointKind::Brainpoolp192r1(p) => PointKind::Brainpoolp192r1(p.normalize()),
            PointKind::Brainpoolp224r1(p) => PointKind::Brainpoolp224r1(p.normalize()),
            PointKind::Brainpoolp256r1(p) => PointKind::Brainpoolp256r1(p.normalize()),
            PointKind::Brainpoolp320r1(p) => PointKind::Brainpoolp320r1(p.normalize()),
            PointKind::Brainpoolp384r1(p) => PointKind::Brainpoolp384r1(p.normalize()),
            PointKind::Brainpoolp512r1(p) => PointKind::Brainpoolp512r1(p.normalize()),
            PointKind::P192(p) => PointKind::P192(p.normalize()),
            PointKind::P224(p) => PointKind::P224(p.normalize()),
            PointKind::P256(p) => PointKind::P256(p.normalize()),
            PointKind::P384(p) => PointKind::P384(p.normalize()),
            PointKind::P521(p) => PointKind::P521(p.normalize()),
            PointKind::Secp192k1(p) => PointKind::Secp192k1(p.normalize()),
            PointKind::Secp224k1(p) => PointKind::Secp224k1(p.normalize()),
            PointKind::Secp256k1(p) => PointKind::Secp256k1(p.normalize()),
            PointKind::Generic(p) => PointKind::Generic(p.normalize()),
        }
    }
}
