use std::sync::Arc;

use crypto_bigint::modular::ConstMontyParams;
use fastecdsa_rs_core::brainpool_curve::BrainpoolCurve;
use fastecdsa_rs_core::brainpool_curve::brainpoolp160r1::{
    Brainpool160P, Brainpool160Q, Brainpoolp160r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp192r1::{
    Brainpool192P, Brainpool192Q, Brainpoolp192r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp224r1::{
    Brainpool224P, Brainpool224Q, Brainpoolp224r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp256r1::{
    Brainpool256P, Brainpool256Q, Brainpoolp256r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp320r1::{
    Brainpool320P, Brainpool320Q, Brainpoolp320r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp384r1::{
    Brainpool384P, Brainpool384Q, Brainpoolp384r1,
};
use fastecdsa_rs_core::brainpool_curve::brainpoolp512r1::{
    Brainpool512P, Brainpool512Q, Brainpoolp512r1,
};
use fastecdsa_rs_core::generic_curve::GenericCurve;
use fastecdsa_rs_core::scalar::ScalarField;
use fastecdsa_rs_core::sec2_curve::p192::P192;
use fastecdsa_rs_core::sec2_curve::p224::P224;
use fastecdsa_rs_core::sec2_curve::p256::P256;
use fastecdsa_rs_core::sec2_curve::p384::P384;
use fastecdsa_rs_core::sec2_curve::p521::P521;
use fastecdsa_rs_core::sec2_curve::secp192k1::Secp192k1;
use fastecdsa_rs_core::sec2_curve::secp224k1::Secp224k1;
use fastecdsa_rs_core::sec2_curve::secp256k1::Secp256k1;
use fastecdsa_rs_core::sec2_curve::{Field, Point, Sec2Curve};
use num_bigint::BigUint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::point_kind::PointKind;

#[derive(Clone, PartialEq, Debug)]
pub enum CurveKind {
    Brainpoolp160r1,
    Brainpoolp192r1,
    Brainpoolp224r1,
    Brainpoolp256r1,
    Brainpoolp320r1,
    Brainpoolp384r1,
    Brainpoolp512r1,
    P192,
    P224,
    P256,
    P384,
    P521,
    Secp192k1,
    Secp224k1,
    Secp256k1,
    Generic(Arc<GenericCurve>),
}

impl CurveKind {
    pub fn sign(&self, msg: &[u8], d: &[u8], k: &[u8]) -> (Vec<u8>, Vec<u8>) {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpoolp160r1::sign(msg, d, k),
            CurveKind::Brainpoolp192r1 => Brainpoolp192r1::sign(msg, d, k),
            CurveKind::Brainpoolp224r1 => Brainpoolp224r1::sign(msg, d, k),
            CurveKind::Brainpoolp256r1 => Brainpoolp256r1::sign(msg, d, k),
            CurveKind::Brainpoolp320r1 => Brainpoolp320r1::sign(msg, d, k),
            CurveKind::Brainpoolp384r1 => Brainpoolp384r1::sign(msg, d, k),
            CurveKind::Brainpoolp512r1 => Brainpoolp512r1::sign(msg, d, k),
            CurveKind::P192 => P192::sign(msg, d, k),
            CurveKind::P224 => P224::sign(msg, d, k),
            CurveKind::P256 => P256::sign(msg, d, k),
            CurveKind::P384 => P384::sign(msg, d, k),
            CurveKind::P521 => P521::sign(msg, d, k),
            CurveKind::Secp192k1 => Secp192k1::sign(msg, d, k),
            CurveKind::Secp224k1 => Secp224k1::sign(msg, d, k),
            CurveKind::Secp256k1 => Secp256k1::sign(msg, d, k),
            CurveKind::Generic(c) => c.sign(msg, d, k),
        }
    }

    pub fn verify(&self, r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpoolp160r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp192r1 => Brainpoolp192r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp224r1 => Brainpoolp224r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp256r1 => Brainpoolp256r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp320r1 => Brainpoolp320r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp384r1 => Brainpoolp384r1::verify(r, s, msg, qx, qy),
            CurveKind::Brainpoolp512r1 => Brainpoolp512r1::verify(r, s, msg, qx, qy),
            CurveKind::P192 => P192::verify(r, s, msg, qx, qy),
            CurveKind::P224 => P224::verify(r, s, msg, qx, qy),
            CurveKind::P256 => P256::verify(r, s, msg, qx, qy),
            CurveKind::P384 => P384::verify(r, s, msg, qx, qy),
            CurveKind::P521 => P521::verify(r, s, msg, qx, qy),
            CurveKind::Secp192k1 => Secp192k1::verify(r, s, msg, qx, qy),
            CurveKind::Secp224k1 => Secp224k1::verify(r, s, msg, qx, qy),
            CurveKind::Secp256k1 => Secp256k1::verify(r, s, msg, qx, qy),
            CurveKind::Generic(c) => c.verify(r, s, msg, qx, qy),
        }
    }

    pub fn field_order(&self) -> Vec<u8> {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpool160P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp192r1 => Brainpool192P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp224r1 => Brainpool224P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp256r1 => Brainpool256P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp320r1 => Brainpool320P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp384r1 => Brainpool384P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp512r1 => Brainpool512P::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::P192 => Vec::<u8>::from(P192::P),
            CurveKind::P224 => Vec::<u8>::from(P224::P),
            CurveKind::P256 => Vec::<u8>::from(P256::P),
            CurveKind::P384 => Vec::<u8>::from(P384::P),
            CurveKind::P521 => Vec::<u8>::from(P521::P),
            CurveKind::Secp192k1 => Vec::<u8>::from(Secp192k1::P),
            CurveKind::Secp224k1 => Vec::<u8>::from(Secp224k1::P),
            CurveKind::Secp256k1 => Vec::<u8>::from(Secp256k1::P),
            CurveKind::Generic(c) => c.p.to_le_bytes().to_vec(),
        }
    }

    pub fn a_const(&self) -> Vec<u8> {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpoolp160r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp192r1 => Brainpoolp192r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp224r1 => Brainpoolp224r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp256r1 => Brainpoolp256r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp320r1 => Brainpoolp320r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp384r1 => Brainpoolp384r1::A.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp512r1 => Brainpoolp512r1::A.to_le_bytes().to_vec(),
            CurveKind::P192 => Vec::<u8>::from(P192::A),
            CurveKind::P224 => Vec::<u8>::from(P224::A),
            CurveKind::P256 => Vec::<u8>::from(P256::A),
            CurveKind::P384 => Vec::<u8>::from(P384::A),
            CurveKind::P521 => Vec::<u8>::from(P521::A),
            CurveKind::Secp192k1 => Vec::<u8>::from(Secp192k1::A),
            CurveKind::Secp224k1 => Vec::<u8>::from(Secp224k1::A),
            CurveKind::Secp256k1 => Vec::<u8>::from(Secp256k1::A),
            CurveKind::Generic(c) => c.a.to_le_bytes().to_vec(),
        }
    }

    pub fn b_const(&self) -> Vec<u8> {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpoolp160r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp192r1 => Brainpoolp192r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp224r1 => Brainpoolp224r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp256r1 => Brainpoolp256r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp320r1 => Brainpoolp320r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp384r1 => Brainpoolp384r1::B.to_le_bytes().to_vec(),
            CurveKind::Brainpoolp512r1 => Brainpoolp512r1::B.to_le_bytes().to_vec(),
            CurveKind::P192 => Vec::<u8>::from(P192::B),
            CurveKind::P224 => Vec::<u8>::from(P224::B),
            CurveKind::P256 => Vec::<u8>::from(P256::B),
            CurveKind::P384 => Vec::<u8>::from(P384::B),
            CurveKind::P521 => Vec::<u8>::from(P521::B),
            CurveKind::Secp192k1 => Vec::<u8>::from(Secp192k1::B),
            CurveKind::Secp224k1 => Vec::<u8>::from(Secp224k1::B),
            CurveKind::Secp256k1 => Vec::<u8>::from(Secp256k1::B),
            CurveKind::Generic(c) => c.b.to_le_bytes().to_vec(),
        }
    }

    pub fn point_order(&self) -> Vec<u8> {
        match self {
            CurveKind::Brainpoolp160r1 => Brainpool160Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp192r1 => Brainpool192Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp224r1 => Brainpool224Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp256r1 => Brainpool256Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp320r1 => Brainpool320Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp384r1 => Brainpool384Q::PARAMS.modulus().to_le_bytes().to_vec(),
            CurveKind::Brainpoolp512r1 => Brainpool512Q::PARAMS.modulus().to_le_bytes().to_vec(),
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
            CurveKind::Secp192k1 => [
                0x8d, 0xfd, 0xde, 0x74, 0x6a, 0x46, 0x69, 0x0f, 0x17, 0xfc, 0xf2, 0x26, 0xfe, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::Secp224k1 => [
                0xf7, 0xb1, 0x9f, 0x76, 0x71, 0xa9, 0xf0, 0xca, 0x84, 0x61, 0xec, 0xd2, 0xe8, 0xdc,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01,
            ]
            .to_vec(),
            CurveKind::Secp256k1 => [
                0x41, 0x41, 0x36, 0xd0, 0x8c, 0x5e, 0xd2, 0xbf, 0x3b, 0xa0, 0x48, 0xaf, 0xe6, 0xdc,
                0xae, 0xba, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff,
            ]
            .to_vec(),
            CurveKind::Generic(c) => c.q.to_le_bytes().to_vec(),
        }
    }

    pub fn generator(&self) -> PointKind {
        match self {
            CurveKind::Brainpoolp160r1 => PointKind::Brainpoolp160r1(Brainpoolp160r1::G),
            CurveKind::Brainpoolp192r1 => PointKind::Brainpoolp192r1(Brainpoolp192r1::G),
            CurveKind::Brainpoolp224r1 => PointKind::Brainpoolp224r1(Brainpoolp224r1::G),
            CurveKind::Brainpoolp256r1 => PointKind::Brainpoolp256r1(Brainpoolp256r1::G),
            CurveKind::Brainpoolp320r1 => PointKind::Brainpoolp320r1(Brainpoolp320r1::G),
            CurveKind::Brainpoolp384r1 => PointKind::Brainpoolp384r1(Brainpoolp384r1::G),
            CurveKind::Brainpoolp512r1 => PointKind::Brainpoolp512r1(Brainpoolp512r1::G),
            CurveKind::P192 => PointKind::P192(P192::G),
            CurveKind::P224 => PointKind::P224(P224::G),
            CurveKind::P256 => PointKind::P256(P256::G),
            CurveKind::P384 => PointKind::P384(P384::G),
            CurveKind::P521 => PointKind::P521(P521::G),
            CurveKind::Secp192k1 => PointKind::Secp192k1(Secp192k1::G),
            CurveKind::Secp224k1 => PointKind::Secp224k1(Secp224k1::G),
            CurveKind::Secp256k1 => PointKind::Secp256k1(Secp256k1::G),
            CurveKind::Generic(c) => PointKind::Generic(c.generator()),
        }
    }

    pub fn oid(&self) -> Option<Vec<u8>> {
        match self {
            CurveKind::Brainpoolp160r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x01].to_vec())
            }
            CurveKind::Brainpoolp192r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x03].to_vec())
            }
            CurveKind::Brainpoolp224r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x05].to_vec())
            }
            CurveKind::Brainpoolp256r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x07].to_vec())
            }
            CurveKind::Brainpoolp320r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x09].to_vec())
            }
            CurveKind::Brainpoolp384r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x0b].to_vec())
            }
            CurveKind::Brainpoolp512r1 => {
                Some([0x2b, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x0d].to_vec())
            }
            CurveKind::P192 => Some([0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x01].to_vec()),
            CurveKind::P224 => Some([0x2b, 0x81, 0x04, 0x00, 0x21].to_vec()),
            CurveKind::P256 => Some([0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07].to_vec()),
            CurveKind::P384 => Some([0x2b, 0x81, 0x04, 0x00, 0x22].to_vec()),
            CurveKind::P521 => Some([0x2b, 0x81, 0x04, 0x00, 0x23].to_vec()),
            CurveKind::Secp192k1 => Some([0x2b, 0x81, 0x04, 0x00, 0x1f].to_vec()),
            CurveKind::Secp224k1 => Some([0x2b, 0x81, 0x04, 0x00, 0x20].to_vec()),
            CurveKind::Secp256k1 => Some([0x2b, 0x81, 0x04, 0x00, 0x0a].to_vec()),
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
            (CurveKind::Brainpoolp160r1, PointKind::Brainpoolp160r1(p)) => {
                check!(Brainpoolp160r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp192r1, PointKind::Brainpoolp192r1(p)) => {
                check!(Brainpoolp192r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp224r1, PointKind::Brainpoolp224r1(p)) => {
                check!(Brainpoolp224r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp256r1, PointKind::Brainpoolp256r1(p)) => {
                check!(Brainpoolp256r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp320r1, PointKind::Brainpoolp320r1(p)) => {
                check!(Brainpoolp320r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp384r1, PointKind::Brainpoolp384r1(p)) => {
                check!(Brainpoolp384r1, Brainpool256r1, p)
            }
            (CurveKind::Brainpoolp512r1, PointKind::Brainpoolp512r1(p)) => {
                check!(Brainpoolp512r1, Brainpool256r1, p)
            }
            (CurveKind::P192, PointKind::P192(p)) => check!(P192, P192, p),
            (CurveKind::P224, PointKind::P224(p)) => check!(P224, P224, p),
            (CurveKind::P256, PointKind::P256(p)) => check!(P256, P256, p),
            (CurveKind::P384, PointKind::P384(p)) => check!(P384, P384, p),
            (CurveKind::P521, PointKind::P521(p)) => check!(P521, P521, p),
            (CurveKind::Secp192k1, PointKind::Secp192k1(p)) => check!(Secp192k1, Secp192k1, p),
            (CurveKind::Secp224k1, PointKind::Secp224k1(p)) => check!(Secp224k1, Secp224k1, p),
            (CurveKind::Secp256k1, PointKind::Secp256k1(p)) => check!(Secp256k1, Secp256k1, p),
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
            CurveKind::Brainpoolp160r1 => Brainpoolp160r1::evaluate(x_bytes),
            CurveKind::Brainpoolp192r1 => Brainpoolp192r1::evaluate(x_bytes),
            CurveKind::Brainpoolp224r1 => Brainpoolp224r1::evaluate(x_bytes),
            CurveKind::Brainpoolp256r1 => Brainpoolp256r1::evaluate(x_bytes),
            CurveKind::Brainpoolp320r1 => Brainpoolp320r1::evaluate(x_bytes),
            CurveKind::Brainpoolp384r1 => Brainpoolp384r1::evaluate(x_bytes),
            CurveKind::Brainpoolp512r1 => Brainpoolp512r1::evaluate(x_bytes),
            CurveKind::P192 => eval!(P192),
            CurveKind::P224 => eval!(P224),
            CurveKind::P256 => eval!(P256),
            CurveKind::P384 => eval!(P384),
            CurveKind::P521 => eval!(P521),
            CurveKind::Secp192k1 => eval!(Secp192k1),
            CurveKind::Secp224k1 => eval!(Secp224k1),
            CurveKind::Secp256k1 => eval!(Secp256k1),
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
            CurveKind::Brainpoolp160r1 => {
                PointKind::Brainpoolp160r1(Brainpoolp160r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp192r1 => {
                PointKind::Brainpoolp192r1(Brainpoolp192r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp224r1 => {
                PointKind::Brainpoolp224r1(Brainpoolp224r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp256r1 => {
                PointKind::Brainpoolp256r1(Brainpoolp256r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp320r1 => {
                PointKind::Brainpoolp320r1(Brainpoolp320r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp384r1 => {
                PointKind::Brainpoolp384r1(Brainpoolp384r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::Brainpoolp512r1 => {
                PointKind::Brainpoolp512r1(Brainpoolp512r1::point_from_affine(x_bytes, y_bytes))
            }
            CurveKind::P192 => build!(P192, P192),
            CurveKind::P224 => build!(P224, P224),
            CurveKind::P256 => build!(P256, P256),
            CurveKind::P384 => build!(P384, P384),
            CurveKind::P521 => build!(P521, P521),
            CurveKind::Secp192k1 => build!(Secp192k1, Secp192k1),
            CurveKind::Secp224k1 => build!(Secp224k1, Secp224k1),
            CurveKind::Secp256k1 => build!(Secp256k1, Secp256k1),
            CurveKind::Generic(c) => PointKind::Generic(c.point_from_affine(x_bytes, y_bytes)),
        };

        if !self.is_point_on_curve(&point)? {
            return Err(PyValueError::new_err("point is not on the curve"));
        }

        Ok(point)
    }

    pub fn repr_name(&self) -> String {
        match self {
            CurveKind::Brainpoolp160r1 => "Brainpoolp160r1".to_string(),
            CurveKind::Brainpoolp192r1 => "Brainpoolp192r1".to_string(),
            CurveKind::Brainpoolp224r1 => "Brainpoolp224r1".to_string(),
            CurveKind::Brainpoolp256r1 => "Brainpoolp256r1".to_string(),
            CurveKind::Brainpoolp320r1 => "Brainpoolp320r1".to_string(),
            CurveKind::Brainpoolp384r1 => "Brainpoolp384r1".to_string(),
            CurveKind::Brainpoolp512r1 => "Brainpoolp512r1".to_string(),
            CurveKind::P192 => "P192".to_string(),
            CurveKind::P224 => "P224".to_string(),
            CurveKind::P256 => "P256".to_string(),
            CurveKind::P384 => "P384".to_string(),
            CurveKind::P521 => "P521".to_string(),
            CurveKind::Secp192k1 => "Secp192k1".to_string(),
            CurveKind::Secp224k1 => "Secp224k1".to_string(),
            CurveKind::Secp256k1 => "Secp256k1".to_string(),
            CurveKind::Generic(c) => c.name.clone(),
        }
    }
}
