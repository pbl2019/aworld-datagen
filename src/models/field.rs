use std::clone::Clone;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[macro_export]
macro_rules! init_field {
    ($e:expr) => {
        Field::new($e)
    };
}

pub struct Field<T: Clone> {
    data: Arc<RwLock<T>>,
}

impl<T: Clone> Field<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }
}

impl<T: Clone> std::clone::Clone for Field<T> {
    fn clone(&self) -> Self {
        Field {
            data: self.data.clone(),
        }
    }
}

impl<T: Clone> Field<T> {
    pub fn write(&mut self, replacer: T) {
        *self.data.write().unwrap() = replacer;
    }
    pub fn read(&self) -> T {
        self.data.read().unwrap().clone()
    }
}

#[test]
fn test_field() {
    let x = Field {
        data: Arc::new(RwLock::new(3)),
    };
    let mut x1 = x.clone();
    let mut x2 = x.clone();
    thread::spawn(move || {
        x1.write(5);
    });
    thread::spawn(move || {
        x2.write(10);
    });
    thread::sleep(Duration::from_millis(50));
    let x_ = x.read();
    assert!(x_ == 5 || x_ == 10);
}
