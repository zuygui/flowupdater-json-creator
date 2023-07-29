use crate::errors::Error;

use super::Questions;

impl Questions {
    pub async fn ask_minecraft(&mut self) -> Result<(), Error> {
        // Ask the user if they want to use the latest version or a specific one
        let version_type = requestty::Question::select("version_type")
            .message("What Minecraft version type would you like to use ?")
            .choices(vec!["Latest", "A Specific one"])
            .build();

        // Get the answers list item index and text
        let version_type = requestty::prompt_one(version_type)?;
        let version_type = version_type.as_list_item();

        if version_type.is_none() {
            return Err(Error::InvalidMinecraftVersion);
        }

        let version_type = version_type.unwrap();

        if version_type.text.is_empty() {
            return Err(Error::InvalidMinecraftVersion);
        }

        let versions = self.curse_api.get_minecraft_versions().await.unwrap();
        let versions = versions
            .data
            .iter()
            .map(|v| v.version_string.to_string())
            .collect::<Vec<String>>();

        if version_type.text == "Latest" {
            self.mc_version = Some(versions[0].clone());
            println!("Latest version: {}", versions[0]);
            return Ok(());
        }

        let v = requestty::Question::select("minecraft")
            .message("What Minecraft version would you like to use ?")
            .page_size(10)
            .choices(versions)
            .build();

        // Get the answers list item index and text
        let result = requestty::prompt_one(v)?;
        let result = result.as_list_item();

        match result {
            Some(item) => {
                let version = item.text.to_string();
                self.mc_version = Some(version);
            }
            None => {
                return Err(Error::InvalidMinecraftVersion);
            }
        }

        Ok(())
    }
}
