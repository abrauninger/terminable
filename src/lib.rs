use pyo3::prelude::*;

#[pyclass]
struct Listener {
}

#[pymethods]
impl Listener {
	#[new]
	fn new() -> Self {
		Listener {}
	}
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn terminal_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Listener>()?;
    Ok(())
}