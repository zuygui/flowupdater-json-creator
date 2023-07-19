use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ModLoaderType {
    Fabric,
    Forge,
}
