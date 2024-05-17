use core::fmt;
use std::collections::HashMap;
use std::error;

#[derive(Debug, Clone)]
pub enum SelectorError {
    NotFound(u64),
}

impl error::Error for SelectorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for SelectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "{:?}:", self),
        }
    }
}

pub trait Selector {
    fn add_label(
        &mut self,
        id: u64,
        label: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, SelectorError>;
    fn remove_label(&mut self, id: u64, label: &Vec<String>)->Result<(), SelectorError>;
    fn get_label(&self, id: u64) -> Result<HashMap<String, String>, SelectorError>;
    fn get_id_by_label(
        &self,
        labels: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<u64>, SelectorError>;
}
