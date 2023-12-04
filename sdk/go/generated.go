// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    generated, err := UnmarshalGenerated(bytes)
//    bytes, err = generated.Marshal()

package st_crew_sdk

import "encoding/json"

func UnmarshalGenerated(data []byte) (Generated, error) {
	var r Generated
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *Generated) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type Generated struct {
	Command   Command `json:"command"`
	WithStuff *string `json:"with_stuff,omitempty"`
}
