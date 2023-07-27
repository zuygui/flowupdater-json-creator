use crate::{errors::CreatorError, curse_api::mods::CurseMod, minecraft::Mod};

use super::Questions;

impl Questions {

    async fn select_founded_mod(&self, query: String) -> Option<CurseMod> {
        let mods = self.curse_api
            .search_mod(query, self.mc_version.as_ref().unwrap().to_string(), self.mod_loader.unwrap())
            .await
            .ok()?
            .data
            .iter()
            .map(|mod_| Mod {
                name: mod_.name.clone(),
                id: mod_.id.clone(),
            })
            .collect::<Vec<Mod>>();

        if mods.is_empty() {
            println!("No mods found.");
            return None;
        }

        let choices = requestty::Question::select("mod")
            .message("Which mod do you want to add?")
            .choices(
                mods.iter()
                    .map(|mod_| mod_.name.clone())
                    .collect::<Vec<String>>(),
            )
            .build();

        let binding = requestty::prompt_one(choices).unwrap();
        let mod_data = &mods[binding.as_list_item().unwrap().index];

        let file_id = self.curse_api
            .get_mod_file_id(mod_data.id, mod_data.name.clone(), self.mc_version.as_ref().unwrap().to_string())
            .await
            .ok()?;

        Some(CurseMod {
            name: mod_data.name.clone(),
            mod_id: mod_data.id.clone(),
            file_id: file_id.file_id,
        })
    }

    fn check_if_user_wanna_add_mod(&self) -> bool {
        let add_mod_question = requestty::Question::confirm("add_mod")
            .message("Do you want to add a mod ?")
            .build();

        let binding = requestty::prompt_one(add_mod_question)
            .unwrap()
            .as_bool()
            .unwrap();
        return binding;
    }

    fn add_mod(&self) -> String {
        let what_mod = requestty::Question::input("what_mod")
            .message("What mod do you want to add ?")
            .build();

        let binding = requestty::prompt_one(what_mod).unwrap();
        let mod_name = &binding.as_string();
        mod_name.unwrap().to_string()
    }


    pub async fn ask_mods(&self) -> Result<Vec<CurseMod>, CreatorError> {
        let mut mods : Vec<CurseMod> = Vec::new();
        let mut mod_added:  bool = false;
        loop {
            let add_mod : bool = self.check_if_user_wanna_add_mod();

            if add_mod {
                let mod_name : String = Self::add_mod(self);
                println!("mod_name: {}", mod_name);

                if let Some(founded_mod) = self.select_founded_mod(mod_name).await {
                mods.push(founded_mod);
                mod_added = true; // Un mod a été ajouté
            }
            } else {
                break;
            }
        }

        if !mod_added {
            return Err(CreatorError::NoModAdded);
        }
        Ok(mods)
    }

}