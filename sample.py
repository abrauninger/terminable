import terminal_input

with terminal_input.capture() as input_capture:
	while True:
		input_capture.read()