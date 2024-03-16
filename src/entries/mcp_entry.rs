use super::Entry;
use cliclack::log;

#[derive(Clone, Eq, PartialEq)]
pub struct MCPEntry;

impl Entry for MCPEntry {
    fn start(&self) {
        log::remark("MCP entry started");
    }
}