use std::fmt::Debug;
use axum::extract::{Query, State};
use rstar::{Envelope, Point, RTreeObject, RTree, AABB};
use crate::spatial::{Coordinate, Region};
use crate::Index;
use crate::index::Node;

enum QueryErrorKind {
    FooError
}
#[derive(Debug)]
struct QueryRegion {
    top_left: Coordinate,
    bottom_right: Coordinate
}


#[derive(Debug)]
struct PaginatedQueryResponse {
    count: usize,
    results: Vec<Node>
}

impl Region for QueryRegion {
    fn bottom_left(&self) -> Coordinate { (self.top_left.0, self.bottom_right.1) }
    fn bottom_right(&self) -> Coordinate { self.bottom_right }
    fn top_left(&self) -> Coordinate { self.top_left }
    fn top_right(&self) -> Coordinate { (self.bottom_right.0, self.top_left.1) }
}


pub async fn search(State(mut state): State<Index>, Query(query): Query<QueryRegion>) -> Result<PaginatedQueryResponse, QueryErrorKind> {
    let x = state.i.get_mut().drain_in_envelope(AABB::from_corners(query.bottom_right, query.top_left));
    return Ok(PaginatedQueryResponse {
        count: 10,
        results: x.take(10).collect()
    });
}