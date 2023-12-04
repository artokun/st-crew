// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    person, err := UnmarshalPerson(bytes)
//    bytes, err = person.Marshal()

package st_crew_sdk

import "encoding/json"

func UnmarshalPerson(data []byte) (Person, error) {
	var r Person
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *Person) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type Person struct {
	LuckyNumber float64 `json:"luckyNumber"`
	Name        string  `json:"name"`
	Nickname    *string `json:"nickname,omitempty"`
}
