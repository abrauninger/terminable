import terminal_input

with terminal_input.capture() as input_capture:
	while True:
		returned_value = input_capture.read()

		import pdb
		pdb.set_trace()
		
		print(f"Input received {returned_value}\r")
