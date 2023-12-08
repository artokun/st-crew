extends Button

var _is_frozen = false

func _on_pressed():
	if _is_frozen:
		Events.emit_signal("camera_freeze_requested", false)
		self.text = "Freeze Camera"
	else: 
		Events.emit_signal("camera_freeze_requested", true)
		self.text = "Unfreeze Camera"
	_is_frozen = !_is_frozen
