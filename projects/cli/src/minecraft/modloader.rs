use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ModLoaderType {
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "forge")]
    Forge,
}

impl ModLoaderType {
    pub fn get_id(&self) -> u32 {
        match self {
            ModLoaderType::Fabric => 4,
            ModLoaderType::Forge => 1,
        }
    }
}
