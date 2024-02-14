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
use crate::worker::{QueryState, QueryTask};
use tracing::{instrument, event, Level, span};

pub async fn index() -> &'static str {
    "INDEX ROOT"
}

pub async fn search(Extension(state): Extension<Arc<State>>, Query(query): Query<QueryRegion>) -> Result<Json<SearchQueryResponse>, (StatusCode, String)> {
    let search_span = span!(Level::INFO, "/search handler");
    let _g = search_span.enter();
    event!(Level::INFO, "Received search request!");
    event!(Level::DEBUG, "Request Content: {query:?}");

    let _uuid = Uuid::new_v4();
    let task = Arc::new(RwLock::new(QueryTask {
        uuid: _uuid.clone(),
        state: Waiting,
        region: query.into(),
        results: Vec::new() // TODO: With capacity?
    }));
    return match state.tx.send(task.clone()) {
        Ok(_) => {
            event!(Level::DEBUG, "Sent task to worker! Adding to lookup and returning token!");
            state.j.write().await.insert(_uuid, task);
            event!(Level::INFO, "Created and Responded with new Task; uuid: {_uuid:?}");
            Ok(Json(SearchQueryResponse {
                token: _uuid.clone()
            }))
        }, Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
    };
}


pub async fn results(Extension(state): Extension<Arc<State>>, Query(query): Query<ResultQuery>) -> Result<Json<PaginatedQueryResponse>, (StatusCode, String)> {
    let results_span = span!(Level::INFO, "/results handler");
    let _g = results_span.enter();
    // Lock results tabletable
    event!(Level::INFO, "Got /results request, for task: {:?}", query.uuid);
    match state.j.read().await.get(&query.uuid) {
        Some(v) => {
            event!(Level::DEBUG, "Awaiting READ lock on lookup table!");
            let v = v.read().await;
            event!(Level::DEBUG, "Got lock, building & paginating results!");
            let to_return: Vec<Node> = v.results.iter().cloned().take(PER_PAGE as usize).map(|x| x.clone()).collect();
            event!(Level::INFO, "Returning current state for task: {:?}", query.uuid);
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
