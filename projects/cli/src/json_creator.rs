use std::io::Write;
use fujc::curse_api::mods::CurseMod;

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

pub fn compile_mods_to_json(mod_list: Vec<CurseMod>) {
    let json = serde_json::json!({
        "curseFiles": mod_list.iter().map(|mod_| {
                        serde_json::json!({
                            "projectId": mod_.mod_id,
                            "fileId": mod_.file_id
                        })
                    }).collect::<Vec<serde_json::Value>>()
    });

    let mut file = std::fs::File::create("mods_list.json").unwrap();
    file.write_all(json.to_string().as_bytes()).unwrap();
}
