use crate::errors::Error;

use fujc::minecraft::modloader::ModLoaderType  ;

use super::Questions;

impl Questions {
    pub async fn ask_modloader(&mut self) -> Result<(), Error> {
        let ml = requestty::Question::select("modloader")
            .message("What Mod Loader would you like to use ?")
            .choices(vec!["Fabric", "Forge", "None"])
            .build();

        // Get the answers list item index and text
        let result = requestty::prompt_one(ml)?;
        let result = result.as_list_item();

        match result {
            Some(item) => match item.text.as_str() {
                "Fabric" => {
                    self.mod_loader = Some(ModLoaderType::Fabric);
                }
                "Forge" => {
                    self.mod_loader = Some(ModLoaderType::Forge);
                }
                "None" => {
                    self.mod_loader = None;
                }
                _ => {
                    return Err(Error::InvalidModLoader);
                }
            },
            None => {
                return Err(Error::InvalidModLoader);
            }
        }

        // Check if there is a mod loader version available for the minecraft version
        if let Some(ml) = &self.mod_loader {
            let mc_version = self.mc_version.clone().unwrap();

            let versions = self.curse_api.get_modloaders(mc_version, ml).await.unwrap();

            if versions.len() == 0 {
                return Err(Error::NoModLoaderAvailable);
            }
        }

        Ok(())
    }
}
