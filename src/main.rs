use errors::CreatorError;
use questions::Questions;

mod errors;
mod questions;
mod minecraft;
mod curse_api;
mod constants;

#[tokio::main]
async fn main() -> Result<(), CreatorError> {
    let mut questions = Questions::new()?;

    questions.ask_minecraft().await?;
    questions.ask_modloader()?;
    Ok(())
}
