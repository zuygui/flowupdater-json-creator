use crate::{minecraft::{modloader::ModLoaderType, minecraft::McVersion}, curse_api::CurseApi, constants, errors::CreatorError};

mod modloader;
mod mc_versions;

pub struct Questions {
    mod_loader: Option<ModLoaderType>,
    mc_versions: McVersion,

    curse_api: CurseApi,
}

impl Questions {
    pub fn new() -> Result<Self, CreatorError> {
        Ok(Self {
            mod_loader: None,
            mc_versions: McVersion::Latest,
            curse_api: CurseApi::new(constants::TOKEN)?,
        })
    }
}