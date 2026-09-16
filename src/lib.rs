use std::sync::Arc;

use crypto_bigint::modular::ConstMontyParams;
use crypto_bigint::{BoxedUint, Encoding, Resize};
use num_bigint::BigUint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::brainpoolp160r1::{Brainpool160P, Brainpool160Q, Brainpoolp160r1};
use crate::brainpoolp192r1::{Brainpool192P, Brainpool192Q, Brainpoolp192r1};
use crate::brainpoolp224r1::{Brainpool224P, Brainpool224Q, Brainpoolp224r1};
use crate::brainpoolp256r1::{Brainpool256P, Brainpool256Q, Brainpoolp256r1};
use crate::brainpoolp320r1::{Brainpool320P, Brainpool320Q, Brainpoolp320r1};
use crate::brainpoolp384r1::{Brainpool384P, Brainpool384Q, Brainpoolp384r1};
use crate::brainpoolp512r1::{Brainpool512P, Brainpool512Q, Brainpoolp512r1};
use crate::curve::{BrainpoolCurve, BrainpoolPoint, Curve, Field, Point};
use crate::ecdsa::{sign, verify};
use crate::edwards25519::G;
use crate::generic::{CurveError, GenericCurve, GenericPoint};
use crate::p192::P192;
use crate::p224::P224;
use crate::p256::P256;
use crate::p384::P384;
use crate::p521::P521;
use crate::scalar::ScalarField;
use crate::secp192k1::Secp192k1;
use crate::secp224k1::Secp224k1;
use crate::secp256k1::Secp256k1;

pub mod brainpoolp160r1;
pub mod brainpoolp192r1;
pub mod brainpoolp224r1;
pub mod brainpoolp256r1;
pub mod brainpoolp320r1;
pub mod brainpoolp384r1;
pub mod brainpoolp512r1;
pub mod comb;
pub mod curve;
pub mod ecdsa;
pub mod edwards25519;
pub mod generic;
pub mod p192;
pub mod p224;
pub mod p256;
pub mod p384;
pub mod p521;
pub mod scalar;
pub mod secp192k1;
pub mod secp224k1;
pub mod secp256k1;

#[derive(Clone, PartialEq, Debug)]
enum CurveKind {
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

#[derive(Clone)]
enum PointKind {
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
            CurveKind::P192 => sign::<P192>(msg, d, k),
            CurveKind::P224 => sign::<P224>(msg, d, k),
            CurveKind::P256 => sign::<P256>(msg, d, k),
            CurveKind::P384 => sign::<P384>(msg, d, k),
            CurveKind::P521 => sign::<P521>(msg, d, k),
            CurveKind::Secp192k1 => sign::<Secp192k1>(msg, d, k),
            CurveKind::Secp224k1 => sign::<Secp224k1>(msg, d, k),
            CurveKind::Secp256k1 => sign::<Secp256k1>(msg, d, k),
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
            CurveKind::P192 => verify::<P192>(r, s, msg, qx, qy),
            CurveKind::P224 => verify::<P224>(r, s, msg, qx, qy),
            CurveKind::P256 => verify::<P256>(r, s, msg, qx, qy),
            CurveKind::P384 => verify::<P384>(r, s, msg, qx, qy),
            CurveKind::P521 => verify::<P521>(r, s, msg, qx, qy),
            CurveKind::Secp192k1 => verify::<Secp192k1>(r, s, msg, qx, qy),
            CurveKind::Secp224k1 => verify::<Secp224k1>(r, s, msg, qx, qy),
            CurveKind::Secp256k1 => verify::<Secp256k1>(r, s, msg, qx, qy),
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

    fn repr_name(&self) -> String {
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

impl PointKind {
    fn curve_kind(&self) -> CurveKind {
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

    fn add(&self, other: &PointKind, normalize: bool) -> PyResult<PointKind> {
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

    fn mul(&self, scalar_bytes: &[u8], normalize: bool) -> PointKind {
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

    fn neg(&self) -> PointKind {
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

    fn sub(&self, other: &PointKind, normalize: bool) -> PyResult<PointKind> {
        self.add(&other.neg(), normalize)
    }

    fn xyz(&self) -> (BigUint, BigUint, BigUint) {
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

    fn normalize(&self) -> PointKind {
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

#[pyfunction]
pub fn edwards25519_mul(x: &[u8]) -> [u8; 32] {
    (G * x).normalize().into()
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCurve>()?;
    m.add_class::<PyPoint>()?;
    m.add_function(wrap_pyfunction!(edwards25519_mul, m)?)?;
    return Ok(());
}
