use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConfigDisplay {
    pub title: String,
    pub dimensions: Option<(u32, u32)>,
}
