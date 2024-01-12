use std::sync::{Arc};
use rstar::AABB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::index::Node;
use crate::spatial::Region;
use crate::State;
use crate::worker::QueryState::{Complete, Processing};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub results: Vec<Node>
}

pub async fn worker(state: Arc<State>) {
    loop { // Loop forever, exit via break.
        let task = {
            let mut rx_lck = state.rx.lock().await; // Get next task.
            let Some(task) = rx_lck.recv().await else {
                break; // If returns None means link closed. Hence break worker.
            };
            task // Return task.
        };

        task.write().await.state = Processing;
        for v in state.i.read().await.locate_in_envelope_intersecting(&AABB::from_corners(task.read().await.region.top_left, task.read().await.region.bottom_right)) {
            let n = v.clone();
            task.write().await.results.push(n);
        }

        task.write().await.state = Complete;
    }
}