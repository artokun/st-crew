// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    notPerson, err := UnmarshalNotPerson(bytes)
//    bytes, err = notPerson.Marshal()

package st_crew_sdk

import "encoding/json"

func UnmarshalNotPerson(data []byte) (NotPerson, error) {
	var r NotPerson
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *NotPerson) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type NotPerson struct {
	Bar float64 `json:"bar"`
	Foo string  `json:"foo"`
}
