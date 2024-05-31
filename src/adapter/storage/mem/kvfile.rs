use std::collections::{HashMap, HashSet};

use crate::adapter::storage::*;

#[derive(Debug, Clone)]
pub struct MemFileKVFileStorage {
    default_file: FileItem,
    files: HashMap<u64, FileItem>,
    last_id: u64,
}

impl MemFileKVFileStorage {
    pub fn new() -> Self {
        MemFileKVFileStorage{
            default_file: FileItem::new(0),
            files: HashMap::new(),
            last_id: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct FileItem {
    id: u64,
    kvs: HashMap<String, String>,
}

impl Into<KVFile> for &FileItem {
    fn into(self) -> KVFile {
        KVFile {
            id: self.id,
            label: self
                .kvs
                .iter()
                .map(|(k, v)| KV::new(k.clone(), v.clone()))
                .collect(),
        }
    }
}

impl FileItem {
    fn new(id: u64) -> Self {
        FileItem { id: id, kvs: HashMap::new() }
    }

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

    fn set_labels(&mut self, labels: HashMap<String, String>) {
        for (k, v) in labels {
            self.kvs.insert(k, v);
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
        for (_, item) in &mut self.files {
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
            for (k, v) in &file.1.kvs {
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

impl KVFileStorage for MemFileKVFileStorage {
    fn list_file(&self, params: ListFileParams) -> Result<ListFileResult, KVFileStorageError> {
        let files = self
            .files
            .iter()
            .filter(|f| params.ids.is_empty() || params.ids.contains(&f.0))
            .filter(|f| {
                params.selectors.is_empty()
                    || Selector::is_match_selectors(&params.selectors, &f.1.kvs)
            })
            .map(|f| f.1.into())
            .collect();
        Ok(ListFileResult { files })
    }

    fn add_file(&mut self, params: AddFileParams) -> Result<AddFileResult, KVFileStorageError> {
        let mut new_file = self.default_file.clone();
        for (k, v) in params.label {
            new_file.kvs.insert(k, v);
        }
        self.last_id += 1;
        new_file.id = self.last_id;
        self.files.insert(new_file.id, new_file.clone());
        Ok(AddFileResult {
            id: new_file.id,
            label: new_file.kvs,
        })
    }

    fn remove_file(
        &mut self,
        params: RemoveFileParams,
    ) -> Result<RemoveFileResult, KVFileStorageError> {
        let amount = params
            .ids
            .iter()
            .filter_map(|id| self.files.get(id))
            .count();
        for k in params.ids {
            self.files.remove(&k);
        }
        Ok(RemoveFileResult { amount })
    }

    fn set_label(&mut self, params: SetLabelParams) -> Result<SetLabelResult, KVFileStorageError> {
        match self.files.get_mut(&params.id) {
            Some(v) => {
                v.set_labels(params.label);
                Ok(SetLabelResult {
                    kvs: v.kvs.clone(),
                })
            }
            None => Err(KVFileStorageError::NotFound),
        }
    }
}
