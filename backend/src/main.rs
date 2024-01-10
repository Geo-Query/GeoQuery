use std::collections::{HashMap, VecDeque};
use std::future::IntoFuture;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc};
use axum;
use rstar::RTree;
mod spatial;
mod index;
mod parsing;
mod routes;
mod worker;

use crate::spatial::Region;
use tokio;
use serde::{Deserialize};
use tokio::sync::{Mutex, mpsc, RwLock};
use uuid::Uuid;
use crate::index::{Node, parse};
use crate::routes::{index, results, search};
use crate::worker::{QueryState, QueryTask, worker};

const INDEX_ADDRESS: &str = "0.0.0.0:42069";


struct State {
    i: RwLock<RTree<Node>>,
    j: RwLock<HashMap<Uuid, QueryTask>>,
    tx: mpsc::UnboundedSender<QueryTask>,
    rx: Mutex<mpsc::UnboundedReceiver<QueryTask>>
}


#[tokio::main]
async fn main() {

    // Build inputs
    let inputs = HashMap::from([
        ("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Sat Imagery/PlanetSAT_10_0s3_N54W004.tif", parse(PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/Sat Imagery/PlanetSAT_10_0s3_N54W004.tif"))),
        ("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/terrain/DTED/PlanetDEM_1s__W4_N52.dt2", parse(PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Raster/terrain/DTED/PlanetDEM_1s__W4_N52.dt2"))),
        ("/home/ben/uni/psd/teamproj/sample_data/Sample map types/dted/DTED-Checking/TCD_DTED119/DTED/E000/N42.DT1", parse(PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/dted/DTED-Checking/TCD_DTED119/DTED/E000/N42.DT1"))),
        ("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Vector/Kml/luciad.kml", parse(PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Vector/Kml/luciad.kml"))),
        ("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Vector/geojson/world.geojson", parse(PathBuf::from("/home/ben/uni/psd/teamproj/sample_data/Sample map types/Vector/geojson/world.geojson")))
    ]);

    // Build index.
    let mut i = RTree::new();
    for (path, region) in inputs {
        let node = Node {
            region,
            path: PathBuf::from(path)
        };
        i.insert(node);
    }


    let (tx, rx) = mpsc::unbounded_channel();
    let state = Arc::new(State {
        i: RwLock::new(i),
        tx,
        j: RwLock::new(HashMap::new()),
        rx: Mutex::new(rx),
    });

    let shared_state = state.clone();


    let app = axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/search", axum::routing::get(search))
        .route("/results", axum::routing::get(results))
        .layer(axum::Extension(state));


    // Open TCP Transport
    let listener = match tokio::net::TcpListener::bind(INDEX_ADDRESS).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open TCP Listener, reasion: {e:?}");
            panic!();
        }
    };


    let axum_task = axum::serve(listener, app);
    let t = futures::join!(axum_task.into_future(), worker(shared_state));
}
