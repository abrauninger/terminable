use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::PyKeyboardInterrupt;
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

	// TODO: Rename?  'listen'?
	fn read(&self, py: Python<'_>) -> PyResult<()> {
		println!("Reading.");

		match crossterm::event::read()? {
    		Event::Key(event) => {
    			match event {
    				KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => {
    					return Err(PyKeyboardInterrupt::new_err("Ctrl+C"));
    				}
    				_ => println!("{:?}", event)
    			}
    		},
    		Event::Mouse(event) => println!("{:?}", event),
    		Event::Resize(width, height) => println!("New size {}x{}", width, height),
		}

		Ok(())
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