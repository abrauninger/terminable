import terminal_input

with terminal_input.Listener() as listener:
	while True:
		listener.read()