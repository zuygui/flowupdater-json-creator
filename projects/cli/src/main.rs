use errors::CreatorError;
use questions::Questions;


mod constants;
mod curse_api;
mod errors;
mod minecraft;
mod questions;
mod json_creator;

#[tokio::main]
async fn main() -> Result<(), CreatorError> {
    let mut questions = Questions::new()?;

    questions.ask_minecraft().await?;
    questions.ask_modloader().await?;
    
    let mods = questions.ask_mods().await?;

    // compile mods to json
    json_creator::compile_mods_to_json(mods);

    println!("----------------------------------");
    println!("Your mods list has been generated !");
    println!("> Output file: mods_list.json");
    println!("----------------------------------");

    Ok(())
}
