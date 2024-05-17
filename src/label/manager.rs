use core::fmt;
use std::error;
use std::vec::Vec;

#[derive(Debug, Clone)]

pub enum ManagerError {
    NotEmpty(String),
    Conflict,
    NotFound,
}

impl error::Error for ManagerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEmpty(key) => write!(f, "{:?}: {}", self, key),
            _ => write!(f, "{:?}:", self),
        }
    }
}

pub trait Manager {
    fn define_label(&mut self, label_type: &String) -> Result<(), ManagerError>;
    fn remove_label(&mut self, label_type: &String) -> Result<(), ManagerError>;
    fn define_label_value(
        &mut self,
        label_type: &String,
        label_values: &[String],
    ) -> Result<(), ManagerError>;
    fn remove_label_value(
        &mut self,
        label_type: &String,
        label_values: &[String],
    ) -> Result<(), ManagerError>;
    fn list_label(&self) -> Result<Vec<String>, ManagerError>;
    fn list_label_value(&self, label: &String) -> Result<Vec<String>, ManagerError>;
}
