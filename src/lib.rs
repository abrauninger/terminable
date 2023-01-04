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

// Cached values for KeyModifiers constants (defined in types.py)
struct ModifierConstants {
    shift: u8,
    control: u8,
    alt: u8,
}

impl ModifierConstants {
    fn new(py: Python<'_>) -> PyResult<Self> {
        let module = PyModule::import(py, env!("CARGO_PKG_NAME"))?;

        Ok(ModifierConstants {
            shift: get_keymodifier_constant_value(module, "SHIFT")?,
            control: get_keymodifier_constant_value(module, "CONTROL")?,
            alt: get_keymodifier_constant_value(module, "ALT")?,
        })
    }
}

fn get_keymodifier_constant_value(module: &PyModule, value_name: &str) -> PyResult<u8> {
    let modifiers = module.getattr("KeyModifiers")?;
    modifiers.getattr(value_name)?.getattr("value")?.extract()
}

fn get_modifiers_u8_from_xt(modifiers_xt: KeyModifiersXT, constants: &ModifierConstants) -> u8 {
    let mut modifiers = 0;

    if (modifiers_xt & KeyModifiersXT::SHIFT) != KeyModifiersXT::NONE {
        modifiers |= constants.shift;
    }
    if (modifiers_xt & KeyModifiersXT::CONTROL) != KeyModifiersXT::NONE {
        modifiers |= constants.control;
    }
    if (modifiers_xt & KeyModifiersXT::ALT) != KeyModifiersXT::NONE {
        modifiers |= constants.alt;
    }

    return modifiers;
}

fn get_modifiers_expr(modifiers_xt: KeyModifiersXT, constants: &ModifierConstants) -> String {
    let modifiers = get_modifiers_u8_from_xt(modifiers_xt, constants);
    
    if modifiers == 0 {
        "None".to_string()
    } else {
        format!("{}.KeyModifiers({})", env!("CARGO_PKG_NAME"), modifiers)
    }
}

fn get_modifiers_py<'a>(py: Python<'_>, module: &'a PyModule, modifiers_xt: KeyModifiersXT, constants: &ModifierConstants) -> PyResult<PyObject> {
    let key_modifiers_attr = module.getattr("KeyModifiers")?;

    let modifiers = get_modifiers_u8_from_xt(modifiers_xt, constants);

    if modifiers == 0 {
        return Ok(None::<PyObject>.into_py(py));
    }
    else {
        return Ok(key_modifiers_attr.call1((modifiers,))?.to_object(py));
    }
}

#[pyclass]
struct TerminalInput {
    raw_mode: Option<RawMode>,
    modifier_constants: ModifierConstants,
}

#[pymethods]
impl TerminalInput {
    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        let terminal_input = TerminalInput {
            raw_mode: Some(RawMode::new()),
            modifier_constants: ModifierConstants::new(py)?,
        };

        Ok(terminal_input)
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
        // TODO: Cache the module?  Or receive it as a parameter?
        let module = PyModule::import(py, env!("CARGO_PKG_NAME"))?;

