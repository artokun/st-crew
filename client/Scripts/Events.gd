extends Node

signal camera_moved(new_location: Vector3)
signal camera_jump_requested(location: Vector3, duration: float)
signal camera_freeze_requested(is_frozen: bool)
