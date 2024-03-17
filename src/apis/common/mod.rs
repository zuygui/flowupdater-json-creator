pub(crate) mod modloaders;

use serde::Deserialize;
use super::common::modloaders::ModLoaderType;
use time::OffsetDateTime;


pub trait Api {
    fn search_mod(&self, search_query: String, mc_version: String, mod_loader: ModLoaderType);
    fn get_file_id(&self, mod_id: isize, mod_name: String, mc_version: String);
}

/// A Minecraft mod struct.
#[derive(Clone)]
pub struct Mod {
    pub name: String,
    pub id: isize,
}

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
    /// Whether the Minecraft version is approved.
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

