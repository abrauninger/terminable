import terminable

with terminable.capture_input() as terminal_input:
	while True:
		returned_value = terminal_input.read()

		print(f"Input received: {returned_value}\r")
