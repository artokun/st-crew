extends Node

class ServerInfoPayload:
	var connectedClients: int

class EnergyPayload:
	var current: int
	var capacity: int

class ServerMessage:
	var event: String
	var payload: Dictionary
	

func _on_web_socket_received(buf: PackedByteArray):
	var json_dict: Dictionary = JSON.parse_string(buf.get_string_from_utf8())
	var message: ServerMessage = JsonNode.json_to_class(json_dict, ServerMessage.new())
	
	match message.event:
		"server_info":
			var payload: ServerInfoPayload = JsonNode.json_to_class(message.payload, ServerInfoPayload.new())
			print("CLIENTS: ", payload.connectedClients)
		"energy_changed":
			var payload: EnergyPayload = JsonNode.json_to_class(message.payload, EnergyPayload.new())
			print("ENERGY: %d/%d" % [payload.current, payload.capacity])
		_:
			printerr("Uncaught Event: %s" % message.event)
			printerr(json_dict)
