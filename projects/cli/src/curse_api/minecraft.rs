use crate::errors::CreatorError;

use super::{CurseApi, CURSE_API_URL};
use serde::Deserialize;
use time::OffsetDateTime;

/// A Minecraft version struct.
/// 
/// This struct is used to deserialize the response from the CurseForge API.
/// 
/// # Fields
/// 
/// * `id` - The ID of the Minecraft version.
/// * `game_version_id` - The ID of the Minecraft version's game version.
/// * `version_string` - The string representation of the Minecraft version.
/// * `jar_download_url` - The URL to download the Minecraft version's JAR file.
/// * `json_download_url` - The URL to download the Minecraft version's JSON file.
/// * `approved` - Whether or not the Minecraft version is approved.
/// * `date_modified` - The date and time the Minecraft version was last modified.
/// * `game_version_type_id` - The ID of the Minecraft version's game version type.
/// * `game_version_status` - The status of the Minecraft version's game version.
/// * `game_version_type_status` - The status of the Minecraft version's game version type. 
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    /// The ID of the Minecraft version.
    pub id: u32,
    /// The ID of the Minecraft version's game version.
    pub game_version_id: u32,
    /// The string representation of the Minecraft version.
    pub version_string: String,
    /// The URL to download the Minecraft version's JAR file.
    pub jar_download_url: String,
    /// The URL to download the Minecraft version's JSON file.
    pub json_download_url: String,
    /// Whether or not the Minecraft version is approved.
    pub approved: bool,
    /// The date and time the Minecraft version was last modified.
    #[serde(with = "time::serde::rfc3339")]
    pub date_modified: OffsetDateTime,
    /// The ID of the Minecraft version's game version type.
    pub game_version_type_id: u32,
    /// The status of the Minecraft version's game version.
    pub game_version_status: u32,
    /// The status of the Minecraft version's game version type.
    pub game_version_type_status: u32,
}

/// A list of Minecraft versions.
/// 
/// This struct is used to deserialize the response from the CurseForge API.
/// 
/// # Fields
/// 
/// * `data` - The list of Minecraft versions.
#[derive(Debug, serde::Deserialize)]
pub struct MinecraftVersionsList {
    /// The list of Minecraft versions.
    pub data: Vec<MinecraftVersion>
}

impl CurseApi {
    /// Gets a list of Minecraft versions.
    /// 
    /// # Returns
    /// 
    /// A list of Minecraft versions.
    pub async fn get_minecraft_versions(&self) -> Result<MinecraftVersionsList, CreatorError> {
        let versions = self.http_client.get(format!("{}{}", CURSE_API_URL, "minecraft/version").as_str())
            .send()
            .await?
            .json::<MinecraftVersionsList>()
            .await?;

        Ok(versions)
    }
}