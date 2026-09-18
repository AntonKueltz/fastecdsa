use std::sync::Arc;

use crypto_bigint::{BoxedUint, Encoding, Resize};
use num_bigint::BigUint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::curve_kind::CurveKind;
use crate::edwards25519::{G as Ed25519G, P as Ed25519P, Point25519, edwards25519_comb};
use crate::generic_curve::{CurveError, GenericCurve};
use crate::point_kind::PointKind;

pub mod brainpool_curve;
pub mod comb;
pub mod curve_kind;
pub mod edwards25519;
pub mod generic_curve;
pub mod point_kind;
pub mod scalar;
pub mod sec2_curve;
pub mod util;

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
    fn brainpoolp160r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp160r1))
    }

    #[staticmethod]
    fn brainpoolp192r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp192r1))
    }

    #[staticmethod]
    fn brainpoolp224r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp224r1))
    }

    #[staticmethod]
    fn brainpoolp256r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp256r1))
    }

    #[staticmethod]
    fn brainpoolp320r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp320r1))
    }

    #[staticmethod]
    fn brainpoolp384r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp384r1))
    }

    #[staticmethod]
    fn brainpoolp512r1() -> Self {
        PyCurve(Arc::new(CurveKind::Brainpoolp512r1))
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

    #[staticmethod]
    fn secp192k1() -> Self {
        PyCurve(Arc::new(CurveKind::Secp192k1))
    }

    #[staticmethod]
    fn secp224k1() -> Self {
        PyCurve(Arc::new(CurveKind::Secp224k1))
    }

    #[staticmethod]
    fn secp256k1() -> Self {
        PyCurve(Arc::new(CurveKind::Secp256k1))
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
            projective: false,
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

#[pyclass(name = "Point")]
struct PyPoint {
    curve: Arc<CurveKind>,
    point: PointKind,
    projective: bool,
}

#[pymethods]
impl PyPoint {
    #[new]
    #[pyo3(signature = (x, y, curve, projective = false))]
    fn new(x: BigUint, y: BigUint, curve: &PyCurve, projective: bool) -> PyResult<Self> {
        let point = curve
            .0
            .point_from_affine(&x.to_bytes_le(), &y.to_bytes_le())?;
        Ok(PyPoint {
            curve: curve.0.clone(),
            point,
            projective,
        })
    }

    #[getter]
    fn x(&self) -> BigUint {
        self.point.xyz().0
    }

    #[getter]
    fn y(&self) -> BigUint {
        self.point.xyz().1
    }

    #[getter]
    fn z(&self) -> BigUint {
        self.point.xyz().2
    }

    #[getter]
    fn curve(&self) -> PyCurve {
        PyCurve(self.curve.clone())
    }

    fn __add__(&self, other: &PyPoint) -> PyResult<PyPoint> {
        let sum = self
            .point
            .add(&other.point, !(self.projective || other.projective))?;
        Ok(PyPoint {
            curve: self.curve.clone(),
            point: sum,
            projective: self.projective || other.projective,
        })
    }

    fn __mul__(&self, scalar: BigUint) -> PyPoint {
        let k_bytes = scalar.to_bytes_le();
        PyPoint {
            curve: self.curve.clone(),
            point: self.point.mul(&k_bytes, !self.projective),
            projective: self.projective,
        }
    }

    fn __rmul__(&self, scalar: BigUint) -> PyPoint {
        self.__mul__(scalar)
    }

    fn __neg__(&self) -> PyPoint {
        PyPoint {
            curve: self.curve.clone(),
            point: self.point.neg(),
            projective: self.projective,
        }
    }

    fn __sub__(&self, other: &PyPoint) -> PyResult<PyPoint> {
        let diff = self
            .point
            .sub(&other.point, !(self.projective || other.projective))?;
        Ok(PyPoint {
            curve: self.curve.clone(),
            point: diff,
            projective: self.projective || other.projective,
        })
    }

    fn __eq__(&self, other: &PyPoint) -> bool {
        let (sx, sy, sz) = self.point.xyz();
        let (ox, oy, oz) = other.point.xyz();

        if sz == BigUint::ONE && oz == BigUint::ONE {
            self.point.curve_kind() == other.point.curve_kind() && sx == ox && sy == oy
        } else {
            self.point.curve_kind() == other.point.curve_kind()
                && &sx * &oz == &ox * &sz
                && &sy * &oz == &oy * &sz
        }
    }

    fn __repr__(&self) -> String {
        let (x, y, z) = self.point.xyz();

        if z == BigUint::ZERO {
            String::from("<Point at Infinity>")
        } else if self.projective {
            format!(
                "X: 0x{x:x}\nY: 0x{y:x}\nZ: 0x{z:x}\n(Projective point on curve {:?})",
                self.curve.repr_name()
            )
        } else {
            format!(
                "X: 0x{x:x}\nY: 0x{y:x}\n(Affine point on curve {:?})",
                self.curve.repr_name()
            )
        }
    }

    fn normalize(&self) -> PyResult<PyPoint> {
        if !self.projective {
            Ok(PyPoint {
                curve: self.curve.clone(),
                point: self.point.clone(),
                projective: false,
            })
        } else {
            let p = self.point.normalize();
            Ok(PyPoint {
                curve: self.curve.clone(),
                point: p,
                projective: false,
            })
        }
    }
}

#[pyclass(name = "Ed25519Point")]
struct PyEd25519Point {
    point: Point25519,
    projective: bool,
}

#[pymethods]
impl PyEd25519Point {
    #[new]
    #[pyo3(signature = (x, y, projective = false))]
    fn new(x: BigUint, y: BigUint, projective: bool) -> PyResult<Self> {
        match Point25519::try_from((x, y)) {
            Ok(point) => Ok(Self { point, projective }),
            Err(s) => Err(PyValueError::new_err(s)),
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        if self.projective || other.projective {
            self.point.x * other.point.z == other.point.x * self.point.z
                && self.point.y * other.point.z == other.point.y * self.point.z
        } else {
            self.point.x == other.point.x && self.point.y == other.point.y
        }
    }

    fn __add__(&self, other: &Self) -> Self {
        let projective = self.projective || other.projective;
        let result = self.point + other.point;

        Self {
            point: if projective {
                result
            } else {
                result.normalize()
            },
            projective,
        }
    }

    fn __mul__(&self, scalar: BigUint) -> Self {
        let result = self.point * &scalar.to_bytes_le();

        Self {
            point: if self.projective {
                result
            } else {
                result.normalize()
            },
            projective: self.projective,
        }
    }

    fn __rmul__(&self, scalar: BigUint) -> Self {
        self.__mul__(scalar)
    }

    fn __neg__(&self) -> Self {
        let neg_x = (Ed25519P - self.point.x).reduce();
        let neg_t = (Ed25519P - self.point.t).reduce();

        Self {
            point: Point25519 {
                x: neg_x,
                y: self.point.y,
                z: self.point.z,
                t: neg_t,
            },
            projective: self.projective,
        }
    }

    fn __sub__(&self, other: &Self) -> PyResult<Self> {
        Ok(self.__add__(&other.__neg__()))
    }

    fn __repr__(&self) -> String {
        if self.point.is_infinity() {
            String::from("<Point at Infinity>")
        } else if self.projective {
            let xbytes: [u8; 32] = self.point.x.into();
            let x = BigUint::from_bytes_le(&xbytes);
            let ybytes: [u8; 32] = self.point.y.into();
            let y = BigUint::from_bytes_le(&ybytes);
            let zbytes: [u8; 32] = self.point.z.into();
            let z = BigUint::from_bytes_le(&zbytes);
            let tbytes: [u8; 32] = self.point.t.into();
            let t = BigUint::from_bytes_le(&tbytes);

            format!(
                "X: 0x{x:x}\nY: 0x{y:x}\nZ: 0x{z:x}\nT: 0x{t:x}\n(Extended projective point on curve Edwards25519)",
            )
        } else {
            let xbytes: [u8; 32] = self.point.x.into();
            let x = BigUint::from_bytes_le(&xbytes);
            let ybytes: [u8; 32] = self.point.y.into();
            let y = BigUint::from_bytes_le(&ybytes);

            format!("X: 0x{x:x}\nY: 0x{y:x}\n(Affine point on curve Edwards25519)")
        }
    }

    #[getter]
    fn x(&self) -> BigUint {
        let bytes: [u8; 32] = self.point.x.into();

        BigUint::from_bytes_le(&bytes.to_vec())
    }

    #[getter]
    fn y(&self) -> BigUint {
        let bytes: [u8; 32] = self.point.y.into();

        BigUint::from_bytes_le(&bytes.to_vec())
    }

    #[getter]
    fn z(&self) -> BigUint {
        let bytes: [u8; 32] = self.point.z.into();

        BigUint::from_bytes_le(&bytes.to_vec())
    }

    #[getter]
    fn t(&self) -> BigUint {
        let bytes: [u8; 32] = self.point.t.into();

        BigUint::from_bytes_le(&bytes.to_vec())
    }

    #[staticmethod]
    fn g() -> Self {
        Self {
            point: Ed25519G,
            projective: false,
        }
    }

    #[staticmethod]
    fn decode(x: Vec<u8>) -> PyResult<Self> {
        match Point25519::try_from(x) {
            Ok(point) => Ok(Self {
                point,
                projective: false,
            }),
            Err(s) => Err(PyValueError::new_err(s)),
        }
    }

    fn encode(&self) -> Vec<u8> {
        let bytes: [u8; 32] = self.point.into();

        bytes.to_vec()
    }

    fn normalize(&self) -> Self {
        if self.projective {
            Self {
                point: self.point.normalize(),
                projective: false,
            }
        } else {
            Self {
                point: self.point.clone(),
                projective: false,
            }
        }
    }

    #[staticmethod]
    fn scale_base(x: BigUint) -> Self {
        Self {
            point: edwards25519_comb().mul(&x.to_bytes_le()),
            projective: true,
        }
    }
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCurve>()?;
    m.add_class::<PyPoint>()?;
    m.add_class::<PyEd25519Point>()?;
    return Ok(());
}
