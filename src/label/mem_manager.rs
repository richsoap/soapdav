use super::{Manager, ManagerError};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub struct MemManager {
    labels: HashMap<String, LabelValue>,
}

struct LabelValue {
    label: String,
    value: HashSet<String>,
}

impl LabelValue {
    fn new(label: &String) -> Self {
        LabelValue {
            label: label.to_string(),
            value: HashSet::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl MemManager {
    fn new() -> MemManager {
        return MemManager {
            labels: HashMap::new(),
        };
    }
}

impl Manager for MemManager {
    fn define_label(&mut self, label_type: &String) -> Result<(), ManagerError> {
        match self.define_label_value(label_type, &Vec::new()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
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
            _ => (),
        }
        Ok(())
    }

    fn define_label_value(
        &mut self,
        label_type: &String,
        label_values: &Vec<String>,
    ) -> Result<(), ManagerError> {
        let label_value = self
            .labels
            .entry(label_type.to_string())
            .or_insert_with(|| LabelValue::new(label_type));
        label_value.value.extend(label_values.iter().cloned());
        Ok(())
    }

    fn remove_label_value(
        &mut self,
        label_type: &String,
        label_values: &Vec<String>,
    ) -> Result<(), ManagerError> {
        let existed_value = self.labels.get_mut(label_type);
        if existed_value.is_none() {
            return Ok(());
        }
        let existed_value = existed_value.unwrap();
        existed_value.value.retain(|a| !label_values.contains(a));
        Ok(())
    }

    fn list_label(&self) -> Result<Vec<String>, ManagerError> {
        return Ok(self.labels.keys().cloned().collect());
    }

    fn list_label_value(&self, label: &String) -> Result<Vec<String>, ManagerError> {
        let values = self.labels.get(label);
        match values {
            Some(values) => Ok(values.value.iter().cloned().collect()),
            None => Err(ManagerError::NotFound(label.to_string())),
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
        // 创建标签类型
        assert!(
            manager.define_label(&author_key).is_ok(),
            "define a new label type"
        );
        assert!(
            manager.define_label(&author_key).is_ok(),
            "redefine a new label type"
        );
        assert_eq!(
            manager.list_label().unwrap().len(),
            1,
            "only contains one element"
        );
        assert!(
            manager.define_label(&rate_key).is_ok(),
            "define another new label type"
        );
        assert_eq!(
            manager.list_label().unwrap().len(),
            2,
            "ontains two label type now"
        );
        // 定义标签值
        assert!(manager
            .define_label_value(
                &author_key,
                &vec![String::from("alice"), String::from("bob")]
            )
            .is_ok());
        assert!(manager
            .define_label_value(&rate_key, &vec![String::from("1")])
            .is_ok());
        assert_eq!(
            manager.list_label_value(&author_key).unwrap().len(),
            2,
            "两个标签值"
        );
        assert_eq!(
            manager.list_label_value(&rate_key).unwrap().len(),
            1,
            "一个标签值"
        );
        // 删除标签值
        assert!(
            manager
                .remove_label_value(&rate_key, &vec![String::from("2")])
                .is_ok(),
            "删除不存在的标签值"
        );
        assert_eq!(
            manager.list_label_value(&rate_key).unwrap().len(),
            1,
            "一个标签值"
        );
        assert!(
            manager
                .remove_label_value(&rate_key, &vec![String::from("1")])
                .is_ok(),
            "删除存在的标签值"
        );
        assert!(
            manager.list_label_value(&rate_key).unwrap().is_empty(),
            "没有标签值"
        );
        // 删除标签
        assert!(
            manager.remove_label(&author_key).is_err(),
            "不能删除不为空的标签"
        );
        assert!(manager.remove_label(&rate_key).is_ok(), "可以删除空标签");
        assert!(manager.remove_label(&rate_key).is_ok(), "可以重复删除空标签");
        // 查询不存在的标签
        assert!(
            manager.list_label_value(&rate_key).is_err(),
            "标签不存在的话会报错"
        );
    }
}
