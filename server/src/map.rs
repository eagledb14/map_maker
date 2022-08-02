use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct Map {
    map: Arc<Mutex<Vec<Vec<i16>>>>,
}



impl Map {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(vec![Vec::new()])),
        }
    }

    pub async fn map(&self) -> Vec<Vec<i16>> {
        let some_map = self.map.lock().await;

        some_map.clone()
    }

    pub async fn push(&self, num: i16) {
        let mut map = self.map.lock().await;

        map[0].push(num);
    }

    pub async fn inc(&self) {
        let mut map = self.map.lock().await;

        map[0][0] += 1;
    }
}
