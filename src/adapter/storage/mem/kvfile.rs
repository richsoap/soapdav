use std::collections::{HashMap, HashSet};

use crate::adapter::storage::*;

#[derive(Debug, Clone)]
pub struct MemFileKVFileStorage {
    default_file: FileItem,
    files: Vec<FileItem>,
}

#[derive(Debug, Clone)]
struct FileItem {
    id: u64,
    kvs: HashMap<String, String>,
}

impl FileItem {
    fn to_selectors(&self, keys: &Vec<String>) -> Selectors {
        self.kvs
            .iter()
            .filter(|(k, _)| keys.is_empty() || keys.contains(k))
            .map(FileItem::to_selector)
            .collect()
    }

    fn to_selector((k, v): (&String, &String)) -> Selector {
        let mut value = HashSet::new();
        value.insert(v.clone());
        Selector {
            key: k.clone(),
            value: value,
        }
    }
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
                item.kvs
                    .insert(params.key.clone(), params.default_value.clone());
            } else {
                item.kvs.insert(params.key.clone(), String::from(""));
            }
        }
        Ok(DefineSelectorResult {})
    }

    fn list_selector(
        &self,
        params: &ListSelectorParams,
    ) -> Result<ListSelectorResult, SelectorStorageError> {
        let default_selectors = self.default_file.to_selectors(&params.key);
        let mut selectors: HashMap<String, Selector> = default_selectors
            .iter()
            .map(|s| (s.key.clone(), s.clone()))
            .collect();
        for file in &self.files {
            for (k, v) in &file.kvs {
                match selectors.get_mut(k) {
                    Some(s) => s.add_value(v.clone()),
                    None => {
                        selectors.insert(k.clone(), Selector::new(k.clone(), v.clone()));
                    }
                };
            }
        }
        Ok(ListSelectorResult {
            default_selector: default_selectors,
            selectors: selectors.iter().map(|(_k, v)| v.clone()).collect(),
        })
    }
}