        match crossterm::event::read()? {
            Event::Key(key_event) => {
                let key_event_attr = module.getattr("KeyEvent")?;

                let modifiers_py = get_modifiers_py(py, module, key_event.modifiers, &self.modifier_constants)?;

                if let KeyCode::Char(ch) = key_event.code {
                    if ch == 'c' && key_event.modifiers == KeyModifiersXT::CONTROL {
                        self.raw_mode = None;
                        return Err(PyKeyboardInterrupt::new_err(""));
                    }

                    let char_py = module.getattr("Char")?.call1((ch,))?;
                    return Ok(key_event_attr.call1((char_py, modifiers_py))?.to_object(py));
                }

                let key_attr = module.getattr("Key")?;

                let key_value_name = match key_event.code {
                    KeyCode::F(1) => Ok("F1"),
                    KeyCode::F(2) => Ok("F2"),
                    KeyCode::F(3) => Ok("F3"),
                    KeyCode::F(4) => Ok("F4"),
                    KeyCode::F(5) => Ok("F5"),
                    KeyCode::F(6) => Ok("F6"),
                    KeyCode::F(7) => Ok("F7"),
                    KeyCode::F(8) => Ok("F8"),
                    KeyCode::F(9) => Ok("F9"),
                    KeyCode::F(10) => Ok("F10"),
                    KeyCode::F(11) => Ok("F11"),
                    KeyCode::F(12) => Ok("F12"),
                    KeyCode::F(13) => Ok("F13"),
                    KeyCode::F(14) => Ok("F14"),
                    KeyCode::F(15) => Ok("F15"),
                    KeyCode::F(16) => Ok("F16"),
                    KeyCode::F(17) => Ok("F17"),
                    KeyCode::F(18) => Ok("F18"),
                    KeyCode::F(19) => Ok("F19"),
                    KeyCode::F(20) => Ok("F20"),
                    KeyCode::F(21) => Ok("F21"),
                    KeyCode::F(22) => Ok("F22"),
                    KeyCode::F(23) => Ok("F23"),
                    KeyCode::Backspace => Ok("BACKSPACE"),
                    KeyCode::Enter => Ok("ENTER"),
                    KeyCode::Left => Ok("LEFT"),
                    KeyCode::Right => Ok("RIGHT"),
                    KeyCode::Up => Ok("UP"),
                    KeyCode::Down => Ok("DOWN"),
                    KeyCode::Home => Ok("HOME"),
                    KeyCode::End => Ok("END"),
                    KeyCode::PageUp => Ok("PAGEUP"),
                    KeyCode::PageDown => Ok("PAGEDOWN"),
                    KeyCode::Tab => Ok("TAB"),
                    KeyCode::BackTab => Ok("BACKTAB"),
                    KeyCode::Delete => Ok("DELETE"),
                    KeyCode::Insert => Ok("INSERT"),
                    KeyCode::Esc => Ok("ESC"),
                    _ => Err(PyException::new_err("Unrecognized keyboard event")),
                }?;

                let key_value_py = key_attr.getattr(key_value_name)?;
                return Ok(key_event_attr.call1((key_value_py, modifiers_py))?.to_object(py));
            },
            Event::Mouse(mouse_event) => {
                let mouse_event_kind_attr = module.getattr("MouseEventKind")?;
                let mouse_button_attr = module.getattr("MouseButton")?;
                let mouse_event_attr = module.getattr("MouseEvent")?;
                
                let modifiers_py = get_modifiers_py(py, module, mouse_event.modifiers, &self.modifier_constants)?;

                let (kind_value_name, button_value_name) = match mouse_event.kind {
                    MouseEventKindXT::Down(MouseButtonXT::Left) => ("DOWN", Some("LEFT")),
                    MouseEventKindXT::Down(MouseButtonXT::Right) => ("DOWN", Some("RIGHT")),
                    MouseEventKindXT::Down(MouseButtonXT::Middle) => ("DOWN", Some("MIDDLE")),
                    MouseEventKindXT::Up(MouseButtonXT::Left) => ("UP", Some("LEFT")),
                    MouseEventKindXT::Up(MouseButtonXT::Right) => ("UP", Some("RIGHT")),
                    MouseEventKindXT::Up(MouseButtonXT::Middle) => ("UP", Some("MIDDLE")),
                    MouseEventKindXT::Drag(MouseButtonXT::Left) => ("DRAG", Some("LEFT")),
                    MouseEventKindXT::Drag(MouseButtonXT::Right) => ("DRAG", Some("RIGHT")),
                    MouseEventKindXT::Drag(MouseButtonXT::Middle) => ("DRAG", Some("MIDDLE")),
                    MouseEventKindXT::Moved => ("MOVED", None),
                    MouseEventKindXT::ScrollDown => ("SCROLL_DOWN", None),
                    MouseEventKindXT::ScrollUp => ("SCROLL_UP", None),
                };

                let kind_py = mouse_event_kind_attr.getattr(kind_value_name)?;

                let button_py = match button_value_name {
                    Some(button_value) => mouse_button_attr.getattr(button_value)?.to_object(py),
                    None => None::<PyObject>.into_py(py)
                };

                return Ok(mouse_event_attr.call1((kind_py, button_py, mouse_event.column.into_py(py), mouse_event.row.into_py(py), modifiers_py))?.to_object(py));
            }
            Event::Resize(columns, rows) => {
                // let event_expr = format!("{}.ResizeEvent(columns={}, rows={})", PKG_NAME, columns, rows);
                // return Ok(py.eval(&event_expr, None, None)?.to_object(py));
                panic!("TODO: Resize");
            }
        }
    }
}

#[pyfunction]
fn capture_input(py: Python<'_>) -> PyResult<TerminalInput> {
    return TerminalInput::new(py);
}

#[pymodule]
fn terminable(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TerminalInput>()?;
    m.add_function(wrap_pyfunction!(capture_input, m)?)?;
    Ok(())
}