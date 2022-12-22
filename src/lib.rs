use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyType};

struct RawMode {
}

impl RawMode {
	fn new() -> Self {
		println!("enable_raw_mode");
		terminal::enable_raw_mode().unwrap();

		execute!(
	    	std::io::stdout(),
	    	EnableMouseCapture,
	    ).unwrap();

		RawMode {}
	}
}

impl Drop for RawMode {
	fn drop(&mut self) {
	    execute!(
	    	std::io::stdout(),
	    	DisableMouseCapture,
	    ).unwrap();

		terminal::disable_raw_mode().unwrap();

		println!("disable_raw_mode");
	}
}

#[pyclass]
struct Listener {
	mode: Option<RawMode>
}

#[pymethods]
impl Listener {
	#[new]
	fn new() -> Self {
		println!("Creating a Listener object");
		Listener { mode: Some(RawMode::new()) }
	}

	fn __enter__(slf: Py<Self>) -> Py<Self> {
		slf
	}

	fn __exit__(
		&self,
	    _exc_type: Option<&PyType>, 
	    _exc_value: Option<&PyAny>, 
	    _traceback: Option<&PyAny>) -> PyResult<bool> {
		Ok(false)
	}

	// TODO: Rename?  'listen'?
	fn read(&mut self) -> PyResult<()> {
		match crossterm::event::read()? {
    		Event::Key(event) => {
    			match event {
    				KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => {
    					self.mode = None;
    					return Err(PyKeyboardInterrupt::new_err(""));
    				}
    				_ => println!("{:?}\r", event)
    			}
    		},
    		Event::Mouse(event) => println!("{:?}\r", event),
    		Event::Resize(width, height) => println!("New size {}x{}\r", width, height),
		}

		Ok(())
	}
}

impl Drop for Listener {
	fn drop(&mut self) {
		println!("Destroying a Listener object");
	}
}

/// A Python module implemented in Rust.
#[pymodule]
fn terminal_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Listener>()?;
    Ok(())
}