extends Control

@onready var _log_dest = $Panel/Logs


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_router_server_info_payload(payload: Router.ServerInfoPayload):
	Utils.log(_log_dest, "CLIENTS: %d" % payload.connectedClients)



func _on_router_energy_payload(payload: Router.EnergyPayload):
	Utils.log(_log_dest, "ENERGY: %d/%d" % [payload.current, payload.capacity])
