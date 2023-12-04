#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    MsgPack,
    Form,
}

impl DataFormat {
    pub fn deserialize<T>(&self, data: &[u8]) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(match self {
            DataFormat::Json => serde_json::from_slice(data)?,
            DataFormat::MsgPack => rmp_serde::from_slice(data)?,
            DataFormat::Form => serde_urlencoded::from_bytes(data)?,
        })
    }

    pub fn serialize<T>(
        &self,
        data: &T,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
    where
        T: serde::Serialize,
    {
        Ok(match self {
            DataFormat::Json => serde_json::to_vec(data)?,
            DataFormat::MsgPack => rmp_serde::to_vec(data)?,
            DataFormat::Form => serde_urlencoded::to_string(data)?.into_bytes(),
        })
    }
}
