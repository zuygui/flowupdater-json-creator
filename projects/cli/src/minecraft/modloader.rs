#[derive(Debug, Clone, Copy)]
pub enum ModLoaderType {
    Fabric,
    Forge,
}

impl ModLoaderType {
    pub fn get_id(&self) -> u32 {
        match self {
            ModLoaderType::Fabric => 4,
            ModLoaderType::Forge => 1,
        }
    }
}
