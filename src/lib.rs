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

#[pyclass]
struct TerminalInput {
    raw_mode: Option<RawMode>,
    // The following values are here because PyO3 doesn't support exporting bitflag enums to Python
    modifiers_shift: u8,
    modifiers_control: u8,
    modifiers_alt: u8,
}

fn get_keymodifier_constant_value(module: &PyModule, value_name: &str) -> PyResult<u8> {
    let modifiers = module.getattr("KeyModifiers")?;
    modifiers.getattr(value_name)?.getattr("value")?.extract()
}

fn get_modifiers_u8_from_xt(modifiers_xt: KeyModifiersXT, ti: &TerminalInput) -> u8 {
    let mut modifiers = 0;

    if (modifiers_xt & KeyModifiersXT::SHIFT) != KeyModifiersXT::NONE {
        modifiers |= ti.modifiers_shift;
    }
    if (modifiers_xt & KeyModifiersXT::CONTROL) != KeyModifiersXT::NONE {
        modifiers |= ti.modifiers_control;
    }
    if (modifiers_xt & KeyModifiersXT::ALT) != KeyModifiersXT::NONE {
        modifiers |= ti.modifiers_alt;
    }

    return modifiers;
}

fn get_modifiers_expr(modifiers_xt: KeyModifiersXT, ti: &TerminalInput) -> String {
    let modifiers = get_modifiers_u8_from_xt(modifiers_xt, ti);
    
    if modifiers == 0 {
        "None".to_string()
    } else {
        format!("terminable.KeyModifiers({})", modifiers)
    }
}


#[pymethods]
impl TerminalInput {
    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        let module = PyModule::import(py, "terminable")?;
        let terminal_input = TerminalInput {
            raw_mode: Some(RawMode::new()),
            modifiers_shift: get_keymodifier_constant_value(module, "SHIFT")?,
            modifiers_control: get_keymodifier_constant_value(module, "CONTROL")?,
            modifiers_alt: get_keymodifier_constant_value(module, "ALT")?,
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
        // TODO: Cache this
        let module = PyModule::import(py, "terminable")?;

        match crossterm::event::read()? {
            Event::Key(key_event) => {
                if let KeyCode::Char('c') = key_event.code {
                    if key_event.modifiers == KeyModifiersXT::CONTROL {
                        self.raw_mode = None;
                        return Err(PyKeyboardInterrupt::new_err(""));
                    }
                }

                let modifiers_expr = get_modifiers_expr(key_event.modifiers, self);

                let code_expr = match key_event.code {
                    KeyCode::Char(ch) => Ok(format!("Char('{}')", ch)),
                    KeyCode::F(n) => Ok(format!("Key.F{}", n)),
                    KeyCode::Backspace => Ok("Key.BACKSPACE".to_string()),
                    KeyCode::Enter => Ok("Key.ENTER".to_string()),
                    KeyCode::Left => Ok("Key.LEFT".to_string()),
                    KeyCode::Right => Ok("Key.RIGHT".to_string()),
                    KeyCode::Up => Ok("Key.UP".to_string()),
                    KeyCode::Down => Ok("Key.DOWN".to_string()),
                    KeyCode::Home => Ok("Key.HOME".to_string()),
                    KeyCode::End => Ok("Key.END".to_string()),
                    KeyCode::PageUp => Ok("Key.PAGEUP".to_string()),
                    KeyCode::PageDown => Ok("Key.PAGEDOWN".to_string()),
                    KeyCode::Tab => Ok("Key.TAB".to_string()),
                    KeyCode::BackTab => Ok("Key.BACKTAB".to_string()),
                    KeyCode::Delete => Ok("Key.DELETE".to_string()),
                    KeyCode::Insert => Ok("Key.INSERT".to_string()),
                    KeyCode::Esc => Ok("Key.ESC".to_string()),
                    _ => Err(PyException::new_err("Unrecognized keyboard event")),
                }?;

                let event_expr = format!("terminable.KeyEvent(code=terminable.{}, modifiers={})", code_expr, modifiers_expr);
                return Ok(py.eval(&event_expr, None, None)?.to_object(py));
            },
            Event::Mouse(mouse_event) => {
                let modifiers_expr = get_modifiers_expr(mouse_event.modifiers, self);

                let (kind_expr, button_expr) = match mouse_event.kind {
                    MouseEventKindXT::Down(MouseButtonXT::Left) => ("MouseEventKind.DOWN", "terminable.MouseButton.LEFT"),
                    MouseEventKindXT::Down(MouseButtonXT::Right) => ("MouseEventKind.DOWN", "terminable.MouseButton.RIGHT"),
                    MouseEventKindXT::Down(MouseButtonXT::Middle) => ("MouseEventKind.DOWN", "terminable.MouseButton.MIDDLE"),
                    MouseEventKindXT::Up(MouseButtonXT::Left) => ("MouseEventKind.UP", "terminable.MouseButton.LEFT"),
                    MouseEventKindXT::Up(MouseButtonXT::Right) => ("MouseEventKind.UP", "terminable.MouseButton.RIGHT"),
                    MouseEventKindXT::Up(MouseButtonXT::Middle) => ("MouseEventKind.UP", "terminable.MouseButton.MIDDLE"),
                    MouseEventKindXT::Drag(MouseButtonXT::Left) => ("MouseEventKind.DRAG", "terminable.MouseButton.LEFT"),
                    MouseEventKindXT::Drag(MouseButtonXT::Right) => ("MouseEventKind.DRAG", "terminable.MouseButton.RIGHT"),
                    MouseEventKindXT::Drag(MouseButtonXT::Middle) => ("MouseEventKind.DRAG", "terminable.MouseButton.MIDDLE"),
                    MouseEventKindXT::Moved => ("MouseEventKind.MOVED", "None"),
                    MouseEventKindXT::ScrollDown => ("MouseEventKind.SCROLL_DOWN", "None"),
                    MouseEventKindXT::ScrollUp => ("MouseEventKind.SCROLL_UP", "None"),
                };

                let event_expr = format!(
                    "terminable.MouseEvent(kind=terminable.{}, button={}, column={}, row={}, modifiers = {})",
                    kind_expr,
                    button_expr,
                    mouse_event.column,
                    mouse_event.row,
                    modifiers_expr
                );

                return Ok(py.eval(&event_expr, None, None)?.to_object(py));
            }
            Event::Resize(columns, rows) => {
                let event_expr = format!("terminable.ResizeEvent(columns={}, rows={})", columns, rows);
                return Ok(py.eval(&event_expr, None, None)?.to_object(py));
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