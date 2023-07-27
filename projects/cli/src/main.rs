use errors::CreatorError;
use questions::Questions;


mod constants;
mod curse_api;
mod errors;
mod minecraft;
mod questions;

#[tokio::main]
async fn main() -> Result<(), CreatorError> {
    let mut questions = Questions::new()?;

    questions.ask_minecraft().await?;
    questions.ask_modloader().await?;
    questions.ask_mods().await?;

    Ok(())
}
