use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum McVersion {
    Latest,
    Version(String),
}