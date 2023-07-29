#[derive(thiserror::Error, Debug)]
pub enum CreatorError {
    #[error("Invalid Mod Loader")]
    InvalidModLoader,

    #[error("Invalid Minecraft Version")]
    InvalidMinecraftVersion,

    #[error("No Mod Loader Available")]
    NoModLoaderAvailable,

    #[error("No Mod Added")]
    NoModAdded,

    #[error("IO Error: {0}")]
    IoError(std::io::Error),

    #[error("{0}")]
    HttpError(reqwest::Error),
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
