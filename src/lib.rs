use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;

struct RawMode {
}

impl RawMode {
	fn new() -> Self {
		println!("enable_raw_mode");
		terminal::enable_raw_mode();
		RawMode {}
	}
}

impl Drop for RawMode {
	fn drop(&mut self) {
		terminal::disable_raw_mode();
		println!("disable_raw_mode");
	}
}

#[pyclass]
struct Listener {
	mode: RawMode
}

#[pymethods]
impl Listener {
	#[new]
	fn new() -> Self {
		println!("Creating a Listener object");
		terminal::enable_raw_mode();
		Listener { mode: RawMode {} }
	}

	// TODO: Rename?  'listen'?
	fn read(&self, py: Python<'_>) -> PyResult<()> {
		match crossterm::event::read()? {
    		Event::Key(event) => {
    			match event {
    				KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => {
    					return Err(PyKeyboardInterrupt::new_err("Ctrl+C"));
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