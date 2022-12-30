use crossterm::{
	event::{
		DisableMouseCapture,
		EnableMouseCapture,
		Event,
		KeyCode,
		KeyModifiers as KeyModifiersXT,
		MouseButton as MouseButtonXT,
		MouseEventKind as MouseEventKindXT,
	},
	execute,
	terminal,
};

use pyo3::exceptions::{PyException, PyKeyboardInterrupt};
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
	struct KeyModifiers: u32 {
		const NONE = 0x00;
		const SHIFT = 0x01;
		const CONTROL = 0x02;
		const ALT = 0x04;
	}
}

#[pymethods]
impl KeyModifiers {
	fn __repr__(&self) -> String {
		if *self == KeyModifiers::NONE {
			"KeyModifiers.NONE".to_string()
		}
		else {
			let mut value = String::new();

			if (*self & KeyModifiers::SHIFT) != KeyModifiers::NONE {
				value.push_str("KeyModifiers.SHIFT");
			}
			if (*self & KeyModifiers::CONTROL) != KeyModifiers::NONE {
				if value.len() != 0 {
					value.push_str(" | ");
				}
				value.push_str("KeyModifiers.CONTROL");
			}
			if (*self & KeyModifiers::ALT) != KeyModifiers::NONE {
				if value.len() != 0 {
					value.push_str(" | ");
				}
				value.push_str("KeyModifiers.ALT");
			}

			value
		}
	}
}

impl From<KeyModifiersXT> for KeyModifiers {
	fn from(modifiers_xt: KeyModifiersXT) -> KeyModifiers {
		let mut modifiers = KeyModifiers::NONE;

		if (modifiers_xt & KeyModifiersXT::SHIFT) != KeyModifiersXT::NONE {
			modifiers |= KeyModifiers::SHIFT;
		}
		if (modifiers_xt & KeyModifiersXT::CONTROL) != KeyModifiersXT::NONE {
			modifiers |= KeyModifiers::CONTROL;
		}
		if (modifiers_xt & KeyModifiersXT::ALT) != KeyModifiersXT::NONE {
			modifiers |= KeyModifiers::ALT;
		}

		return modifiers;
	}
}

#[pyclass]
struct KeyEvent {
	#[pyo3(get)]
	code: PyObject,

	#[pyo3(get)]
	modifiers: KeyModifiers,
}

#[pymethods]
impl KeyEvent {
	fn __repr__(&self) -> String {
		format!("KeyEvent({}, {})", self.code, self.modifiers.__repr__())
	}
}

#[pyclass]
#[derive(Clone)]
enum MouseButton {
	LEFT,
	RIGHT,
	MIDDLE,
}

#[pyclass]
#[derive(Clone)]
#[allow(non_camel_case_types)]
enum MouseEventKind {
	DOWN,
	UP,
	DRAG,
	MOVED,
	SCROLL_DOWN,
	SCROLL_UP,
}

#[pyclass]
struct MouseEvent {
	#[pyo3(get)]
	kind: MouseEventKind,

	#[pyo3(get)]
	button: Option<MouseButton>,

	#[pyo3(get)]
	column: u16,

	#[pyo3(get)]
	row: u16,

	#[pyo3(get)]
	modifiers: KeyModifiers
}

#[pymethods]
impl MouseEvent {
	fn __repr__(&self) -> String {
		let button_str = match &self.button {
			Some(b) => b.__pyo3__repr__(),
			None => "None"
		};

		format!("MouseEvent({}, {}, {}, {}, {})", self.kind.__pyo3__repr__(), button_str, self.column, self.row, self.modifiers.__repr__())
	}
}

#[pyclass]
struct ResizeEvent {
	#[pyo3(get)]
	columns: u16,

	#[pyo3(get)]
	rows: u16,
}

#[pymethods]
impl ResizeEvent {
	fn __repr__(&self) -> String {
		format!("ResizeEvent({}, {})", self.columns, self.rows)
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

struct RawMode {
}

impl RawMode {
	fn new() -> Self {
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
    				if key_event.modifiers == KeyModifiersXT::CONTROL {
						self.raw_mode = None;
						return Err(PyKeyboardInterrupt::new_err(""));
	    			}
	    		}

	    		let modifiers = KeyModifiers::from(key_event.modifiers);

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
    				InternalKeyCode::None => return Err(PyException::new_err("Unrecognized keyboard event"))
    			}
    		},
    		Event::Mouse(mouse_event) => {
	    		let modifiers = KeyModifiers::from(mouse_event.modifiers);

    			let (kind, button) = match mouse_event.kind {
    				MouseEventKindXT::Down(MouseButtonXT::Left) => (MouseEventKind::DOWN, Some(MouseButton::LEFT)),
    				MouseEventKindXT::Down(MouseButtonXT::Right) => (MouseEventKind::DOWN, Some(MouseButton::RIGHT)),
    				MouseEventKindXT::Down(MouseButtonXT::Middle) => (MouseEventKind::DOWN, Some(MouseButton::MIDDLE)),
    				MouseEventKindXT::Up(MouseButtonXT::Left) => (MouseEventKind::UP, Some(MouseButton::LEFT)),
    				MouseEventKindXT::Up(MouseButtonXT::Right) => (MouseEventKind::UP, Some(MouseButton::RIGHT)),
    				MouseEventKindXT::Up(MouseButtonXT::Middle) => (MouseEventKind::UP, Some(MouseButton::MIDDLE)),
    				MouseEventKindXT::Drag(MouseButtonXT::Left) => (MouseEventKind::DRAG, Some(MouseButton::LEFT)),
    				MouseEventKindXT::Drag(MouseButtonXT::Right) => (MouseEventKind::DRAG, Some(MouseButton::RIGHT)),
    				MouseEventKindXT::Drag(MouseButtonXT::Middle) => (MouseEventKind::DRAG, Some(MouseButton::MIDDLE)),
    				MouseEventKindXT::Moved => (MouseEventKind::MOVED, None),
    				MouseEventKindXT::ScrollDown => (MouseEventKind::SCROLL_DOWN, None),
    				MouseEventKindXT::ScrollUp => (MouseEventKind::SCROLL_UP, None),
    			};

    			return Ok(MouseEvent { kind: kind, button: button, column: mouse_event.column, row: mouse_event.row, modifiers: modifiers }.into_py(py));
    		}
    		Event::Resize(columns, rows) => return Ok(ResizeEvent { columns: columns, rows: rows }.into_py(py)),
		}
	}
}

#[pyfunction]
fn capture_input() -> InputCapture {
	return InputCapture::new();
}

#[pymodule]
fn terminable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(capture_input, m)?)?;
    Ok(())
}