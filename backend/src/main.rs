use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::future::IntoFuture;
use tracing::{event, Level, span};
use std::path::PathBuf;
use std::sync::Arc;
use axum;
use tracing_subscriber;
use rstar::RTree;
use tokio;
use tokio::sync::{mpsc, Mutex, RwLock};
use uuid::Uuid;
use crate::index::Node;
use crate::routes::{index, results, search};
use crate::worker::{QueryTask, worker};
use crate::config::Config;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use parsing::parse;
use std::process::Command;
use geotiff::GeoTiffMap;
use crate::error::RootErrorKind;
use crate::parsing::dted::DTEDMap;
use crate::parsing::geojson::GEOJSONMap;
use crate::parsing::kml::KMLMap;
use crate::parsing::mbtiles::MBTilesMap;
use crate::parsing::gpkg::GPKGMap;
use crate::parsing::shapefile::ShapeFileMap;

mod spatial;
mod index;
mod parsing;
mod routes;
mod worker;
mod io;
mod config;
mod error;

const INDEX_ADDRESS: &str = "0.0.0.0:42069";



#[derive(Debug)]
struct State {
    i: RwLock<RTree<Node>>,
    j: RwLock<HashMap<Uuid, Arc<RwLock<QueryTask>>>>,
    tx: mpsc::UnboundedSender<Arc<RwLock<QueryTask>>>,
    rx: Mutex<mpsc::UnboundedReceiver<Arc<RwLock<QueryTask>>>>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapType {
    GEOTIFF(GeoTiffMap),
    DTED(DTEDMap),
    KML(KMLMap),
    GEOJSON(GEOJSONMap),
    MBTILES(MBTilesMap),
    GPKG(GPKGMap),
    SHAPEFILE(ShapeFileMap)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub path: PathBuf
}



// File traversal logic.
fn traverse(p: PathBuf) -> Result<Vec<MapType>, Box<dyn Error>>{
    let mut build = Vec::new();
    if !p.is_dir() {
        return Err(RootErrorKind::InvalidMapDirectory("cfg.directory is not a directory!".to_string()).into());
    }

    let files: Vec<DirEntry> = p.read_dir()?.map(|f| f.unwrap()).collect();

    for file in files.iter() {
        let path = file.path();
        if path.is_file() {
            let ext = path.extension().and_then(OsStr::to_str);
            if let Some(ext) = ext {
                match ext {
                    "tif" => {
                        build.push(MapType::GEOTIFF(GeoTiffMap {
                            tiff: path.clone(),
                            tfw: files.iter().find(|candidate|
                                candidate.path().extension().and_then(OsStr::to_str).is_some_and(|s| s == "tfw")
                                && candidate.path().file_stem().is_some_and(|s| s == path.file_stem().unwrap())
                            ).map(DirEntry::path),
                            prj: files.iter().find(|candidate|
                                candidate.path().extension().and_then(OsStr::to_str).is_some_and(|s| s == "prj")
                                && candidate.path().file_stem().is_some_and(|s| s == path.file_stem().unwrap())
                            ).map(DirEntry::path),
                        }));
                    },
                    "kml" => build.push(MapType::KML(KMLMap {
                        path,
                    })),
                    "dt1" | "dt2" => build.push(MapType::DTED(DTEDMap {
                        path,
                    })),
                    "geojson" => build.push(MapType::GEOJSON(GEOJSONMap {
                        path,
                    })),
                    "mbtiles" => build.push(MapType::MBTILES(MBTilesMap {
                        path,
                    })),
                    "gpkg" => build.push(MapType::GPKG(GPKGMap {
                        path,
                    })),
                    "shp" => {
                        build.push(MapType::SHAPEFILE (ShapeFileMap {
                            shp: path.clone(),
                            tfw: files.iter().find(|candidate|
                                candidate.path().extension().and_then(OsStr::to_str).is_some_and(|s| s == "tfw")
                                    && candidate.path().file_stem().is_some_and(|s| s == path.file_stem().unwrap())
                            ).map(DirEntry::path),
                            prj: files.iter().find(|candidate|
                                candidate.path().extension().and_then(OsStr::to_str).is_some_and(|s| s == "prj")
                                    && candidate.path().file_stem().is_some_and(|s| s == path.file_stem().unwrap())
                            ).map(DirEntry::path),
                        }));
                    }
                    _ => {

                    }
                }
            } else {
                continue
            }
        } else if path.is_dir() {
          build.append(&mut traverse(path)?)
        } else {
            return Err(RootErrorKind::UnexpectedPathType.into())
        }
    }
    return Ok(build);
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Load Config (Expect in WD)
    let cfg = match std::env::current_dir() {
        Ok(d) => match File::open(d.join("config.json")) {
            Ok(f) => match serde_json::from_reader::<BufReader<File>, Config>(BufReader::new(f)) {
                Ok(cfg) => cfg,
                Err(e) => {
                    event!(Level::ERROR, "Failed to parse config.json, reason: {e:?}");
                    panic!();
                }
            },
            Err(e) => {
                event!(Level::ERROR, "Failed to open config.json in current wd, reason: {e:?}");
                panic!();
            }
        },
        Err(e) => {
            event!(Level::ERROR, "Failed to read current wd, reason: {e:?}");
            panic!();
        }
    };
    event!(Level::INFO, "config.json Loaded from current working directory!");

    if !cfg.directory.exists() {
        event!(Level::ERROR, "Map Directory: {:?}", cfg.directory);
        event!(Level::ERROR, "Does not exist! Please edit in config.json!");
        panic!();
    }


    let files: Vec<Arc<MapType>> = match traverse(cfg.directory) {
        Ok(files) => files.into_iter().map(Arc::new).collect(),
        Err(e) => {
            event!(Level::ERROR, "Failed to traverse files to build index.");
            event!(Level::ERROR, "Reason: {e:?}");
            panic!();
        }
    };

    let index_building = span!(Level::INFO, "Indexing");
    let _index_build_guard = index_building.enter();

    let mut idx: RTree<Node> = RTree::new();
    event!(Level::INFO, "Building Index");
    event!(Level::DEBUG, "Empty Index Initialised!");

    for (_, map) in files.iter().enumerate() {
        match parse(map.clone()) {
            Ok(v) => match v {
                None => {
                    // Ignore if none!
                }
                Some(node) => {
                    event!(Level::DEBUG, "Found & Inserted: {:?}", node);
                    idx.insert(node)
                }
            },
            Err(e) => {
                event!(Level::ERROR, "{:?}", e);
            }
        }
    }
    event!(Level::DEBUG, "Added all found maps to index!");
    drop(_index_build_guard);

    // Open channel between Axum and Worker
    let (tx, rx) = mpsc::unbounded_channel();

    event!(Level::DEBUG, "Building Shared State (For Multithreading)");
    // Build state. This will be shared between threads.
    let state = Arc::new(State {
        i: RwLock::new(idx),
        tx,
        j: RwLock::new(HashMap::new()),
        rx: Mutex::new(rx),
    });

    // Clone arc for use in worker.
    let shared_state = state.clone();

    event!(Level::INFO, "Initializing Axum Web Server.");
    // Define Axum app.
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    event!(Level::WARN, "CORS currently set to allow all! Potential vulnerability, please fix!");

    let app = axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/search", axum::routing::get(search))
        .route("/results", axum::routing::get(results))
        .layer(cors)
        .layer(axum::Extension(state)); // Pass state through to methods (le middleware)

    event!(Level::INFO, "Initialising TCP Socket for Web Server.");
    // Open TCP Transport
    let listener = match tokio::net::TcpListener::bind(INDEX_ADDRESS).await {
        Ok(t) => t,
        Err(e) => {
            event!(Level::ERROR, "Failed to open TCP Listener, reasion: {e:?}");
            panic!();
        }
    };

    // Dispatch tasks.
    let axum_task = axum::serve(listener, app);

    event!(Level::INFO, "Starting Web Server & Parallel Worker!");


    if let Ok(_) = Command::new("electron-refactor").spawn() {
        println!("Launched Frontend!");
    } else {
        println!("Failed to launch frontend!")
    }
    futures::join!(axum_task.into_future(), worker(shared_state));
}


