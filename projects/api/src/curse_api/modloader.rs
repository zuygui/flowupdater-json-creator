use serde::Deserialize;
use time::OffsetDateTime;

use crate::{errors::CreatorError, minecraft::modloader::ModLoaderType};

use super::{CurseApi, CURSE_API_URL};

/// A Minecraft modloader version struct.
/// 
/// This struct is used to deserialize the response from the CurseForge API.
/// 
/// # Fields
/// 
/// * `name` - The name of the modloader version.
/// * `game_version` - The game version of the modloader version.
/// * `latest` - Whether or not the modloader version is the latest.
/// * `recommended` - Whether or not the modloader version is the recommended.
/// * `date_modified` - The date and time the modloader version was last modified.
/// * `type_id` - The ID of the modloader version's type.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLoaderVersion {
    /// The name of the modloader version.
    pub name: String,
    /// The game version of the modloader version.
    pub game_version: String,
    /// Whether or not the modloader version is the latest.
    pub latest: bool,
    /// Whether or not the modloader version is the recommended.
    pub recommended: bool,
    /// The date and time the modloader version was last modified.
    #[serde(with = "time::serde::rfc3339")]
    /// The ID of the modloader version's type.
    pub date_modified: OffsetDateTime,
    /// The ID of the modloader version's type.
    #[serde(rename = "type")]
    pub type_id: u32,
}

/// A list of Minecraft modloader versions.
#[derive(Debug, Deserialize)]
pub struct ModLoaderList {
    /// The list of Minecraft modloader versions.
    pub data: Vec<ModLoaderVersion>,
}

impl CurseApi {
    /// Gets a list of Minecraft modloader versions.
    /// 
    /// # Arguments
    /// 
    /// * `game_version` - The game version to get modloader versions for.
    /// * `modloader_type` - The type of modloader to get versions for.
    /// 
    /// # Returns
    /// 
    /// A list of Minecraft modloader versions.
    pub async fn get_modloaders(
        &self,
        game_version: String,
        modloader_type: &ModLoaderType,
    ) -> Result<Vec<ModLoaderVersion>, CreatorError> {
        let response = self
            .http_client
            .get(format!("{}{}", CURSE_API_URL, "minecraft/modloader").as_str())
            .query(&[("version", game_version.as_str()), ("includeAll", "true")])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(CreatorError::NoModLoaderAvailable);
        }

        let versions: Vec<ModLoaderVersion> = response
            .json::<ModLoaderList>()
            .await?
            .data
            .into_iter()
            .filter(|v| v.type_id == modloader_type.get_id())
            .collect();

        Ok(versions)
    }
}
