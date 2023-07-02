use std::fmt::Debug;

use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new_mapbuilder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();

        Self::insert_random_noise(rng, &mut mb.map);
        for _ in 0..10 {
            Self::cellular_automata_iteration(&mut mb.map);
        }

        let start = Self::find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    fn insert_random_noise(rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile| {
            *tile = match rng.range(0, 100) {
                0..=45 => TileType::Floor,
                _ => TileType::Wall,
            };
        });
    }

    fn count_neighboring_walls(x: i32, y: i32, map: &Map) -> usize {
        // iterate over the 3x3 grid around the given point and map each tile to a 0 or 1
        (-1..=1)
            .map(|dx| {
                (-1..=1)
                    .map(|dy| {
                        usize::from(map.tiles[point_to_index(x + dx, y + dy)] == TileType::Wall)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            // subtract 1 from the total to account for the center tile
            - usize::from(map.tiles[point_to_index(x, y)] == TileType::Wall)
    }

    fn cellular_automata_iteration(map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let walls_count = Self::count_neighboring_walls(x, y, map);
                let idx = point_to_index(x, y);

                new_tiles[idx] = if walls_count > 4 || walls_count == 0 {
                    TileType::Wall
                } else {
                    TileType::Floor
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        map.tiles
            .iter()
            .filter(|&&t| t == TileType::Floor)
            .enumerate()
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            .map(|(idx, _)| map.index_to_point2d(idx))
            .unwrap()
    }
}

impl Debug for CellularAutomataArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellularAutomataArchitect").finish()
    }
}
