use std::sync::Arc;

use num_bigint::BigUint;
use pyo3::prelude::*;

use crate::ecdsa::{sign, verify};
use crate::p192::P192;
use crate::p224::P224;
use crate::p256::P256;
use crate::p384::P384;
use crate::p521::P521;

pub mod comb;
pub mod curve;
pub mod ecdsa;
pub mod p192;
pub mod p224;
pub mod p256;
pub mod p384;
pub mod p521;
pub mod scalar;

enum CurveKind {
    P192,
    P224,
    P256,
    P384,
    P521,
}

impl CurveKind {
    pub fn sign(&self, msg: &[u8], d: &[u8], k: &[u8]) -> (Vec<u8>, Vec<u8>) {
        match self {
            CurveKind::P192 => sign::<P192>(msg, d, k),
            CurveKind::P224 => sign::<P224>(msg, d, k),
            CurveKind::P256 => sign::<P256>(msg, d, k),
            CurveKind::P384 => sign::<P384>(msg, d, k),
            CurveKind::P521 => sign::<P521>(msg, d, k),
        }
    }

    pub fn verify(&self, r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
        match self {
            CurveKind::P192 => verify::<P192>(r, s, msg, qx, qy),
            CurveKind::P224 => verify::<P224>(r, s, msg, qx, qy),
            CurveKind::P256 => verify::<P256>(r, s, msg, qx, qy),
            CurveKind::P384 => verify::<P384>(r, s, msg, qx, qy),
            CurveKind::P521 => verify::<P521>(r, s, msg, qx, qy),
        }
    }

    pub fn order(&self) -> &'static [u8] {
        match self {
            CurveKind::P192 => &[
                0x31, 0x28, 0xd2, 0xb4, 0xb1, 0xc9, 0x6b, 0x14, 0x36, 0xf8, 0xde, 0x99, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            CurveKind::P224 => &[
                0x3d, 0x2a, 0x5c, 0x5c, 0x45, 0x29, 0xdd, 0x13, 0x3e, 0xf0, 0xb8, 0xe0, 0xa2, 0x16,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            CurveKind::P256 => &[
                0x51, 0x25, 0x63, 0xfc, 0xc2, 0xca, 0xb9, 0xf3, 0x84, 0x9e, 0x17, 0xa7, 0xad, 0xfa,
                0xe6, 0xbc, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                0xff, 0xff, 0xff, 0xff,
            ],
            CurveKind::P384 => &[
                0x73, 0x29, 0xc5, 0xcc, 0x6a, 0x19, 0xec, 0xec, 0x7a, 0xa7, 0xb0, 0x48, 0xb2, 0x0d,
                0x1a, 0x58, 0xdf, 0x2d, 0x37, 0xf4, 0x81, 0x4d, 0x63, 0xc7, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            CurveKind::P521 => &[
                0x09, 0x64, 0x38, 0x91, 0x1e, 0xb7, 0x6f, 0xbb, 0xae, 0x47, 0x9c, 0x89, 0xb8, 0xc9,
                0xb5, 0x3b, 0xd0, 0xa5, 0x09, 0xf7, 0x48, 0x01, 0xcc, 0x7f, 0x6b, 0x96, 0x2f, 0xbf,
                0x83, 0x87, 0x86, 0x51, 0xfa, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
            ],
        }
    }
}

#[pyclass(name = "Curve")]
struct PyCurve(Arc<CurveKind>);

#[pymethods]
impl PyCurve {
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
    fn q(&self) -> BigUint {
        BigUint::from_bytes_le(self.0.order())
    }

    fn sign(&self, msg: &[u8], d: &[u8], k: &[u8]) -> (Vec<u8>, Vec<u8>) {
        self.0.sign(msg, d, k)
    }

    fn verify(&self, r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
        self.0.verify(r, s, msg, qx, qy)
    }

    fn __repr__(&self) -> String {
        let name = match *self.0 {
            CurveKind::P192 => "P192",
            CurveKind::P224 => "P224",
            CurveKind::P256 => "P256",
            CurveKind::P384 => "P384",
            CurveKind::P521 => "P521",
        };
        format!("Curve.{name}")
    }
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCurve>()?;
    return Ok(());
}
