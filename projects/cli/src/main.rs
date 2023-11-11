use errors::Error;
use questions::Questions;


mod constants;
mod errors;
mod questions;
mod json_creator;
mod local_files_checker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut questions = Questions::new()?;

    questions.ask_minecraft().await?;
    questions.ask_modloader().await?;
    
    let mods = questions.ask_mods().await?;
    let local_mods = questions.ask_local_mods()?;

    // compile mods to json
    json_creator::compile_mods_to_json(mods, local_mods);

    println!("――――――――――――――――――――――――――――――――――");
    println!("Your mods list has been generated!");
    println!("> Output file: ./mods_list.json");
    println!("――――――――――――――――――――――――――――――――――");

    Ok(())
}
