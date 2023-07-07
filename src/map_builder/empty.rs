use std::fmt::Debug;

use crate::prelude::*;

use super::MapArchitect;

#[allow(clippy::module_name_repetitions)]
pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new_mapbuilder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }

        mb
    }
}

impl Debug for EmptyArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmptyArchitect").finish()
    }
}
