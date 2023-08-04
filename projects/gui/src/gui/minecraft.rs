use std::collections::HashMap;

use qmetaobject::{prelude::*, USER_ROLE};

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct MinecraftVersions {
    base: qt_base_class!(trait QAbstractListModel),
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),
    
    versions: Vec<String>,
    versions_changed: qt_signal!(),

    getMinecraftVersions: qt_method!(fn(&mut self))
}

impl MinecraftVersions {
    #[allow(non_snake_case)]
    fn getMinecraftVersions(&mut self) {
        let mut versions = Vec::<String>::new();

        (self as &mut dyn QAbstractListModel).begin_insert_rows(0 as i32, (3 - 1) as i32);
        versions.push("1.16.5".to_string());
        versions.push("1.16.4".to_string());
        versions.push("1.16.3".to_string());

        log::debug!("Versions: {:?}", versions);
        
        self.versions = versions;
        (self as &mut dyn QAbstractListModel).end_insert_rows();

        self.versions_changed();
        self.count_changed();
    }
}

impl QAbstractListModel for MinecraftVersions {
    fn row_count(&self) -> i32 {
        self.versions.len() as i32
    }

    fn data(&self, index: QModelIndex, _: i32) -> QVariant {
        let idx = index.row() as usize;

        if idx < self.versions.len() {
            QString::from(self.versions[idx].clone()).into()
        } else {
            QVariant::default()
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(USER_ROLE, "version".into());
        roles
    }
}