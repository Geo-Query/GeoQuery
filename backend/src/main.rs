use std::collections::HashMap;
use std::future::IntoFuture;
use std::path::PathBuf;
use std::sync::{Arc};
use axum;
use rstar::RTree;
use tokio;
use tokio::sync::{Mutex, mpsc, RwLock};
use uuid::Uuid;
use crate::index::{Node, parse};
use crate::routes::{index, results, search};
use crate::worker::{QueryTask, worker};
use tower::{ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use serde::{Deserialize, Serialize};

mod spatial;
mod index;
mod parsing;
mod routes;
mod worker;
mod io;

const INDEX_ADDRESS: &str = "0.0.0.0:42069";


struct State {
    i: RwLock<RTree<Node>>,
    j: RwLock<HashMap<Uuid, Arc<RwLock<QueryTask>>>>,
    tx: mpsc::UnboundedSender<Arc<RwLock<QueryTask>>>,
    rx: Mutex<mpsc::UnboundedReceiver<Arc<RwLock<QueryTask>>>>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub path: PathBuf
}

#[tokio::main]
async fn main() {
    let files: Vec<Arc<FileMeta>> = vec![]; // Build and place in Arc here!
    let mut idx: RTree<Node> = RTree::new();

    for (i, file) in files.iter().enumerate() {
        println!("Inserted {i}/{} into index.", files.len());
        idx.insert(Node {
            region: parse(file.path.clone()),
            file: file.clone()
        });
    }

    // Open channel between Axum and Worker
    let (tx, rx) = mpsc::unbounded_channel();

    // Build state. This will be shared between threads.
    let state = Arc::new(State {
        i: RwLock::new(idx),
        tx,
        j: RwLock::new(HashMap::new()),
        rx: Mutex::new(rx),
    });

    // Clone arc for use in worker.2
    let shared_state = state.clone();

    // Define Axum app.
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/search", axum::routing::get(search))
        .route("/results", axum::routing::get(results))
        .layer(cors)
        .layer(axum::Extension(state)); // Pass state through to methods (le middleware)


    // Open TCP Transport
    let listener = match tokio::net::TcpListener::bind(INDEX_ADDRESS).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open TCP Listener, reasion: {e:?}");
            panic!();
        }
    };

    // Dispatch tasks.
    let axum_task = axum::serve(listener, app);
    futures::join!(axum_task.into_future(), worker(shared_state));
}
