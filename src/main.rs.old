use curse_api::{CurseApiClient, CurseApiClientBuilder, CurseMod};

mod curse_api;
mod json_creator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modloader = requestty::Question::select("modloader")
        .message("What Mod Loader do you use ?")
        .choices(vec!["Forge", "Fabric"])
        .build();

    // get the answer list item index and text
    let modloader = requestty::prompt_one(modloader)?
        .as_list_item()
        .unwrap()
        .index;

    let possibles_versions = match modloader {
        0 => curse_api::FORGE_COMPATIBLES_VERSIONS.to_vec(),
        1 => curse_api::FABRIC_COMPATIBLES_VERSIONS.to_vec(),
        _ => panic!("Invalid modloader"),
    };

    let minecraft_version = requestty::Question::select("minecraft_version")
        .message("What Minecraft version do you use ?")
        .choices(possibles_versions)
        .build();

    let binding = requestty::prompt_one(minecraft_version)?;
    let minecraft_version = &binding.as_list_item().unwrap().text;

    let curse_client: CurseApiClient = CurseApiClientBuilder::new()
        .with_api_token("$2a$10$FSMPrnX2TyC9kluMfAWvHuGqGxa7qKuvXpClTB/vB8LE3fVu9ic9e")
        .with_game_version(minecraft_version.to_string())
        .with_mod_loader(match modloader {
            0 => curse_api::ModLoader::Forge,
            1 => curse_api::ModLoader::Fabric,
            _ => {
                panic!("Invalid modloader");
            }
        })
        .build()?;

    let mut mod_list: Vec<CurseMod> = Vec::new();
    let mut mod_added = false; // Nouvelle variable pour vérifier si au moins un mod a été ajouté

    loop {
        let user_wanna_add_mod = check_if_user_wanna_add_mod();

        if user_wanna_add_mod {
            let mods = add_mod();
            if let Some(founded_mod) = select_founded_mod(curse_client.clone(), mods).await {
                mod_list.push(founded_mod);
                mod_added = true; // Un mod a été ajouté
            }
        } else {
            break;
        }
    }

    if !mod_added {
        println!("No mods were added.");
        return Ok(());
    }

    json_creator::compile_mods_to_json(mod_list);

    println!("The JSON file is successfully created at : ./mods_list.json");

    Ok(())
}

fn check_if_user_wanna_add_mod() -> bool {
    let add_mod_question = requestty::Question::confirm("add_mod")
        .message("Do you want to add a mod ?")
        .build();

    let binding = requestty::prompt_one(add_mod_question)
        .unwrap()
        .as_bool()
        .unwrap();
    return binding;
}

fn add_mod() -> String {
    let what_mod = requestty::Question::input("what_mod")
        .message("What mod do you want to add ?")
        .build();

    let binding = requestty::prompt_one(what_mod).unwrap();
    let mod_name = &binding.as_string();
    mod_name.unwrap().to_string()
}

#[derive(Clone)]
struct Mod {
    pub name: String,
    pub id: isize,
}
async fn select_founded_mod(curse_client: CurseApiClient, query: String) -> Option<CurseMod> {
    let mods = curse_client
        .search_mod(query)
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

    let file_id = curse_client
        .get_mod_file_id(mod_data.id, mod_data.name.clone())
        .await
        .ok()?;

    Some(CurseMod {
        name: mod_data.name.clone(),
        mod_id: mod_data.id.clone(),
        file_id: file_id.file_id,
    })
}
