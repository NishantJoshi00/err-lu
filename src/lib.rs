pub mod prelude {
    use std::collections::HashMap;

    const MAPPING: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/mapping.bin"));

    pub fn get_connector_mapping(connector: &str) -> Option<HashMap<&str, &str>> {
        let mut mapping: HashMap<&str, HashMap<&str, &str>> = bincode::deserialize(MAPPING).ok()?;

        mapping.remove(connector)
    }

    pub fn get_connector_list() -> Vec<String> {
        let mapping: HashMap<&str, HashMap<&str, &str>> = bincode::deserialize(MAPPING).unwrap();

        mapping.keys().map(|s| s.to_string()).collect()
    }
}

pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn lookup(connector: &str, error_code: &str) -> Option<String> {
        let mapping = prelude::get_connector_mapping(connector)?;

        mapping.get(error_code).map(|s| s.to_string())
    }
}
