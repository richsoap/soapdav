use std::collections::HashMap;

use crate::adapter::storage::*;

#[derive(Debug, Clone)]
pub struct MemFileKVFileStorage {
    pub default_file: fileItem,
    pub files: Vec<fileItem>,
}

#[derive(Debug, Clone)]
struct fileItem {
    id: u64,
    kvs: HashMap<String, String>,
}

impl SelectorStorage for MemFileKVFileStorage {
    fn define_selector(
        &mut self,
        params: &DefineSelectorParams,
    ) -> Result<DefineSelectorResult, SelectorStorageError> {
        self.default_file
            .kvs
            .insert(params.key.clone(), params.default_value.clone());
        for item in &mut self.files {
            if params.set_default_for_history {
                item.kvs.insert(params.key.clone(), params.default_value.clone());
            } else {
                item.kvs.insert(params.key.clone(), String::from(""));
            }
        }
        Ok(DefineSelectorResult {  })
    }

    fn list_selector(
        &self,
        params: &ListSelectorParams,
    ) -> Result<ListSelectorResult, SelectorStorageError> {
        todo!()
    }
}
