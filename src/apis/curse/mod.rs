use super::common::{modloaders::ModLoaderType, Api, MinecraftVersionsList};
use crate::secrets;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

/// A Minecraft mod struct.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct FilesModResponse {
    #[serde(rename = "data")]
    pub data: DataFiles,
}

/// A list of Minecraft mod files.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct DataFiles {
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<ModFiles>,
}

/// A Minecraft mod file struct.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModFiles {
    #[serde(rename = "fileId")]
    pub file_id: isize,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
}

/// A list of Minecraft mods.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchModResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataMods>,
}

/// A Minecraft mod struct.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct DataMods {
    pub id: isize,
    pub name: String,
}

/// A list of search filters.
#[derive(Debug, Deserialize, Serialize)]
pub enum SearchSortField {
    #[serde(rename = "featured")]
    Featured,
    #[serde(rename = "popularity")]
    Popularity,
    #[serde(rename = "lastupdated")]
    LastUpdated,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "author")]
    Author,
    #[serde(rename = "totaldownloads")]
    TotalDownloads,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "game_version")]
    GameVersion,
}

/// A struct for searching for Minecraft mods.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMod {
    #[serde(rename = "gameId")]
    pub game_id: isize,
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "searchFilter")]
    pub search_filter: Option<String>,
    #[serde(rename = "modLoaderType")]
    pub mod_loader_type: Option<ModLoaderType>,
    #[serde(rename = "sortField")]
    pub sort_field: Option<SearchSortField>,
}

/// A Minecraft mod struct.
///
/// This struct is used to deserialize the response from the CurseForge API.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurseMod {
    pub name: String,
    pub mod_id: isize,
    pub file_id: isize,
}

pub struct CurseApi {
    pub http_client: reqwest::Client,
}

impl CurseApi {
    pub fn new() -> CurseApi {
        let mut headers = HeaderMap::new();

        headers.insert("X-Api-Key", secrets::CURSE_API_KEY.parse().unwrap());

        let client = CurseApi {
            http_client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        };

        client
    }

    pub async fn get_minecraft_versions(&self) -> Result<MinecraftVersionsList, reqwest::Error> {
        let url = "https://api.curseforge.com/v1/minecraft/version";
        let response = self.http_client.get(url).send().await?.json().await?;
        Ok(response)
    }
}

impl Api for CurseApi {
    fn search_mod(&self, search_query: String, mc_version: String, mod_loader: ModLoaderType) {
        todo!()
    }

    fn get_file_id(&self, mod_id: isize, mod_name: String, mc_version: String) {
        todo!()
    }
}
