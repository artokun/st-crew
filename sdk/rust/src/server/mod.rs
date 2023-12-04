use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ServerInfoResponse {
    pub clients_connected: u16,
}

#[cfg(test)]
mod tests {
    use schemars::gen::SchemaGenerator;

    use crate::server::ServerInfoResponse;

    #[test]
    fn test_server_info_response() {
        let schema = SchemaGenerator::default().into_root_schema_for::<ServerInfoResponse>();

        assert_eq!(
            serde_json::to_string(&schema).unwrap(),
            r#"{"$schema":"http://json-schema.org/draft-07/schema#","title":"ServerInfoResponse","type":"object","required":["clients_connected"],"properties":{"clients_connected":{"type":"integer","format":"uint16","minimum":0.0}}}"#
        );
    }
}
