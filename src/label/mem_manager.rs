use super::{Manager, ManagerError};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub struct MemManager {
    labels: HashMap<String, LabelValue>,
}

struct LabelValue {
    label: String,
    value: HashMap<String, IDSet>
}

impl LabelValue {
    fn new(label: &String) -> Self {
        LabelValue { label: label.to_string(), value: HashMap::new() }
    }
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}


struct IDSet {
    ids: HashSet<u64>,
}

impl IDSet {
    fn new() -> IDSet {
        IDSet{
            ids: HashSet::new(),
        }
    }
    fn is_empty(&self) -> bool {
        return self.ids.is_empty()
    }
}


impl MemManager {
    fn new() -> MemManager {
        return MemManager{
            labels: HashMap::new(),
        };
    }
}

impl Manager for MemManager {
    fn define_label(&mut self, label_type: &String) -> Result<(), ManagerError> {
        match self.define_label_value(label_type, &Vec::new()) {
            Ok(_)=> Ok(()),
            Err(e)=> Err(e)
        }
    }
    
    fn remove_label(&mut self, label_type: &String) -> Result<(), ManagerError> {
        let value: Option<&LabelValue> = self.labels.get(label_type);
        match value {
            Some(label_value) => {
                if !label_value.is_empty() {
                    return Err(ManagerError::NotEmpty(label_type.to_string()));
                }
                self.labels.remove(label_type);
            }
            _ => ()
        }
        Ok(())
    }
    
    fn define_label_value(&mut self, label_type: &String, label_values: &[String]) -> Result<(), ManagerError> {
        let label_value = self.labels.entry(label_type.to_string()).or_insert_with(|| LabelValue::new(label_type));
        for new_value in label_values {
            label_value.value.entry(new_value.to_string()).or_insert_with(|| IDSet::new());
        }
        Ok(())
    }
    
    fn remove_label_value(&mut self, label_type: &String, label_values: &[String]) -> Result<(), ManagerError> {
        let existed_value= self.labels.get_mut(label_type);
        if existed_value.is_none() {
            return Ok(());
        }
        let existed_value = existed_value.unwrap();
        for remove_value in label_values {
            match existed_value.value.get(remove_value) {
                None=>(),
                Some(value)=>{
                    if value.is_empty() {
                        existed_value.value.remove(remove_value);
                    } else {
                        return Err(ManagerError::NotEmpty(remove_value.to_string()))
                    }
                }
            }
        }
        Ok(())
    }
    
    fn list_label(&self) -> Result<Vec<String>, ManagerError> {
        return Ok(self.labels.keys().cloned().collect())
    }
    
    fn list_label_value(&self, label: &String)->Result<Vec<String>, ManagerError> {
        let values =  self.labels.get(label);
        match values {
            Some(values) => Ok(values.value.keys().cloned().collect()),
            None => Ok(Vec::new())
        }
    }
}


#[cfg(test)]
mod tests {
    use manager::Manager;

    use crate::label::manager;

    use super::MemManager;

    #[test]
    fn test_mem_manager() {
        let mut manager = MemManager::new();
        let author_key = String::from("author");
        let rate_key = String::from("rate");
        assert!(!manager.define_label(&author_key).is_err(), "define a new label type");
        assert!(!manager.define_label(&author_key).is_err(), "redefine a new label type");
        assert_eq!(manager.list_label().unwrap().len(), 1, "only contains one element");
        assert!(!manager.define_label(&rate_key).is_err(), "define another new label type");
        assert_eq!(manager.list_label().unwrap().len(), 2, "ontains two label type now");
    }
}