// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    userCommand, err := UnmarshalUserCommand(bytes)
//    bytes, err = userCommand.Marshal()

package st_crew_sdk

import "encoding/json"

func UnmarshalUserCommand(data []byte) (UserCommand, error) {
	var r UserCommand
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserCommand) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserCommand struct {
	Bar float64 `json:"bar"`
	Foo string  `json:"foo"`
}
