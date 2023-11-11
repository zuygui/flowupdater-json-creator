use std::io::Write;
use fujc::curse_api::mods::CurseMod;

use crate::questions::local_mods::LocalMod;

/**
 *
 *  Structure of the output JSON file
 *
 * {
 *   "curseFiles": [
 *    {
 *     "projectId": 49084,
 *     "fileId": 981992
 *    },
 *  ]
 * }
 */

pub fn compile_mods_to_json(curse_mod_list: Vec<CurseMod>, local_mod_list: Vec<LocalMod>) {
    let json = serde_json::json!({
        "curseFiles": curse_mod_list.iter().map(|mod_| {
                        serde_json::json!({
                            "projectId": mod_.mod_id,
                            "fileId": mod_.file_id
                        })
                    }).collect::<Vec<serde_json::Value>>(),
        // A "mods" field empty for now
        "mods": local_mod_list.iter().map(|mod_| {
                        serde_json::json!({
                            "name": mod_.name,
                            "downloadUrl": mod_.download_url,
                            "sha1": mod_.sha1,
                            "size": mod_.size
                        })
                    }).collect::<Vec<serde_json::Value>>() //, 
        });

    let mut file = std::fs::File::create("mods_list.json").unwrap();
    file.write_all(json.to_string().as_bytes()).unwrap();
}
