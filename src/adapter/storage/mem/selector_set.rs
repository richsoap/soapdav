use std::collections::HashMap;

use crate::{adapter::storage::*, Shared};

#[derive(Debug, Clone)]
pub struct MemSelectorSetStorage {
    selector_sets: Shared<HashMap<String, SelectorSet>>,
}

impl MemSelectorSetStorage {
    pub fn new() -> Self {
        Self {
            selector_sets: Shared::new(HashMap::new()),
        }
    }
}

impl SelectorSetStorage for MemSelectorSetStorage {
    fn define_selector_set<'a >(
        &'a  self,
        params: &DefineSelectorSetParams,
    ) -> Result<DefineSelectorSetResult, SelectorSetStorageError> {
        for ss in &params.selector_sets {
            self.selector_sets.write().insert(ss.name.clone(), ss.clone());
        }
        Ok(DefineSelectorSetResult {})
    }

    fn remove_selector_set<'a >(
        &'a  self,
        params: &RemoveSelectorSetParams,
    ) -> Result<RemoveSelectorSetResult, SelectorSetStorageError> {
        for name in &params.names {
            self.selector_sets.write().remove(name);
        }
        Ok(RemoveSelectorSetResult {
            names: params.names.clone(),
        })
    }

    fn list_selector_set<'a>(
        &'a self,
        params: &ListSelectorSetParams,
    ) -> Result<ListSelectorSetResult, SelectorSetStorageError> {
        if params.names.is_empty() {
            return Ok(ListSelectorSetResult {
                selector_set: self.selector_sets.read().values().cloned().collect(),
            });
        }
        let mut result = Vec::new();
        for k in &params.names {
            match self.selector_sets.read().get(k) {
                Some(value) => result.push(value.clone()),
                None => (),
            }
        }
        Ok(ListSelectorSetResult {
            selector_set: result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_selector_set() {
        let storage = MemSelectorSetStorage::new();
        // define selector
        {
            let mut selector_sets = vec![];
            selector_sets.push(SelectorSet::new(&String::from("first")));
            selector_sets.push(SelectorSet::new(&String::from("second")));
            selector_sets.push(SelectorSet::new(&String::from("third")));
            let params = DefineSelectorSetParams {
                selector_sets: selector_sets,
            };
            assert!(storage.define_selector_set(&params).is_ok());
        }
        // list selector
        {
            let params = ListSelectorSetParams{
                names: vec![String::from("first"), String::from("second"), String::from("third")],
            };
            let result = storage.list_selector_set(&params);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().selector_set.len(), 3);
        }
        // remove selector
        {
            let params = RemoveSelectorSetParams{
                names: vec![String::from("first"), String::from("second")],
            };
            let result = storage.remove_selector_set(&params);
            assert!(result.is_ok());
        }
        // list selector
        {
            let params = ListSelectorSetParams{
                names: vec![String::from("first"), String::from("second"), String::from("third")],
            };
            let result = storage.list_selector_set(&params);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().selector_set.len(), 1);
        }
    }
}
