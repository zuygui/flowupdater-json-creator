use serde::{Serialize, Deserialize};

use crate::{questions::Questions, minecraft::modloader::ModLoaderType, errors::CreatorError};

use super::{CurseApi, CURSE_API_URL};


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

impl CurseApi {
    pub async fn search_mod<T: ToString>(&self, search_query: T, questions: Questions)
        -> Result<SearchModResponse, CreatorError>
        where
            T: Into<String>,
        {
        let url = format!("{}{}", CURSE_API_URL, "mods/search");

        let query = SearchMod {
            game_id: 432,
            game_version: questions.mc_version,
            search_filter: Some(search_query.into()),
            mod_loader_type: questions.mod_loader,
            sort_field: Some(SearchSortField::Name),
        };

        let response = self.http_client.get(&url).query(&query).send().await?;

        Ok(response.json::<SearchModResponse>().await?)
    }
}