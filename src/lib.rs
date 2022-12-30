use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::{PyKeyboardInterrupt};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyType};

use bitflags::bitflags;

#[pyclass]
struct Char {
	#[pyo3(get)]
	code: char
}

#[pymethods]
impl Char {
	fn __repr__(&self) -> String {
		format!("Char({})", self.code)
	}
}

// A flattened version of crossterm::event::KeyCode
// Key codes match https://blessed.readthedocs.io/en/latest/keyboard.html
#[pyclass]
enum Key {
	BACKSPACE = 263,
	ENTER = 343,
	LEFT = 260,
	RIGHT = 261,
	UP = 259,
	DOWN = 258,
	HOME = 262,
	END = 360,
	PAGEUP = 339,
	PAGEDOWN = 338,
	TAB = 512,
	BACKTAB = 353,
	DELETE = 330,
	INSERT = 331,
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
	ESC = 361,
}

bitflags! {
	#[pyclass]
	struct Modifiers: u32 {
		const NONE = 0x00;
		const SHIFT = 0x01;
		const CONTROL = 0x02;
		const ALT = 0x04;
	}
}

#[pymethods]
impl Modifiers {
	fn __repr__(&self) -> String {
		if *self == Modifiers::NONE {
			"Modifiers.NONE".to_string()
		}
		else {
			let mut value = String::new();

			if (*self & Modifiers::SHIFT) != Modifiers::NONE {
				value.push_str("Modifiers.SHIFT");
			}
			if (*self & Modifiers::CONTROL) != Modifiers::NONE {
				if value.len() != 0 {
					value.push_str(" | ");
				}
				value.push_str("Modifiers.CONTROL");
			}
			if (*self & Modifiers::ALT) != Modifiers::NONE {
				if value.len() != 0 {
					value.push_str(" | ");
				}
				value.push_str("Modifiers.ALT");
			}

			value
		}
	}
}

#[pyclass]
struct KeyEvent {
	code: PyObject,
	modifiers: Modifiers,
}

#[pymethods]
impl KeyEvent {
	fn __repr__(&self) -> String {
		format!("KeyEvent({}, {})", self.code, self.modifiers.__repr__())
	}
}

enum InternalKeyCode {
	Char(Char),
	Key(Key),
	None,
}

fn key(k: Key) -> InternalKeyCode {
	return InternalKeyCode::Key(k)
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

	    		let mut modifiers = Modifiers::NONE;

	    		if (key_event.modifiers & KeyModifiers::SHIFT) != KeyModifiers::NONE {
	    			modifiers |= Modifiers::SHIFT;
	    		}
	    		if (key_event.modifiers & KeyModifiers::CONTROL) != KeyModifiers::NONE {
	    			modifiers |= Modifiers::CONTROL;
	    		}
	    		if (key_event.modifiers & KeyModifiers::ALT) != KeyModifiers::NONE {
	    			modifiers |= Modifiers::ALT;
	    		}

    			let internal_key_event = match key_event.code {
    				KeyCode::Char(ch) => InternalKeyCode::Char(Char { code: ch }),
    				KeyCode::F(n) => {
   						match n {
							0 => key(Key::F0),
							1 => key(Key::F1),
							2 => key(Key::F2),
							3 => key(Key::F3),
							4 => key(Key::F4),
							5 => key(Key::F5),
							6 => key(Key::F6),
							7 => key(Key::F7),
							8 => key(Key::F8),
							9 => key(Key::F9),
							10 => key(Key::F10),
							11 => key(Key::F11),
							12 => key(Key::F12),
							13 => key(Key::F13),
							14 => key(Key::F14),
							15 => key(Key::F15),
							16 => key(Key::F16),
							17 => key(Key::F17),
							18 => key(Key::F18),
							19 => key(Key::F19),
							20 => key(Key::F20),
							21 => key(Key::F21),
							22 => key(Key::F22),
							23 => key(Key::F23),
							_ => InternalKeyCode::None
						}
    				},
					KeyCode::Backspace => key(Key::BACKSPACE),
					KeyCode::Enter => key(Key::ENTER),
					KeyCode::Left => key(Key::LEFT),
					KeyCode::Right => key(Key::RIGHT),
					KeyCode::Up => key(Key::UP),
					KeyCode::Down => key(Key::DOWN),
					KeyCode::Home => key(Key::HOME),
					KeyCode::End => key(Key::END),
					KeyCode::PageUp => key(Key::PAGEUP),
					KeyCode::PageDown => key(Key::PAGEDOWN),
					KeyCode::Tab => key(Key::TAB),
					KeyCode::BackTab => key(Key::BACKTAB),
					KeyCode::Delete => key(Key::DELETE),
					KeyCode::Insert => key(Key::INSERT),
					KeyCode::Esc => key(Key::ESC),
    				_ => InternalKeyCode::None,
    			};

    			match internal_key_event {
    				InternalKeyCode::Char(ch) => return Ok(KeyEvent { code: ch.into_py(py), modifiers }.into_py(py)),
    				InternalKeyCode::Key(k) => return Ok(KeyEvent { code: k.into_py(py), modifiers }.into_py(py)),
    				InternalKeyCode::None => println!("Unrecognized event: {:?}\r", key_event),
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