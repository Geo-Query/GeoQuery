use std::sync::{Arc};
use std::time::Duration;
use rstar::AABB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::index::Node;
use crate::spatial::Region;
use crate::State;
use crate::worker::QueryState::{Complete, Processing};
use tracing::{event, Level, span};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    let worker_span = span!(Level::INFO, "Worker");
    let _worker_span_guard = worker_span.enter();
    loop { // Loop forever, exit via break.
        let task = {
            event!(Level::DEBUG, "Awaiting Lock on Receive Channel! This is where we get new tasks from search()!");
            let mut rx_lck = state.rx.lock().await; // Get next task.
            event!(Level::DEBUG, "Got lock, awaiting task!");
            let Some(task) = rx_lck.recv().await else {
                event!(Level::ERROR, "Closed unexpectedly! Receive Channel returned EOF!");
                break; // If returns None means link closed. Hence break worker.
            };
            event!(Level::INFO, "Got new task from channel!");
            task // Return task.
        };

        event!(Level::DEBUG, "Awaiting WRITE lock on task state, setting to Processing");
        task.write().await.state = Processing;
        event!(Level::DEBUG, "Awaiting READ lock on task, reading region of query!");
        let envelope = AABB::from_corners(task.read().await.region.top_left, task.read().await.region.bottom_right);
        event!(Level::DEBUG, "Awaiting READ lock on index");
        for v in state.i.read().await.locate_in_envelope_intersecting(&envelope) {
            let n = v.clone();
            event!(Level::DEBUG, "Got result: {v:?}");
            event!(Level::DEBUG, "Awaiting WRITE lock on task to add result!");
            task.write().await.results.push(n);
            event!(Level::DEBUG, "Result added!");
            std::thread::sleep(Duration::from_secs(3));
        }
        event!(Level::DEBUG, "Awaiting WRITE lock on task state, setting to Complete");
        task.write().await.state = Complete;
        event!(Level::INFO, "Finished processing task: {task:?}");
    }
}

