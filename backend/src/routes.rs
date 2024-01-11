use std::fmt::Debug;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Query;
use axum::http::StatusCode;
use uuid::Uuid;
use crate::{State, worker::QueryState};
use crate::io::{PaginatedQueryResponse, Pagination, PER_PAGE, QueryRegion, ResultQuery, SearchQueryResponse};
use crate::worker::QueryTask;

pub async fn index() -> &'static str {
    "INDEX ROOT"
}

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

pub async fn results(Extension(state): Extension<Arc<State>>, Query(query): Query<ResultQuery>) -> Result<Json<PaginatedQueryResponse>, (StatusCode, String)> {
    // Lock results table
    let j_lck = state.j.read().await;
    let task = match j_lck.get(&query.uuid) {
            Some(t) => t,
            None => return Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    };
    // Get the task.
    let Some(results) = &task.results else {
        // Handle event where no results for complete task.
        println!("Encountered complete task without results!");
        return Ok(Json(PaginatedQueryResponse {
            status: QueryState::Complete,
            pagination: Some(Pagination {
                count: 0,
                current_page: 0,
                per_page: 0,
            }),
            results: None,
        }));
    };

    match task.state {
        QueryState::Waiting => {
            return Ok(Json::from(PaginatedQueryResponse {
                status: QueryState::Waiting,
                pagination: None,
                results: None,
            }));
        },
        QueryState::Processing => {
            return Ok(Json::from(PaginatedQueryResponse {
                status: QueryState::Processing,
                pagination: Some(Pagination {
                    count: 0,
                    current_page: 0,
                    per_page: PER_PAGE as usize,
                }),
                results: Some(results.to_vec())
            }))
        },
        QueryState::Complete => {
            return Ok(Json::from(PaginatedQueryResponse {
                status: QueryState::Complete,
                pagination: Some(Pagination {
                    count: results.len(),
                    current_page: 0,
                    per_page: PER_PAGE as usize,
                }),
                results: Some(results.to_vec())
            }))
        }

    }
}
