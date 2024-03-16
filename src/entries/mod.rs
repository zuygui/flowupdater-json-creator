pub mod mods_entry;
pub mod mcp_entry;

pub trait Entry {
  fn start(&self);
}

#[derive(Eq, PartialEq, Clone)]
pub enum EntryKind {
  ModEntry,
  MCPEntry
}