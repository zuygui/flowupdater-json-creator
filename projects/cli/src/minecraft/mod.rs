pub mod modloader;

/// A Minecraft mod struct.
#[derive(Clone)]
pub struct Mod {
    pub name: String,
    pub id: isize,
}