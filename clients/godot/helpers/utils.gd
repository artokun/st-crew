extends Node

func log(node, msg):
	print(msg)
	node.add_text(str(msg) + "\n")
