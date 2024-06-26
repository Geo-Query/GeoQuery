use crate::index::Node;
use crate::worker::QueryState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PER_PAGE: usize = 50;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQueryResponse {
    pub token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedQueryResponse {
    pub status: QueryState,
    pub pagination: Pagination,
    pub results: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub count: usize,
    pub current_page: usize,
    pub per_page: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRegion {
    pub top_left_long: f64,
    pub top_left_lat: f64,
    pub bottom_right_long: f64,
    pub bottom_right_lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultQuery {
    pub uuid: Uuid,
}
