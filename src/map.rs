use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub const fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub const fn try_idx(point: Point) -> Option<usize> {
        if Self::in_bounds(point) {
            Some(point_to_index(point.x, point.y))
        } else {
            None
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point) && self.tiles[point_to_index(point.x, point.y)] == TileType::Floor
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if Self::in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[allow(clippy::cast_sign_loss)]
pub const fn point_to_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let location = self.index_to_point2d(idx);

        let deltas = vec![
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];

        deltas
            .iter()
            .filter_map(|d| self.valid_exit(location, *d))
            .map(|idx| (idx, 1.0))
            .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        Self::in_bounds(point)
    }
}
