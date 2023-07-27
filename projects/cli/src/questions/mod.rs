use crate::{
    constants, curse_api::CurseApi, errors::CreatorError, minecraft::modloader::ModLoaderType,
};

mod mc_versions;
mod modloader;

pub struct Questions {
    pub mod_loader: Option<ModLoaderType>,
    pub mc_version: Option<String>,

    curse_api: CurseApi,
}

impl Questions {
    pub fn new() -> Result<Self, CreatorError> {
        Ok(Self {
            mod_loader: None,
            mc_version: None,
            curse_api: CurseApi::new(constants::TOKEN)?,
        })
    }
}
