use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
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
use crate::config::Config;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use axum::extract::Path;
use rayon::prelude::*;
use tower::Layer;

mod spatial;
mod index;
mod parsing;
mod routes;
mod worker;
mod io;
mod config;

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

fn traverse(p: PathBuf) -> Result<Vec<PathBuf>, String>{
    let mut build = Vec::new();
    if !p.is_dir() {
        return Err("cfg.directory is not a directory!".to_string());
    }
    match p.read_dir() {
        Ok(d) => {
            for e in d.filter_map(Result::ok) {
                if e.path().is_dir() {
                    build.append(&mut traverse(e.path()).unwrap())
                } else if e.path().is_file() {
                    match e.path().extension().and_then(OsStr::to_str) {
                        Some(ext) => match ext {
                            "kml" | "tif" | "dt2" | "dt1" | "geojson"  => build.push(e.path()),
                            _ => continue
                        },
                        None => continue
                    }
                }
            }
            return Ok(build);
        }
        Err(e) => {
            return Err(format!("Failed to iterate over dir: {p:?}"));
        }
    }
}

#[tokio::main]
async fn main() {
    // Load Config (Expect in WD)
    let cfg = match std::env::current_dir() {
        Ok(d) => match File::open(d.join("config.json")) {
            Ok(f) => match serde_json::from_reader::<BufReader<File>, Config>(BufReader::new(f)) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to parse config.json, reason: {e:?}");
                    panic!();
                }
            },
            Err(e) => {
                eprintln!("Failed to open config.json in current wd, reason: {e:?}");
                panic!();
            }
        },
        Err(e) => {
            eprintln!("Failed to read current wd, reason: {e:?}");
            panic!();
        }
    };

    if !cfg.directory.exists() {
        eprintln!("Map Dir: {:?}", cfg.directory);
        eprintln!("Does not exist!");
        panic!();
    }


    let files: Vec<Arc<FileMeta>> = match traverse(cfg.directory) {
        Ok(f) => f.iter().map(|z| Arc::new(FileMeta {path: z.clone()})).collect(),
        Err(e) => {
            eprintln!("Failed to traverse files to build index.");
            eprintln!("Reason: {e:?}");
            panic!();
        }
    };

    let mut idx: RTree<Node> = RTree::new();

    for (mut i, file) in files.iter().enumerate() {
        i += 1;
        println!("Inserted {i}/{} into index.", files.len());
        idx.insert(Node {
            region: match parse(file.path.clone()) {
                Some(r) => r,
                None => continue
            },
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

    println!("Starting Server!");
    // Dispatch tasks.
    let axum_task = axum::serve(listener, app);
    futures::join!(axum_task.into_future(), worker(shared_state));
}
