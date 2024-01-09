use std::fmt::Debug;
use std::sync::Arc;
use axum::{debug_handler, Extension, Json};
use axum::extract::{Query};
use axum::http::StatusCode;
use rstar::{Envelope, Point, RTreeObject, RTree, AABB};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::{State, worker::QueryState};
use crate::spatial::{Coordinate, Region};
use crate::worker::QueryTask;

enum QueryErrorKind {
    FooError
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQueryResponse {
    token: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedQueryResponse {
    pagination: Pagination,
    results: Vec<QueryState>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    count: usize,
    current_page: usize,
    per_page: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRegion {
    top_left_long: f64,
    top_left_lat: f64,
    bottom_right_long: f64,
    bottom_right_lat: f64
}

impl From<QueryRegion> for Region {
    fn from(value: QueryRegion) -> Self {
        Region {
            top_left: (value.top_left_long, value.top_left_lat),
            bottom_right: (value.bottom_right_long, value.bottom_right_lat)
        }
    }
}
pub async fn index() -> &'static str {
    "INDEX ROOT"
}


#[debug_handler]
pub async fn search(Extension(state): Extension<Arc<State>>, Query(query): Query<QueryRegion>) -> Result<Json<SearchQueryResponse>, (StatusCode, String)> {
    let _uuid = Uuid::new_v4();
    return match state.tx.send(QueryTask {
        uuid: _uuid,
        state: QueryState::Waiting,
        region: query.into(),
        results: None
    }) {
        
        Ok(_) => {
            return Ok(Json(SearchQueryResponse {
                token: _uuid
            }));
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
    };
}
