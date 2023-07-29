use serde::{Deserialize, Serialize};

/// Types of Minecraft modloaders.
/// 
/// This enum is used to make requests to the CurseForge API.
/// 
/// # Variants
/// 
/// * `Fabric` - The Fabric modloader.
/// * `Forge` - The Forge modloader.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
}
