use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Query;
use axum::http::StatusCode;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::State;
use crate::index::Node;
use crate::io::{PaginatedQueryResponse, Pagination, PER_PAGE, QueryRegion, ResultQuery, SearchQueryResponse};
use crate::worker::QueryState::Waiting;
use crate::worker::QueryTask;

pub async fn index() -> &'static str {
    "INDEX ROOT"
}

pub async fn search(Extension(state): Extension<Arc<State>>, Query(query): Query<QueryRegion>) -> Result<Json<SearchQueryResponse>, (StatusCode, String)> {
    let _uuid = Uuid::new_v4();
    let task = Arc::new(RwLock::new(QueryTask {
        uuid: _uuid.clone(),
        state: Waiting,
        region: query.into(),
        results: Vec::new() // TODO: With capacity?
    }));
    return match state.tx.send(task.clone()) {
        Ok(_) => {
            state.j.write().await.insert(_uuid, task);
            Ok(Json(SearchQueryResponse {
                token: _uuid.clone()
            }))
        }, Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
    };
}



pub async fn results(Extension(state): Extension<Arc<State>>, Query(query): Query<ResultQuery>) -> Result<Json<PaginatedQueryResponse>, (StatusCode, String)> {
    // Lock results table
    match state.j.read().await.get(&query.uuid) {
        Some(v) => {
            let v = v.read().await;
            let to_return: Vec<Node> = v.results.iter().cloned().take(PER_PAGE as usize).map(|x| x.clone()).collect();

            // Have results here
            return Ok(Json(PaginatedQueryResponse {
                status: v.state.clone(),
                pagination: Pagination {
                    count: v.results.len(),
                    current_page: to_return.len(),
                    per_page: PER_PAGE as usize,
                },
                results: to_return,
            }));
        },
        None => return Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}
