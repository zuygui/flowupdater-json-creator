use std::path::PathBuf;

use serde::{Serialize, Deserialize};


use crate::errors::Error;

use super::Questions;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalMod {
    /// Name of the mod (ex: "jei-1.20.2.jar")
    pub name: String,
    /// The Download URL (serverUrl + name)
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    /// The SHA1 of the mod
    pub sha1: String,
    /// The size of the mod
    /// (ex: 123456)
    pub size: u64,
}

pub fn dir_to_local_mods(dir_path: PathBuf, server_url: String) -> Result<Vec<LocalMod>, Error> {
    let mut mods = Vec::new();

    // Parse the directory to get all .jar files (**/*.jar)
    let walker = walkdir::WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.path().extension().unwrap() == "jar");

    for entry in walker {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let file_content = std::fs::read(path).unwrap();
       
        let mut hasher = sha1_smol::Sha1::new();

        hasher.update(&file_content);

        let size = path.metadata().unwrap().len();
        let local_mod = LocalMod {
            name: name.clone(),
            download_url: format!("{}/{}", server_url, name),
            sha1: hasher.digest().to_string(),
            size,
        };

        mods.push(local_mod);
    }


    Ok(mods)
}

impl Questions {

    /**
     * Questions are
     * - Do you want to add a local mods from a directory ?
     * - If yes, what is the directory ?
     * - If yes, what is the server url ?
     */

    fn check_if_user_wanna_add_local_mods(&self) -> bool {
        let add_local_mods_question = requestty::Question::confirm("add_local_mods")
            .message("Do you want to add local mods ?")
            .build();

        let binding = requestty::prompt_one(add_local_mods_question)
            .unwrap()
            .as_bool()
            .unwrap();
        return binding;
    }

    fn ask_local_mods_dir(&self) -> PathBuf {
        let local_mods_dir_question = requestty::Question::input("local_mods_dir")
            .message("What is the directory of the local mods ?")
            .build();

        let binding = requestty::prompt_one(local_mods_dir_question).unwrap();
        let local_mods_dir = &binding.as_string();
        PathBuf::from(local_mods_dir.unwrap())
    }

    fn ask_server_url(&self) -> String {
        let server_url_question = requestty::Question::input("server_url")
            .message("What is the server url ?")
            .build();

        let binding = requestty::prompt_one(server_url_question).unwrap();
        let server_url = &binding.as_string();
        server_url.unwrap().to_string()
    }

    pub fn ask_local_mods(&self) -> Result<Vec<LocalMod>, Error> {

        let mut local_mods: Vec<LocalMod> = Vec::new();

        let add_local_mods: bool = self.check_if_user_wanna_add_local_mods();

        if add_local_mods {
            let local_mods_dir: PathBuf = self.ask_local_mods_dir();
            let server_url: String = self.ask_server_url();

            // If the URL ends with a slash, remove it
            let server_url = if server_url.ends_with("/") {
                server_url[..server_url.len() - 1].to_string()
            } else {
                server_url
            };

            local_mods = dir_to_local_mods(local_mods_dir, server_url)?;
        }

        Ok(local_mods)
    }

}