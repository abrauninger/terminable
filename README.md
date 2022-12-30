# terminable
Python library for cross-platform terminal input.

- Tested on **Windows** and **macOS**, probably works on **Linux** (not tested yet)
- **Does not require root access**
- Provides **keyboard**, **mouse**, and **terminal-resize** events
- **Simple API** that you can plug into **your own event loop**
- **Does not require a full-screen terminal app** or separate window

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
- **Keyboard**
  - Characters
  - Arrow keys
  - Function keys
  - Enter, Esc, Backspace, etc.
  - Modifiers: `CONTROL`, `SHIFT`, `ALT`, etc.
- **Mouse**
  - Move
  - Down
  - Up
  - Drag
  - Scroll (scroll wheel, trackpad scroll gestures)
- **Resize** of terminal window


## Implementation
`terminable` is a thin Python wrapper around the excellent [`crossterm`](https://github.com/crossterm-rs/crossterm) Rust library.

`crossterm` does all the heavy lifting.  `terminable` exposes a small subset of `crossterm` functionality to Python using [`PyO3`](https://docs.rs/pyo3/latest/pyo3/).


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
def read(self) -> KeyEvent | MouseEvent | ResizeEvent:
    ...
```

`read` blocks until input is received.

### `Ctrl+C`
`terminable` raises a `KeyboardInterrupt` exception on `Ctrl+C`.

## Comparison with other libraries

There are other existing libraries for getting terminal input in Python, but each of them have their limitations:

- [`curses`](https://docs.python.org/3/howto/curses.html) is not supported on Windows and requires a the terminal app to be full screen
- [`termios`](https://docs.python.org/3/library/termios.html) is not supported on Windows
- [`UniCurses`](https://pypi.org/project/UniCurses/) is a cross-platform implementation of `curses`, but it is no longer maintained and [users have reported issues on Windows](http://sourceforge.net/apps/wordpress/pyunicurses/)
- [`pygame`](https://pypi.org/project/pygame/) provides comprehensive input functionality, but it requires a separate graphical window and is [not well-suited for terminal input](https://stackoverflow.com/a/9816039)
- [`keyboard`](https://pypi.org/project/keyboard/) [requires root access](https://stackoverflow.com/a/54044833) on some platforms
- [`pynput`](https://pypi.org/project/pynput/) [requires root access](https://pynput.readthedocs.io/en/latest/limitations.html) on some platforms
- [`textual`](https://pypi.org/project/textual/) has good cross-platform input handling in a terminal app, but it is tightly coupled with the `textual` application model; you have to let `textual` run your event loop, for example.
- [`blessed`](https://pypi.org/project/blessed/) has excellent cross-platform **keyboard** input functionality that is easy to use in your own event loop, but `blessed` does not support **mouse** input.
