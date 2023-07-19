#[derive(thiserror::Error, Debug)]
pub enum CreatorError {
    #[error("Invalid Mod Loader")]
    InvalidModLoader,

    #[error("Invalid Minecraft Version")]
    InvalidMinecraftVersion,

    #[error("Prompt Error: {0}")]
    PromptError(requestty::ErrorKind),

    #[error("IO Error: {0}")]
    IoError(std::io::Error),

    #[error("{0}")]
    HttpError(reqwest::Error)
}

impl From<requestty::ErrorKind> for CreatorError {
    fn from(err: requestty::ErrorKind) -> Self {
        CreatorError::PromptError(err)
    }
}

impl From<std::io::Error> for CreatorError {
    fn from(err: std::io::Error) -> Self {
        CreatorError::IoError(err)
    }
}

impl From<reqwest::Error> for CreatorError {
    fn from(err: reqwest::Error) -> Self {
        CreatorError::HttpError(err)
    }
}