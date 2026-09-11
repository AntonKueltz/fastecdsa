use std::sync::Arc;

use crypto_bigint::{BoxedUint, Encoding, Resize};
use num_bigint::BigUint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::curve::{Curve, Field, Point};
use crate::ecdsa::{sign, verify};
use crate::generic::{CurveError, GenericCurve, GenericPoint};
use crate::p192::P192;
use crate::p224::P224;
use crate::p256::P256;
use crate::p384::P384;
use crate::p521::P521;

pub mod comb;
pub mod curve;
pub mod ecdsa;
pub mod generic;
pub mod p192;
pub mod p224;
pub mod p256;
pub mod p384;
pub mod p521;
pub mod scalar;

#[derive(Clone, PartialEq, Debug)]
enum CurveKind {
    P192,
    P224,
    P256,
    P384,
    P521,
    Generic(Arc<GenericCurve>),
}

enum PointKind {
    P192(Point<P192>),
    P224(Point<P224>),
    P256(Point<P256>),
    P384(Point<P384>),
    P521(Point<P521>),
    Generic(GenericPoint),
}

impl CurveKind {
    pub fn sign(&self, msg: &[u8], d: &[u8], k: &[u8]) -> (Vec<u8>, Vec<u8>) {
        match self {
            CurveKind::P192 => sign::<P192>(msg, d, k),
            CurveKind::P224 => sign::<P224>(msg, d, k),
            CurveKind::P256 => sign::<P256>(msg, d, k),
            CurveKind::P384 => sign::<P384>(msg, d, k),
            CurveKind::P521 => sign::<P521>(msg, d, k),
            CurveKind::Generic(c) => c.sign(msg, d, k),
        }
    }

    pub fn verify(&self, r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
        match self {
            CurveKind::P192 => verify::<P192>(r, s, msg, qx, qy),
            CurveKind::P224 => verify::<P224>(r, s, msg, qx, qy),
            CurveKind::P256 => verify::<P256>(r, s, msg, qx, qy),
            CurveKind::P384 => verify::<P384>(r, s, msg, qx, qy),
            CurveKind::P521 => verify::<P521>(r, s, msg, qx, qy),
            CurveKind::Generic(c) => c.verify(r, s, msg, qx, qy),
        }
    }

    pub fn field_order(&self) -> Vec<u8> {
        match self {
            CurveKind::P192 => Vec::<u8>::from(P192::P),
            CurveKind::P224 => Vec::<u8>::from(P224::P),
            CurveKind::P256 => Vec::<u8>::from(P256::P),
            CurveKind::P384 => Vec::<u8>::from(P384::P),
            CurveKind::P521 => Vec::<u8>::from(P521::P),
            CurveKind::Generic(c) => c.p.to_le_bytes().to_vec(),
        }
    }

    pub fn a_const(&self) -> Vec<u8> {
        match self {
            CurveKind::P192 => Vec::<u8>::from(P192::A),
            CurveKind::P224 => Vec::<u8>::from(P224::A),
            CurveKind::P256 => Vec::<u8>::from(P256::A),
            CurveKind::P384 => Vec::<u8>::from(P384::A),
            CurveKind::P521 => Vec::<u8>::from(P521::A),
            CurveKind::Generic(c) => c.a.to_le_bytes().to_vec(),
        }
    }

    pub fn b_const(&self) -> Vec<u8> {
        match self {
            CurveKind::P192 => Vec::<u8>::from(P192::B),
            CurveKind::P224 => Vec::<u8>::from(P224::B),
            CurveKind::P256 => Vec::<u8>::from(P256::B),
            CurveKind::P384 => Vec::<u8>::from(P384::B),
            CurveKind::P521 => Vec::<u8>::from(P521::B),
            CurveKind::Generic(c) => c.b.to_le_bytes().to_vec(),
        }
    }

