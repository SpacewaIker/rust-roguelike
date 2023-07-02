use std::fmt::Debug;

use log::debug;

use crate::prelude::*;

use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const TOTAL_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = TOTAL_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new_mapbuilder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        Self::drunkard(center, rng, &mut mb.map);
        let mut iterations_count = 0;

        while mb
            .map
            .tiles
            .iter()
            .filter(|&&t| t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            iterations_count += 1;
            debug!("Drunkard's walk iteration: {}", iterations_count);
            Self::drunkard(
                Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            )
            .map
            .iter()
            .enumerate()
            .filter(|(_, &distance)| distance > 2000.0)
            .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(start: Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start;
        let mut distance_staggered = 0;

        while map.in_bounds(drunkard_pos) && distance_staggered < STAGGER_DISTANCE {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            distance_staggered += 1;
        }
    }
}

impl Debug for DrunkardsWalkArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DrunkardsWalkArchitect").finish()
    }
}
