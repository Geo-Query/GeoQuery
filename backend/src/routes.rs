use std::fmt::Debug;
use axum::extract::Query;
use crate::spatial::{Coordinate, Region};


#[derive(Debug)]
struct QueryRegion {
    top_left: Coordinate,
    bottom_right: Coordinate
}

impl Region for QueryRegion {
    fn bottom_left(&self) -> Coordinate { (self.top_left.0, self.bottom_right.1) }
    fn bottom_right(&self) -> Coordinate { self.bottom_right }
    fn top_left(&self) -> Coordinate { self.top_left }
    fn top_right(&self) -> Coordinate { (self.bottom_right.0, self.top_left.1) }
}

pub fn search(Query(query): Query<QueryRegion>) {

}