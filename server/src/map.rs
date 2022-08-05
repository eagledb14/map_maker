use tokio::sync::Mutex;
use std::sync::Arc;
use crate::noise;
use chrono::Utc;

#[derive(Debug)]
pub struct Map {
    map: Arc<Mutex<Vec<f32>>>,
    width: i64,
    height: i64,
    seed: i32
}



impl Map {
    pub fn new() -> Self {
        let seed_16 = Utc::now().timestamp_millis() as i16; 
        let seed = seed_16 as i32;

        let width = 10;
        let height = 10;
        Self {
            map: Arc::new(Mutex::new(noise::get_map(width, height,seed))),
            width: width,
            height: height,
            seed: seed
        }
    }

    pub async fn map(&self) -> Vec<f32> {
        let some_map = self.map.lock().await;

        some_map.clone()
    }

    pub async fn push(&self, num: f32) {
        let mut map = self.map.lock().await;

        map.push(num);
    }

    pub async fn inc(&self) {
        let mut map = self.map.lock().await;

        map[0] += 1.;
    }
}
