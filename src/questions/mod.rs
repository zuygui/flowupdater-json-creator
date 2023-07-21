use crate::{minecraft::modloader::ModLoaderType, curse_api::CurseApi, constants, errors::CreatorError};

mod modloader;
mod mc_versions;

pub struct Questions {
    mod_loader: Option<ModLoaderType>,
    mc_versions: Option<String>,

    curse_api: CurseApi,
}

impl Questions {
    pub fn new() -> Result<Self, CreatorError> {
        Ok(Self {
            mod_loader: None,
            mc_versions: None,
            curse_api: CurseApi::new(constants::TOKEN)?,
        })
    }
}