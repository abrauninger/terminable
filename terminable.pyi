
class Char:
	__match_args__ = ('code')

	code: str

class Key:
	BACKSPACE: int
	ENTER: int
	LEFT: int
	RIGHT: int
	UP: int
	DOWN: int
	HOME: int
	END: int
	PAGEUP: int
	PAGEDOWN: int
	TAB: int
	BACKTAB: int
	DELETE: int
	INSERT: int
	F0: int
	F1: int
	F2: int
	F3: int
	F4: int
	F5: int
	F6: int
	F7: int
	F8: int
	F9: int
	F10: int
	F11: int
	F12: int
	F13: int
	F14: int
	F15: int
	F16: int
	F17: int
	F18: int
	F19: int
	F20: int
	F21: int
	F22: int
	F23: int
	ESC: int
	

class TerminalInput:
	def __enter__(self) -> TerminalInput: ...

	def __exit__(self, exc_type, exc_value, traceback): ...
	
	def read(self) -> KeyEvent | MouseEvent | ResizeEvent: ...

def capture_input() -> TerminalInput: ...
