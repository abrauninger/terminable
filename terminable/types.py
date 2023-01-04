from enum import auto, Enum, IntFlag
from typing import NamedTuple

class Char(NamedTuple):
	__match_args__ = ('code')
	code: str

# A flattened version of crossterm::event::KeyCode
# Key codes match https://blessed.readthedocs.io/en/latest/keyboard.html
class Key(Enum):
	BACKSPACE = 263
	ENTER = 343
	LEFT = 260
	RIGHT = 261
	UP = 259
	DOWN = 258
	HOME = 262
	END = 360
	PAGEUP = 339
	PAGEDOWN = 338
	TAB = 512
	BACKTAB = 353
	DELETE = 330
	INSERT = 331
	F0 = 264
	F1 = 265
	F2 = 266
	F3 = 267
	F4 = 268
	F5 = 269
	F6 = 270
	F7 = 271
	F8 = 272
	F9 = 273
	F10 = 274
	F11 = 275
	F12 = 276
	F13 = 277
	F14 = 278
	F15 = 279
	F16 = 280
	F17 = 281
	F18 = 282
	F19 = 283
	F20 = 284
	F21 = 285
	F22 = 286
	F23 = 287
	ESC = 361

# Once PyO3 can export enums (and in particular, bitflag enums), we can the types in this
# file back into Rust and export them with #[pyobject].
class KeyModifiers(IntFlag):
	SHIFT = auto()
	CONTROL = auto()
	ALT = auto()

class KeyEvent(NamedTuple):
	code: Key | Char
	modifiers: KeyModifiers | None

class MouseButton(Enum):
	LEFT = auto()
	RIGHT = auto()
	MIDDLE = auto()

class MouseEventKind(Enum):
	DOWN = auto()
	UP = auto()
	DRAG = auto()
	MOVED = auto()
	SCROLL_DOWN = auto()
	SCROLL_UP = auto()

class MouseEvent(NamedTuple):
	kind: MouseEventKind
	button: MouseButton | None
	column: int
	row: int
	modifiers: KeyModifiers

class ResizeEvent(NamedTuple):
	columns: int
	rows: int
