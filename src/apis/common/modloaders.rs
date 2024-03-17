use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ModLoaderType {
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "forge")]
    Forge,
}

impl ModLoaderType {
    /// Gets the ID of the modloader type.
    ///
    /// # Returns
    ///
    /// The ID of the modloader type.
    pub fn get_id(&self) -> u32 {
        match self {
            ModLoaderType::Fabric => 4,
            ModLoaderType::Forge => 1,
        }
    }

    pub async fn get_modloaders(game_version: String, modloader_type: &ModLoaderType) {
        todo!()
    }
}