use std::sync::{Arc};
use rstar::AABB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::index::Node;
use crate::spatial::Region;
use crate::State;
use crate::worker::QueryState::{Complete, Processing};

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryState {
    Waiting,
    Processing,
    Complete
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTask {
    pub uuid: Uuid,
    pub state: QueryState,
    pub region: Region,
    pub results: Option<Vec<Node>>
}

pub async fn worker(state: Arc<State>) {
    println!("FOO");
    loop {
        let mut task = {
            let mut rx_lck = state.rx.lock().await;
            let Some(task) = rx_lck.recv().await else {
                break;
            };
            println!("Task: {:?}", task.uuid);
            task
        };

        task.state = Processing;
        let results: Vec<Node> = state.i.read().await.locate_in_envelope(&AABB::from_corners(task.region.top_left(), task.region.bottom_right())).map(|x| x.clone()).collect();
        task.state = Complete;
        task.results = Some(results);
        {
            let mut j_lck = state.j.write().await;
            j_lck.insert(task.uuid, task);
        }
        println!("Completed Query and added results.");
    }
}