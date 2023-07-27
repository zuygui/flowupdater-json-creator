use serde::{Serialize, Deserialize};

use crate::{minecraft::modloader::ModLoaderType, errors::CreatorError};

use super::{CurseApi, CURSE_API_URL};


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
pub struct SearchModResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataMods>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataMods {
    pub id: isize,
    pub name: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurseMod {
    pub name: String,
    pub mod_id: isize,
    pub file_id: isize,
}

impl CurseApi {
    pub async fn search_mod<T: ToString>(&self, search_query: T, mc_version: String, mod_loader: ModLoaderType)
        -> Result<SearchModResponse, CreatorError>
        where
            T: Into<String>,
        {
        let url = format!("{}{}", CURSE_API_URL, "mods/search");

        let query = SearchMod {
            game_id: 432,
            game_version: Some(mc_version),
            search_filter: Some(search_query.into()),
            mod_loader_type: Some(mod_loader),
            sort_field: Some(SearchSortField::Name),
        };

        let response = self.http_client.get(&url).query(&query).send().await?;

        Ok(response.json::<SearchModResponse>().await?)
    }

     pub async fn get_mod_file_id(
        &self,
        mod_id: isize,
        mod_name: String,
        mc_version: String
    ) -> Result<CurseMod, Box<dyn std::error::Error>> {
        let id = mod_id.clone();
        let url = format!("{}{}", CURSE_API_URL, format!("mods/{}", mod_id));

        let response = self.http_client.get(&url).send().await?;

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
                .find(|&x| x.game_version == mc_version).unwrap()
                .file_id,
        })
    }

}