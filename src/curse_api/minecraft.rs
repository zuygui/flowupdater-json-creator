use crate::errors::CreatorError;

use super::{CurseApi, CURSE_API_URL};
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    pub id: u32,
    pub game_version_id: u32,
    pub version_string: String,
    pub jar_download_url: String,
    pub json_download_url: String,
    pub approved: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub date_modified: OffsetDateTime,
    pub game_version_type_id: u32,
    pub game_version_status: u32,
    pub game_version_type_status: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct MinecraftVersionsList {
    pub data: Vec<MinecraftVersion>
}

impl CurseApi {
    pub async fn get_minecraft_versions(&self) -> Result<MinecraftVersionsList, CreatorError> {
        let versions = self.http_client.get(format!("{}{}", CURSE_API_URL, "minecraft/version").as_str())
            .send()
            .await?
            .json::<MinecraftVersionsList>()
            .await?;

        Ok(versions)
    }
}