use std::fmt::Debug;

use crate::prelude::*;

use super::MapArchitect;

#[allow(clippy::module_name_repetitions)]
pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new_mapbuilder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}

impl Debug for RoomsArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RoomsArchitect").finish()
    }
}
