use std::{fmt::Debug, sync::{Arc, RwLock}};

pub struct Shared<T: Debug> {
    data: Arc<RwLock<T>>,
}

impl<T:Debug> Shared<T> {
    pub fn new(data: T) -> Self {
        Shared {
            data: Arc::new(RwLock::new(data)),
        }
    }

    pub fn read(&self) -> std::sync::RwLockReadGuard<T> {
        self.data.read().unwrap()
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<T> {
        self.data.write().unwrap()
    }

    pub fn clone(&self) -> Self {
        Shared {
            data: Arc::clone(&self.data),
        }
    }

}

impl <T: Debug> Debug for  Shared<T>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
impl <T: Debug> Clone for  Shared<T>  {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}