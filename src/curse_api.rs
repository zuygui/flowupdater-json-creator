use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.curseforge.com/v1/";
const MINECRAFT_ID: isize = 432;

pub const FORGE_COMPATIBLES_VERSIONS: [&str; 13] = [
    "1.7", "1.8", "1.9", "1.10", "1.11", "1.12", "1.13", "1.14", "1.15", "1.16", "1.17", "1.18",
    "1.19",
];

pub const FABRIC_COMPATIBLES_VERSIONS: [&str; 6] = ["1.14", "1.15", "1.16", "1.17", "1.18", "1.19"];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ModLoader {
    #[serde(rename = "forge")]
    Forge,
    #[serde(rename = "fabric")]
    Fabric,
}

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

pub struct CurseApiClientBuilder {
    game_version: Option<String>,
    mod_loader: Option<ModLoader>,
    api_token: Option<String>,
}

pub struct CurseApiClient {
    game_version: String,
    api_token: String,
    mod_loader: ModLoader,

    http_client: reqwest::Client,
}

impl CurseApiClientBuilder {
    pub fn new() -> Self {
        Self {
            game_version: None,
            mod_loader: None,
            api_token: None,
        }
    }

    pub fn with_api_token<T>(mut self, api_token: T) -> Self
    where
        T: Into<String>,
    {
        self.api_token = Some(api_token.into());
        self
    }

    pub fn with_game_version(mut self, game_version: String) -> Self {
        self.game_version = Some(game_version);
        self
    }

    pub fn with_mod_loader(mut self, mod_loader: ModLoader) -> Self {
        self.mod_loader = Some(mod_loader);
        self
    }

    pub fn build(self) -> Result<CurseApiClient, Box<dyn std::error::Error>> {
        if self.api_token.is_none() {
            return Err("api_token is required".into());
        }

        let token = self.api_token.unwrap();

        Ok(CurseApiClient {
            game_version: self.game_version.unwrap_or("latest".to_string()),
            mod_loader: self.mod_loader.unwrap_or(ModLoader::Forge),
            http_client: reqwest::Client::new(),
            api_token: token,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchModResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataMods>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilesModResponse {
    #[serde(rename = "data")]
    pub data: DataFiles,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataFiles {
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<ModFiles>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModFiles {
    #[serde(rename = "fileId")]
    pub file_id: isize,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataMods {
    pub id: isize,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMod {
    #[serde(rename = "gameId")]
    pub game_id: isize,
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "searchFilter")]
    pub search_filter: Option<String>,
    #[serde(rename = "modLoaderType")]
    pub mod_loader_type: Option<ModLoader>,
    #[serde(rename = "sortField")]
    pub sort_field: Option<SearchSortField>,
}

pub struct CurseMod {
    pub name: String,
    pub mod_id: isize,
    pub file_id: isize,
}

impl Clone for CurseApiClient {
    fn clone(&self) -> Self {
        Self {
            game_version: self.game_version.clone(),
            mod_loader: self.mod_loader.clone(),
            http_client: reqwest::Client::new(),
            api_token: self.api_token.clone(),
        }
    }
}

impl CurseApiClient {
    /**
     * Get the mod id from the mod name
     * @param query The mod name
     */

    pub async fn search_mod<T>(
        &self,
        query: T,
    ) -> Result<SearchModResponse, Box<dyn std::error::Error>>
    where
        T: Into<String>,
    {
        let url = format!("{}{}", BASE_URL, "mods/search");

        let query = SearchMod {
            game_id: MINECRAFT_ID,
            game_version: Some(self.game_version.clone()),
            search_filter: Some(query.into()),
            mod_loader_type: Some(self.mod_loader.clone()),
            sort_field: Some(SearchSortField::Name),
        };

        let response = self
            .http_client
            .get(&url)
            .header("accept", "application/json")
            .header("x-api-key", &self.api_token)
            .query(&query)
            .send()
            .await?;

        Ok(response.json::<SearchModResponse>().await?)
    }

    pub async fn get_mod_file_id(
        &self,
        mod_id: isize,
        mod_name: String,
    ) -> Result<CurseMod, Box<dyn std::error::Error>> {
        let id = mod_id.clone();
        let url = format!("{}{}", BASE_URL, format!("mods/{}", mod_id));

        let response = self
            .http_client
            .get(&url)
            .header("accept", "application/json")
            .header("x-api-key", &self.api_token)
            .send()
            .await?;

        let file_id: Result<FilesModResponse, reqwest::Error> =
            response.json::<FilesModResponse>().await;
        // return Ok(file_id.unwrap().data.latest_files_indexes.iter().find(|&x| x.game_version == self.game_version).unwrap().file_id);
        Ok(CurseMod {
            name: mod_name,
            mod_id: id,
            file_id: file_id
                .unwrap()
                .data
                .latest_files_indexes
                .iter()
                .find(|&x| x.game_version == self.game_version)
                .unwrap()
                .file_id,
        })
    }
}
