use cstr::cstr;
use qmetaobject::qml_register_type;

use self::minecraft::MinecraftVersions;

mod minecraft;

pub fn register_types() {
    qml_register_type::<MinecraftVersions>(cstr!("MinecraftVersions"), 1, 0, cstr!("MinecraftVersions"))
}