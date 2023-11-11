use crate::{
    constants, errors::Error
};

use fujc::{curse_api::CurseApi, minecraft::modloader::ModLoaderType};

mod mc_versions;
mod modloader;
pub mod local_mods;
pub mod mods;

pub struct Questions {
    pub mod_loader: Option<ModLoaderType>,
    pub mc_version: Option<String>,

    curse_api: CurseApi,
}

impl Questions {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            mod_loader: None,
            mc_version: None,
            curse_api: CurseApi::new(constants::TOKEN).unwrap(),
        })
    }
}
