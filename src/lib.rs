use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyType};

#[pyclass]
struct Char {
	#[pyo3(get)]
	code: char
}

#[pyclass]
struct ReadComplete {
}

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
struct InputCapture {
	mode: Option<RawMode>
}

#[pymethods]
impl InputCapture {
	#[new]
	fn new() -> Self {
		println!("Creating a InputCapture object");
		InputCapture { mode: Some(RawMode::new()) }
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

	fn read(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		match crossterm::event::read()? {
    		Event::Key(event) => {
    			match event {
    				KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => {
    					self.mode = None;
    					return Err(PyKeyboardInterrupt::new_err(""));
    				}
    				_ => {
    					println!("{:?}\r", event);

    					match event {
    						KeyEvent { code: KeyCode::Char(ch), .. } => {
    							return Ok(Char { code: ch }.into_py(py))
    						},
    						_ => ()
    					}
    				}
    			}
    		},
    		Event::Mouse(event) => println!("{:?}\r", event),
    		Event::Resize(width, height) => println!("New size {}x{}\r", width, height),
		}

		Ok(ReadComplete {}.into_py(py))
	}
}

impl Drop for InputCapture {
	fn drop(&mut self) {
		println!("Destroying a InputCapture object");
	}
}

#[pyfunction]
fn capture() -> InputCapture {
	return InputCapture::new();
}

/// A Python module implemented in Rust.
#[pymodule]
fn terminal_input(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Char>()?;
    m.add_class::<InputCapture>()?;
    m.add_function(wrap_pyfunction!(capture, m)?)?;
    Ok(())
}