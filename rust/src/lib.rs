use pyo3::prelude::*;

pub mod p192;
pub mod p224;
pub mod p256;
pub mod p384;

#[pymodule]
fn fastecdsa_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(p192::p192_scale_point, m)?)?;
    m.add_function(wrap_pyfunction!(p192::p192_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p192::p192_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p224::p224_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p224::p224_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p256::p256_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p256::p256_verify, m)?)?;

    m.add_function(wrap_pyfunction!(p384::p384_sign, m)?)?;
    m.add_function(wrap_pyfunction!(p384::p384_verify, m)?)?;

    return Ok(());
}
