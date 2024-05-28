use std::collections::HashMap;

use crate::adapter::storage::*;

#[derive(Debug, Clone)]
pub struct MemSelectorSetStorage {
    selector_sets: HashMap<String, SelectorSet>,
}

impl MemSelectorSetStorage {
    pub fn new() -> Self {
        Self {
            selector_sets: HashMap::new(),
        }
    }
}

impl SelectorSetStorage for MemSelectorSetStorage {
    fn define_selector_set(
        &mut self,
        params: &DefineSelectorSetParams,
    ) -> Result<DefineSelectorSetResult, SelectorSetStorageError> {
        for ss in &params.selector_sets {
            self.selector_sets.insert(ss.name.clone(), ss.clone());
        }
        Ok(DefineSelectorSetResult {})
    }

    fn remove_selector_set(
        &mut self,
        params: &RemoveSelectorSetParams,
    ) -> Result<RemoveSelectorSetResult, SelectorSetStorageError> {
        for name in &params.names {
            self.selector_sets.remove(name);
        }
        Ok(RemoveSelectorSetResult { names: params.names.clone() })
    }

    fn list_selector_set<'a>(
        &self,
        params: &ListSelectorSetParams,
    ) -> Result<ListSelectorSetResult, SelectorSetStorageError> {
        let mut result = Vec::new();
        for k in &params.names {
            match self.selector_sets.get(k) {
                Some(value) => result.push(value.clone()),
                None => (),
            }
        }
        Ok(ListSelectorSetResult{ selector_set: result })
    }
}
