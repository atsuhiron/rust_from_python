use pyo3::prelude::*;

#[pyfunction]
fn compute_heavy(input: i32) -> i32 {
    input * input
}

#[pymodule]
fn heavy_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_heavy, m)?)?;
    Ok(())
}