    pub fn point_order(&self) -> Vec<u8> {
        match self {
            CurveKind::P192 => [
                0x31, 0x28, 0xd2, 0xb4, 0xb1, 0xc9, 0x6b, 0x14, 0x36, 0xf8, 0xde, 0x99, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::P224 => [
                0x3d, 0x2a, 0x5c, 0x5c, 0x45, 0x29, 0xdd, 0x13, 0x3e, 0xf0, 0xb8, 0xe0, 0xa2, 0x16,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::P256 => [
                0x51, 0x25, 0x63, 0xfc, 0xc2, 0xca, 0xb9, 0xf3, 0x84, 0x9e, 0x17, 0xa7, 0xad, 0xfa,
                0xe6, 0xbc, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::P384 => [
                0x73u8, 0x29, 0xc5, 0xcc, 0x6a, 0x19, 0xec, 0xec, 0x7a, 0xa7, 0xb0, 0x48, 0xb2,
                0x0d, 0x1a, 0x58, 0xdf, 0x2d, 0x37, 0xf4, 0x81, 0x4d, 0x63, 0xc7, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::P521 => [
                0x09, 0x64, 0x38, 0x91, 0x1e, 0xb7, 0x6f, 0xbb, 0xae, 0x47, 0x9c, 0x89, 0xb8, 0xc9,
                0xb5, 0x3b, 0xd0, 0xa5, 0x09, 0xf7, 0x48, 0x01, 0xcc, 0x7f, 0x6b, 0x96, 0x2f, 0xbf,
                0x83, 0x87, 0x86, 0x51, 0xfa, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
            ]
            .to_vec(),
            CurveKind::Generic(c) => c.q.to_le_bytes().to_vec(),
        }
    }

    pub fn generator(&self) -> PointKind {
        match self {
            CurveKind::P192 => PointKind::P192(P192::G),
            CurveKind::P224 => PointKind::P224(P224::G),
            CurveKind::P256 => PointKind::P256(P256::G),
            CurveKind::P384 => PointKind::P384(P384::G),
            CurveKind::P521 => PointKind::P521(P521::G),
            CurveKind::Generic(c) => PointKind::Generic(c.generator()),
        }
    }

    pub fn oid(&self) -> Option<Vec<u8>> {
        match self {
            CurveKind::P192 => Some([0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x01].to_vec()),
            CurveKind::P224 => Some([0x2b, 0x81, 0x04, 0x00, 0x21].to_vec()),
            CurveKind::P256 => Some([0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07].to_vec()),
            CurveKind::P384 => Some([0x2b, 0x81, 0x04, 0x00, 0x22].to_vec()),
            CurveKind::P521 => Some([0x2b, 0x81, 0x04, 0x00, 0x23].to_vec()),
            CurveKind::Generic(_) => None,
        }
    }

    pub fn is_point_on_curve(&self, point: &PointKind) -> PyResult<bool> {
        macro_rules! check {
            ($C:ty, $variant:ident, $p:ident) => {{
                let lhs = $p.y * $p.y;
                let rhs = $p.x * $p.x * $p.x + <$C>::A * $p.x + <$C>::B;
                lhs == rhs
            }};
        }

        Ok(match (self, point) {
            (CurveKind::P192, PointKind::P192(p)) => check!(P192, P192, p),
            (CurveKind::P224, PointKind::P224(p)) => check!(P224, P224, p),
            (CurveKind::P256, PointKind::P256(p)) => check!(P256, P256, p),
            (CurveKind::P384, PointKind::P384(p)) => check!(P384, P384, p),
            (CurveKind::P521, PointKind::P521(p)) => check!(P521, P521, p),
            (CurveKind::Generic(c), PointKind::Generic(p)) => c.is_point_on_curve(p),
            _ => return Err(PyValueError::new_err("point does not belong to this curve")),
        })
    }

    pub fn evaluate(&self, x_bytes: &[u8]) -> BigUint {
        macro_rules! eval {
            ($C:ty) => {{
                let x = Field::<$C>::from(x_bytes);
                let rhs = x * x * x + <$C>::A * x + <$C>::B;
                BigUint::from_bytes_le(&Vec::<u8>::from(rhs))
            }};
        }

        match self {
            CurveKind::P192 => eval!(P192),
            CurveKind::P224 => eval!(P224),
            CurveKind::P256 => eval!(P256),
            CurveKind::P384 => eval!(P384),
            CurveKind::P521 => eval!(P521),
            CurveKind::Generic(c) => c.evaluate(x_bytes),
        }
    }

    pub fn point_from_affine(&self, x_bytes: &[u8], y_bytes: &[u8]) -> PyResult<PointKind> {
        macro_rules! build {
            ($C:ty, $variant:ident) => {{
                let x = Field::<$C>::from(x_bytes);
                let y = Field::<$C>::from(y_bytes);
                PointKind::$variant(Point::<$C> { x, y, z: <$C>::ONE })
            }};
        }

        let point = match self {
            CurveKind::P192 => build!(P192, P192),
            CurveKind::P224 => build!(P224, P224),
            CurveKind::P256 => build!(P256, P256),
            CurveKind::P384 => build!(P384, P384),
            CurveKind::P521 => build!(P521, P521),
            CurveKind::Generic(c) => PointKind::Generic(c.point_from_affine(x_bytes, y_bytes)),
        };

        if !self.is_point_on_curve(&point)? {
            return Err(PyValueError::new_err("point is not on the curve"));
        }

        Ok(point)
    }

    fn repr_name(&self) -> String {
        match self {
            CurveKind::P192 => "P192".to_string(),
            CurveKind::P224 => "P224".to_string(),
            CurveKind::P256 => "P256".to_string(),
            CurveKind::P384 => "P384".to_string(),
            CurveKind::P521 => "P521".to_string(),
            CurveKind::Generic(c) => c.name.clone(),
        }
    }
}

#[pyclass(name = "Curve")]
struct PyCurve(Arc<CurveKind>);

#[pymethods]
impl PyCurve {
    #[new]
    fn new(
        name: String,
        p: BigUint,
        a: BigUint,
        b: BigUint,
        q: BigUint,
        gx: BigUint,
        gy: BigUint,
    ) -> PyResult<Self> {
        let bits = p.bits();

        let widen = |n: BigUint| -> BoxedUint {
            BoxedUint::from_le_bytes(n.to_bytes_le().into()).resize(bits as u32)
        };

        let result = GenericCurve::new(
            name,
            BoxedUint::from_le_bytes(p.to_bytes_le().into()),
            widen(a),
            widen(b),
            BoxedUint::from_le_bytes(q.to_bytes_le().into()),
            widen(gx),
            widen(gy),
        );

        match result {
            Ok(curve) => Ok(PyCurve(Arc::new(CurveKind::Generic(Arc::new(curve))))),
            Err(error) => match error {
                CurveError::EvenModulus => Err(PyValueError::new_err("modulus is even")),
                CurveError::PNotPrime => Err(PyValueError::new_err("field order is not prime")),
                CurveError::SingularCurve => Err(PyValueError::new_err("curve discriminant is 0")),
                CurveError::PointNotOnCurve => Err(PyValueError::new_err(
                    "generator is not a point on the curve",
                )),
                CurveError::QNotPrime => Err(PyValueError::new_err("generator order is not prime")),
            },
        }
    }

    #[staticmethod]
    fn p192() -> Self {
        PyCurve(Arc::new(CurveKind::P192))
    }

    #[staticmethod]
    fn p224() -> Self {
        PyCurve(Arc::new(CurveKind::P224))
    }

    #[staticmethod]
    fn p256() -> Self {
        PyCurve(Arc::new(CurveKind::P256))
    }

    #[staticmethod]
    fn p384() -> Self {
        PyCurve(Arc::new(CurveKind::P384))
    }

    #[staticmethod]
    fn p521() -> Self {
        PyCurve(Arc::new(CurveKind::P521))
    }

    #[getter]
    fn p(&self) -> BigUint {
        BigUint::from_bytes_le(&self.0.field_order())
    }

    #[getter]
    fn a(&self) -> BigUint {
        BigUint::from_bytes_le(&self.0.a_const())
    }

    #[getter]
    fn b(&self) -> BigUint {
        BigUint::from_bytes_le(&self.0.b_const())
    }

    #[getter]
    fn q(&self) -> BigUint {
        BigUint::from_bytes_le(&self.0.point_order())
    }

    #[getter(G)]
    fn generator(&self) -> PyPoint {
        PyPoint {
            curve: self.0.clone(),
            point: self.0.generator(),
        }
    }

    #[getter]
    fn oid(&self) -> Option<Vec<u8>> {
        self.0.oid()
    }

    fn sign(&self, msg: &[u8], d: &[u8], k: &[u8]) -> (Vec<u8>, Vec<u8>) {
        self.0.sign(msg, d, k)
    }

    fn verify(&self, r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
        self.0.verify(r, s, msg, qx, qy)
    }

    fn is_point_on_curve(&self, point: &PyPoint) -> PyResult<bool> {
        self.0.is_point_on_curve(&point.point)
    }

    fn evaluate(&self, x: BigUint) -> BigUint {
        self.0.evaluate(&x.to_bytes_le())
    }

    fn __repr__(&self) -> String {
        self.0.repr_name()
    }
}

impl PointKind {
    fn curve_kind(&self) -> CurveKind {
        match self {
            PointKind::P192(_) => CurveKind::P192,
            PointKind::P224(_) => CurveKind::P224,
            PointKind::P256(_) => CurveKind::P256,
            PointKind::P384(_) => CurveKind::P384,
            PointKind::P521(_) => CurveKind::P521,
            PointKind::Generic(point) => CurveKind::Generic(point.curve.clone()),
        }
    }

    fn add(&self, other: &PointKind) -> PyResult<PointKind> {
        match (self, other) {
            (PointKind::P192(a), PointKind::P192(b)) => Ok(PointKind::P192((*a + *b).normalize())),
            (PointKind::P224(a), PointKind::P224(b)) => Ok(PointKind::P224((*a + *b).normalize())),
            (PointKind::P256(a), PointKind::P256(b)) => Ok(PointKind::P256((*a + *b).normalize())),
            (PointKind::P384(a), PointKind::P384(b)) => Ok(PointKind::P384((*a + *b).normalize())),
            (PointKind::P521(a), PointKind::P521(b)) => Ok(PointKind::P521((*a + *b).normalize())),
            (PointKind::Generic(a), PointKind::Generic(b)) => {
                Ok(PointKind::Generic((a + b).normalize()))
            }
            _ => Err(PyValueError::new_err(
                "cannot add points from different curves",
            )),
        }
    }

    fn mul(&self, scalar_bytes: &[u8]) -> PointKind {
        match self {
            PointKind::P192(p) => PointKind::P192((*p * scalar_bytes).normalize()),
            PointKind::P224(p) => PointKind::P224((*p * scalar_bytes).normalize()),
            PointKind::P256(p) => PointKind::P256((*p * scalar_bytes).normalize()),
            PointKind::P384(p) => PointKind::P384((*p * scalar_bytes).normalize()),
            PointKind::P521(p) => PointKind::P521((*p * scalar_bytes).normalize()),
            PointKind::Generic(p) => PointKind::Generic((p.clone() * scalar_bytes).normalize()),
        }
    }

    fn neg(&self) -> PointKind {
        match self {
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
            PointKind::Generic(p) => PointKind::Generic(GenericPoint {
                x: p.x.clone(),
                y: p.y.neg(),
                z: p.z.clone(),
                curve: p.curve.clone(),
            }),
        }
    }

    fn sub(&self, other: &PointKind) -> PyResult<PointKind> {
        self.add(&other.neg())
    }

    fn xy(&self) -> (BigUint, BigUint) {
        match self {
            PointKind::P192(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
            ),
            PointKind::P224(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
            ),
            PointKind::P256(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
            ),
            PointKind::P384(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
            ),
            PointKind::P521(p) => (
                BigUint::from_bytes_le(&Vec::<u8>::from(p.x)),
                BigUint::from_bytes_le(&Vec::<u8>::from(p.y)),
            ),
            PointKind::Generic(p) => (
                BigUint::from_bytes_le(&p.x.retrieve().to_le_bytes()),
                BigUint::from_bytes_le(&p.y.retrieve().to_le_bytes()),
            ),
        }
    }
}

#[pyclass(name = "Point")]
struct PyPoint {
    curve: Arc<CurveKind>,
    point: PointKind,
}

#[pymethods]
impl PyPoint {
    #[new]
    fn new(x: BigUint, y: BigUint, curve: &PyCurve) -> PyResult<Self> {
        let point = curve
            .0
            .point_from_affine(&x.to_bytes_le(), &y.to_bytes_le())?;
        Ok(PyPoint {
            curve: curve.0.clone(),
            point,
        })
    }

    #[getter]
    fn x(&self) -> BigUint {
        self.point.xy().0
    }

    #[getter]
    fn y(&self) -> BigUint {
        self.point.xy().1
    }

    #[getter]
    fn curve(&self) -> PyCurve {
        PyCurve(self.curve.clone())
    }

    fn __add__(&self, other: &PyPoint) -> PyResult<PyPoint> {
        let sum = self.point.add(&other.point)?;
        Ok(PyPoint {
            curve: self.curve.clone(),
            point: sum,
        })
    }

    fn __mul__(&self, scalar: BigUint) -> PyPoint {
        let k_bytes = scalar.to_bytes_le();
        PyPoint {
            curve: self.curve.clone(),
            point: self.point.mul(&k_bytes),
        }
    }

    fn __rmul__(&self, scalar: BigUint) -> PyPoint {
        self.__mul__(scalar)
    }

    fn __neg__(&self) -> PyPoint {
        PyPoint {
            curve: self.curve.clone(),
            point: self.point.neg(),
        }
    }

    fn __sub__(&self, other: &PyPoint) -> PyResult<PyPoint> {
        let diff = self.point.sub(&other.point)?;
        Ok(PyPoint {
            curve: self.curve.clone(),
            point: diff,
        })
    }

    fn __eq__(&self, other: &PyPoint) -> bool {
        let (sx, sy) = self.point.xy();
        let (ox, oy) = other.point.xy();
        self.point.curve_kind() == other.point.curve_kind() && sx == ox && sy == oy
    }

    fn __repr__(&self) -> String {
        let (x, y) = self.point.xy();
        format!(
            "X: 0x{x:x}\nY: 0x{y:x}\n(On curve {:?})",
            self.curve.repr_name()
        )
    }
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCurve>()?;
    m.add_class::<PyPoint>()?;
    return Ok(());
}
