use pyo3::prelude::*;

use crate::comb::{p192_comb, p224_comb, p256_comb, p384_comb, p521_comb};
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

#[pyfunction]
pub fn p192_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    sign::<P192>(msg, d_bytes, k_bytes, Some(p192_comb()))
}

#[pyfunction]
pub fn p192_verify(r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
    verify::<P192>(r, s, msg, qx, qy)
}

#[pyfunction]
pub fn p224_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    sign::<P224>(msg, d_bytes, k_bytes, Some(p224_comb()))
}

#[pyfunction]
pub fn p224_verify(r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
    verify::<P224>(r, s, msg, qx, qy)
}

#[pyfunction]
pub fn p256_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    sign::<P256>(msg, d_bytes, k_bytes, Some(p256_comb()))
}

#[pyfunction]
pub fn p256_verify(r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
    verify::<P256>(r, s, msg, qx, qy)
}

#[pyfunction]
pub fn p384_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    sign::<P384>(msg, d_bytes, k_bytes, Some(p384_comb()))
}

#[pyfunction]
pub fn p384_verify(r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
    verify::<P384>(r, s, msg, qx, qy)
}

#[pyfunction]
pub fn p521_sign(msg: &[u8], d_bytes: &[u8], k_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    sign::<P521>(msg, d_bytes, k_bytes, Some(p521_comb()))
}

#[pyfunction]
pub fn p521_verify(r: &[u8], s: &[u8], msg: &[u8], qx: &[u8], qy: &[u8]) -> bool {
    verify::<P521>(r, s, msg, qx, qy)
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(p192_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p192_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p224_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p224_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p256_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p256_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p384_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p384_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p521_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p521_verify, m)?)?;

    return Ok(());
}
