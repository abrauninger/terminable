import terminal_input

with terminal_input.capture() as input_capture:
	while True:
		returned_value = input_capture.read()

		print(f"Input received: {returned_value}\r")
