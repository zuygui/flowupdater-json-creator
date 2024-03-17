use super::common::{Api, modloaders::ModLoaderType};

pub struct ModrinthApi {

}

impl Api for ModrinthApi {
    fn search_mod(&self, search_query: String, mc_version: String, mod_loader: ModLoaderType) {
        todo!()
    }

    fn get_file_id(&self, mod_id: isize, mod_name: String, mc_version: String) {
        todo!()
    }
}