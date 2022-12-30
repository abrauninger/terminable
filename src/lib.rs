use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::{PyException, PyKeyboardInterrupt};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyType};

#[pyclass]
struct Char {
	#[pyo3(get)]
	code: char
}

// A flattened version of crossterm::event::KeyCode
// Key codes match https://blessed.readthedocs.io/en/latest/keyboard.html
#[pyclass]
enum Key {
	Backspace = 263,
	Enter = 343,
	Left = 260,
	Right = 261,
	Up = 259,
	Down = 258,
	Home = 262,
	End = 360,
	PageUp = 339,
	PageDown = 338,
	Tab = 512,
	BackTab = 353,
	Delete = 330,
	Insert = 331,
	F0 = 264,
	F1 = 265,
	F2 = 266,
	F3 = 267,
	F4 = 268,
	F5 = 269,
	F6 = 270,
	F7 = 271,
	F8 = 272,
	F9 = 273,
	F10 = 274,
	F11 = 275,
	F12 = 276,
	F13 = 277,
	F14 = 278,
	F15 = 279,
	F16 = 280,
	F17 = 281,
	F18 = 282,
	F19 = 283,
	F20 = 284,
	F21 = 285,
	F22 = 286,
	F23 = 287,
	Esc = 361,
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
	raw_mode: Option<RawMode>,
}

#[pymethods]
impl InputCapture {
	#[new]
	fn new() -> Self {
		println!("Creating a InputCapture object");
		InputCapture { raw_mode: Some(RawMode::new()) }
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
    		Event::Key(key_event) => {
    			if let KeyCode::Char('c') = key_event.code {
    				if key_event.modifiers == KeyModifiers::CONTROL {
						self.raw_mode = None;
						return Err(PyKeyboardInterrupt::new_err(""));
	    			}
	    		}

    			match key_event.code {
    				KeyCode::Char(ch) => return Ok(Char { code: ch }.into_py(py)),
    				KeyCode::F(n) => {
   						let key = match n {
							0 => Key::F0,
							1 => Key::F1,
							2 => Key::F2,
							3 => Key::F3,
							4 => Key::F4,
							5 => Key::F5,
							6 => Key::F6,
							7 => Key::F7,
							8 => Key::F8,
							9 => Key::F9,
							10 => Key::F10,
							11 => Key::F11,
							12 => Key::F12,
							13 => Key::F13,
							14 => Key::F14,
							15 => Key::F15,
							16 => Key::F16,
							17 => Key::F17,
							18 => Key::F18,
							19 => Key::F19,
							20 => Key::F20,
							21 => Key::F21,
							22 => Key::F22,
							23 => Key::F23,
							_ => {
								return Err(PyException::new_err("Unrecognized function key"));
							}
						};
						return Ok(key.into_py(py))
    				},
					KeyCode::Backspace => return Ok(Key::Backspace.into_py(py)),
					KeyCode::Enter => return Ok(Key::Enter.into_py(py)),
					KeyCode::Left => return Ok(Key::Left.into_py(py)),
					KeyCode::Right => return Ok(Key::Right.into_py(py)),
					KeyCode::Up => return Ok(Key::Up.into_py(py)),
					KeyCode::Down => return Ok(Key::Down.into_py(py)),
					KeyCode::Home => return Ok(Key::Home.into_py(py)),
					KeyCode::End => return Ok(Key::End.into_py(py)),
					KeyCode::PageUp => return Ok(Key::PageUp.into_py(py)),
					KeyCode::PageDown => return Ok(Key::PageDown.into_py(py)),
					KeyCode::Tab => return Ok(Key::Tab.into_py(py)),
					KeyCode::BackTab => return Ok(Key::BackTab.into_py(py)),
					KeyCode::Delete => return Ok(Key::Delete.into_py(py)),
					KeyCode::Insert => return Ok(Key::Insert.into_py(py)),
					KeyCode::Esc => return Ok(Key::Esc.into_py(py)),
    				_ => {
    					println!("Unhandled event: {:?}\r", key_event);
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