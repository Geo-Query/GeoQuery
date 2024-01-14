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
            println!("Worker awaiting lock on receiver.");
            let mut rx_lck = state.rx.lock().await; // Get next task.
            println!("Worker got lock on receiver!");
            let Some(task) = rx_lck.recv().await else {
                println!("Worker CLOSED!");
                break; // If returns None means link closed. Hence break worker.
            };
            println!("Worker got TASK!");
            task // Return task.
        };
        println!("Worker awaiting write to task state!");
        task.write().await.state = Processing;
        println!("Worker wrote to task state!");
        println!("Worker awaiting lock on index, task. Task lock remains in scope!");
        for v in state.i.read().await.locate_in_envelope_intersecting(&AABB::from_corners(task.read().await.region.top_left.clone(), task.read().await.region.bottom_right.clone())) {
            let n = v.clone();
            println!("Worker awaiting lock on task!");
            task.write().await.results.push(n);
        }
        println!("Worker awaiting write to task state 2!");
        task.write().await.state = Complete;
        println!("Worker wrote to task state 2!");

    }
}