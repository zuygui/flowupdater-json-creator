use cliclack::log;
use super::Entry;

#[derive(Clone, Eq, PartialEq)]
pub struct ModsEntry {}

impl Entry for ModsEntry {
  fn start(&self) {

    log::remark("Mod entry started");

  }
}