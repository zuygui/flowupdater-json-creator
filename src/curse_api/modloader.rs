use serde::Deserialize;
use time::OffsetDateTime;

use crate::{errors::CreatorError, minecraft::modloader::ModLoaderType};

use super::{CurseApi, CURSE_API_URL};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLoaderVersion {
    pub name: String,
    pub game_version: String,
    pub latest: bool,
    pub recommended: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub date_modified: OffsetDateTime,
    #[serde(rename = "type")]
    pub type_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct ModLoaderList {
    pub data: Vec<ModLoaderVersion>,
}

impl CurseApi {
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
