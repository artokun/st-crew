extends RefCounted
class_name JsonNode

var result = null

func _init(result) -> void:
	self.result = result

func is_array() -> bool:
	return (typeof(result) == TYPE_ARRAY)

func is_dict() -> bool:
	return (typeof(result) == TYPE_DICTIONARY)

# Untyped return
func get_result():
	return result

func as_array() -> Array:
	if not is_array():
		printerr("Result is not an array!")
		return []
	var _res: Array = result
	return _res

func as_dictionary() -> Dictionary:
	if not is_dict():
		printerr("Result is not a dictionary!")
		return {}
	var _res: Dictionary = result
	return _res

func _to_string() -> String:
	return JSON.stringify(result)
	
static func json_string_to_class(json_string: String, _class: Object) -> Object:
	var json: JSON = JSON.new()
	if json.parse(json_string) == OK:
		return json_to_class(json.data, _class)
	return _class

static func json_to_class(json: Dictionary, _class: Object) -> Object:
	var properties: Array = _class.get_property_list()
	for key in json.keys():
		for property in properties:
			if property.name == key and property.usage >= 4096:
				if String(property["class_name"]).is_empty():
					_class.set(key, json[key])
				elif property["class_name"] in ["RefCounted", "Object"]:
					_class.set(key, json_to_class(json[key], _class.get(key)))
				break
	return _class

static func class_to_json_string(_class: Object) -> String:
	return JSON.stringify(class_to_json(_class))

static func class_to_json(_class: Object) -> Dictionary:
	var dictionary: Dictionary = {}
	var properties: Array = _class.get_property_list()
	for property in properties:
		if not property["name"].empty() and property.usage >= (1 << 13):
			if (property["class_name"] in ["Reference", "Object"] and property["type"] == 17):
				dictionary[property.name] = class_to_json(_class.get(property.name))
			else:
				dictionary[property.name] = _class.get(property.name)
		if not property["hint_string"].empty() and property.usage >= (1 << 13):
			if (property["class_name"] in ["Reference", "Object"] and property["type"] == 17):
				dictionary[property.hint_string] = class_to_json(_class.get(property.name))
			else:
				dictionary[property.hint_string] = _class.get(property.name)
	return dictionary
