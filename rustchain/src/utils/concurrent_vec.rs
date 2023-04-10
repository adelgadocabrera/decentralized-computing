use std::sync::{Arc, Mutex};

pub struct ConcurrentVec<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T: Clone> ConcurrentVec<T> {
    pub fn new() -> Self {
        ConcurrentVec {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        data.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        data.pop()
    }

    pub fn peek_last(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.last().cloned()
    }

    pub fn peek_first(&self) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.first().cloned()
    }

    pub fn flush(&self) -> Vec<T> {
        let mut data = self.data.lock().unwrap();
        let result = data.drain(..).collect();
        result
    }
}

#[macro_export]
macro_rules! concurrentvec {
    ($($item:expr),*) => {{
        let vec = ConcurrentVec::new();
        $(
            vec.push($item);
        )*
        vec
    }};
}
