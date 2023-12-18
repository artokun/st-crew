extends Node
class_name Router

class ServerInfoPayload:
	var connectedClients: int
signal server_info_payload(payload: ServerInfoPayload)

class EnergyPayload:
	var current: int
	var capacity: int
signal energy_payload(payload: EnergyPayload)

func _on_web_socket_received(buf: PackedByteArray):
	var message = Message.new(buf)
	
	if message == null:
		return

	match message.event:
		"server_info":
			var payload = message.payload(ServerInfoPayload) as ServerInfoPayload
			server_info_payload.emit(payload)
		"energy_changed":
			var payload = message.payload(EnergyPayload) as EnergyPayload
			energy_payload.emit(payload)
		_:
			printerr("Uncaught Event: %s" % message.event)
			printerr(message.event, message.payload)
