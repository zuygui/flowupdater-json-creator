#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Mod Loader")]
    InvalidModLoader,

    #[error("Invalid Minecraft Version")]
    InvalidMinecraftVersion,

    #[error("No Mod Loader Available")]
    NoModLoaderAvailable,

    #[error("No Mod Added")]
    NoModAdded,

    #[error("Prompt Error: {0}")]
    PromptError(requestty::ErrorKind),

    #[error("IO Error: {0}")]
    IoError(std::io::Error),

    #[error("{0}")]
    HttpError(reqwest::Error),
}

impl From<requestty::ErrorKind> for Error {
    fn from(err: requestty::ErrorKind) -> Self {
        Error::PromptError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}
