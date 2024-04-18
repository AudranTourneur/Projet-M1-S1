use std::{ops::DerefMut, sync::atomic::{AtomicUsize, Ordering}};

use rocket::State;
use tokio::sync::Mutex;

use crate::volumes::{models::VolumeData};

pub(crate) struct HitCount {
    pub count: AtomicUsize,
}

#[get("/count")]
pub fn count_handler(hit_count: &State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    hit_count.count.fetch_add(1, Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}

use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct AllVolumesCacheState {
    pub last_update: u64,
    // pub volumes: Mutex<Vec<VolumeData>>,
    pub volumes: Vec<VolumeData>,
            
}

pub async fn update_volumes_state(state: &Arc<Mutex<AllVolumesCacheState>>) {

    // let mut state = state.lock().await;


    // loop {
    //     let current_timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
    //     let all_volumes = get_all_volumes();

    //     state.last_update = current_timestamp;
    //     state.volumes = all_volumes.await;

    //     tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    // }
}
