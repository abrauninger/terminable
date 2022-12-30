use crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
	execute,
	terminal,
};

use pyo3::exceptions::{PyKeyboardInterrupt};
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

enum InternalKeyEvent {
	Char(Char),
	Key(Key),
	None,
}

fn key(k: Key) -> InternalKeyEvent {
	return InternalKeyEvent::Key(k)
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

    			let internal_key_event = match key_event.code {
    				KeyCode::Char(ch) => InternalKeyEvent::Char(Char { code: ch }),
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
							_ => InternalKeyEvent::None
						}
    				},
					KeyCode::Backspace => key(Key::Backspace),
					KeyCode::Enter => key(Key::Enter),
					KeyCode::Left => key(Key::Left),
					KeyCode::Right => key(Key::Right),
					KeyCode::Up => key(Key::Up),
					KeyCode::Down => key(Key::Down),
					KeyCode::Home => key(Key::Home),
					KeyCode::End => key(Key::End),
					KeyCode::PageUp => key(Key::PageUp),
					KeyCode::PageDown => key(Key::PageDown),
					KeyCode::Tab => key(Key::Tab),
					KeyCode::BackTab => key(Key::BackTab),
					KeyCode::Delete => key(Key::Delete),
					KeyCode::Insert => key(Key::Insert),
					KeyCode::Esc => key(Key::Esc),
    				_ => InternalKeyEvent::None,
    			};

    			match internal_key_event {
    				InternalKeyEvent::Char(ch) => return Ok(ch.into_py(py)),
    				InternalKeyEvent::Key(k) => return Ok(k.into_py(py)),
    				InternalKeyEvent::None => println!("Unrecognized event: {:?}\r", key_event),
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