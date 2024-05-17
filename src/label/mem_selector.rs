use std::collections::HashMap;

use super::{Selector, SelectorError};

struct Item {
    label: HashMap<String,String>,
}

impl Item {
    fn new() -> Self {
        Item { label: HashMap::new() }
    }
    fn is_match(&self, label_key: &String, label_values: &Vec<String>) -> bool {
        match self.label.get(label_key) {
            Some(label_value) => label_values.contains(label_value),
            None => false
        }
    }
}

pub struct MemSelector {
    items: HashMap<u64, Item>,
}

impl Selector for MemSelector {
    fn add_label(
        &mut self,
        id: u64,
        new_label: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>,SelectorError> {
        let item = self.items.entry(id).or_insert_with(Item::new);
        item.label.extend(new_label.iter().map(|(k,v)| (k.clone(),v.clone())));
        return Ok(item.label.clone());
    }

    fn remove_label(&mut self, id: u64, label: &Vec<String>)->Result<(), super::selector::SelectorError> {
        if let Some(item) = self.items.get_mut(&id) {
            item.label.retain(|k,_| !label.contains(k));
        }
        Ok(())
    }

    fn get_label(&self, id: u64) -> Result<HashMap<String, String>, super::selector::SelectorError> {
        let item = self.items.get(&id);
        match item {
            Some(item) => Ok(item.label.clone()),
            None => Err(SelectorError::NotFound(id)),
        }
    }

    fn get_id_by_label(
        &self,
        labels: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<u64>, super::selector::SelectorError> {
        let mut result = Vec::new();
        for (id, item) in &self.items {
            let mut is_match = true;
            for (k,vs) in labels {
                is_match = is_match && item.is_match(k, vs);
                if !is_match {
                    break;
                }
            }
            if is_match {
                result.push(id.clone());
            }
        }
        Ok(result)
    }
}

