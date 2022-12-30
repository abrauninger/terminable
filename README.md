# terminable
Python library for cross-platform terminal input.

## Install
```
python3 -m pip install terminable
```

## Usage
```python
import terminable

with terminable.capture_input() as terminal_input:
    while True:
        returned_value = terminal_input.read()
        print(f"Input received: {returned_value}\r")
```

Sample output:
```
Input received: KeyEvent(Char(a), KeyModifiers.NONE)
Input received: KeyEvent(Char(s), KeyModifiers.NONE)
Input received: KeyEvent(Char(A), KeyModifiers.SHIFT)
Input received: KeyEvent(Char(S), KeyModifiers.SHIFT)
Input received: KeyEvent(Char(F), KeyModifiers.SHIFT)
Input received: KeyEvent(Key.F1, KeyModifiers.NONE)
Input received: KeyEvent(Key.F3, KeyModifiers.NONE)
Input received: KeyEvent(Key.F2, KeyModifiers.NONE)
Input received: KeyEvent(Char(l), KeyModifiers.CONTROL)
Input received: KeyEvent(Char(k), KeyModifiers.CONTROL)
Input received: KeyEvent(Char(p), KeyModifiers.CONTROL)
Input received: MouseEvent(MouseEventKind.MOVED, None, 54, 20, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.MOVED, None, 53, 20, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.MOVED, None, 52, 20, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.MOVED, None, 51, 20, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.MOVED, None, 54, 19, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.SCROLL_DOWN, None, 54, 19, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.SCROLL_DOWN, None, 54, 19, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.SCROLL_DOWN, None, 54, 19, KeyModifiers.NONE)
Input received: MouseEvent(MouseEventKind.SCROLL_DOWN, None, 54, 19, KeyModifiers.NONE)
Input received: ResizeEvent(104, 31)
Input received: ResizeEvent(100, 31)
Input received: ResizeEvent(98, 31)
Input received: ResizeEvent(95, 31)
```

## Types of input
- Keyboard
  - Characters
  - Arrow keys
  - Function keys
  - Enter, Esc, Backspace, etc.
  - Modifiers: `CONTROL`, `SHIFT`, `ALT`, etc.
- Mouse
  - Move
  - Down
  - Up
  - Drag
- Terminal resize


## Implementation
`terminable` is a thin Python wrapper around the excellent [crossterm](https://github.com/crossterm-rs/crossterm) Rust library.

## API

### `capture_input`
`terminable` has a single function:
```python
def capture_input() -> InputCapture:
    ...
```

### `InputCapture`
The `InputCapture` object is a context manager object (intended to be used with Python's `with` statement).

When the `InputCapture` object is created, the terminal is placed into [raw mode](https://en.wikipedia.org/wiki/Terminal_mode), such that:

- Input is not automatically echoed
- `print` calls do not automatically add a carriage return (`\r`)
- Control sequences are not automatically processed

When the `InputCapture` object is destroyed, the terminal exits raw mode.

### `read`
`InputCapture` has a single function:
```
def read(self) -> KeyEvent | MouseEvent | ResizeEvent :
    ...
```

`read` blocks until input is received.

### `Ctrl+C`
`terminable` raises a `KeyboardInterrupt` exception on `Ctrl+C`.
