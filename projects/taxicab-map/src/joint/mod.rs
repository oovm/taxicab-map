use crate::{point::Point, Direction};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    pub point: Point,
    pub direction: Direction,
}

impl Joint {
    pub fn new(point: &Point, direction: Direction) -> Self {
        Self { point: *point, direction }
    }

    pub fn source(&self) -> Point {
        self.point
    }
    pub fn target(&self) -> Point {
        self.point.go(self.direction)
    }
}