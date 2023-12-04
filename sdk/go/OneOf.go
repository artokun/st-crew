package st_crew_sdk

type OneOf struct {
	Type       string     `json:"type"`
	Required   []string   `json:"required"`
	Properties Properties `json:"properties"`
}
