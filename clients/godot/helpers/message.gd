extends RefCounted
class_name Message

class ServerMessage:
	var event: String
	var payload: Dictionary

var message: ServerMessage
var event: String

func _init(buf: PackedByteArray):
	var json = JSON.new()
	var json_string = buf.get_string_from_utf8()
	var error = json.parse(json_string)
	if error == OK:
		var data_received = json.data
		message = JsonNode.json_to_class(data_received, ServerMessage.new()) as ServerMessage
		event = message.event
	else:
		printerr("JSON Parse Error: ", json.get_error_message(), " in ", json_string, " at line ", json.get_error_line())

func payload(_class: Object) -> Variant:
	return JsonNode.json_to_class(message.payload, _class.new())
