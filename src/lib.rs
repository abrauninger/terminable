use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, read},
	execute,
	terminal,
};

use pyo3::prelude::*;

#[pyclass]
struct Listener {
}

#[pymethods]
impl Listener {
	#[new]
	fn new() -> Self {
		println!("Creating a Listener object");
		terminal::enable_raw_mode();
		Listener {}
	}

	fn do_something(&self) {
		println!("Something!");
	}
}

impl Drop for Listener {
	fn drop(&mut self) {
		terminal::disable_raw_mode();
		println!("Destroying a Listener object");
	}
}

/// A Python module implemented in Rust.
#[pymodule]
fn terminal_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Listener>()?;
    Ok(())
}