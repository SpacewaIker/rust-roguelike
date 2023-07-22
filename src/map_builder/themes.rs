use crate::prelude::*;

pub struct DungeonTheme;

impl DungeonTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType, random: usize) -> FontCharType {
        let mut rng = RandomNumberGenerator::seeded(random as u64);
        match tile_type {
            TileType::Floor => rng.range(128, 130),
            TileType::Wall => 130,
            TileType::Exit => rng.range(131, 135),
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY30
    }
}

pub struct ForestTheme;

impl ForestTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType, random: usize) -> FontCharType {
        let mut rng = RandomNumberGenerator::seeded(random as u64);
        match tile_type {
            TileType::Floor => 96,
            TileType::Wall => rng.range(97, 102),
            TileType::Exit => rng.range(102, 104),
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY80
    }
}

pub struct CaveTheme;

impl CaveTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for CaveTheme {
    fn tile_to_render(&self, tile_type: TileType, random: usize) -> FontCharType {
        let mut rng = RandomNumberGenerator::seeded(random as u64);
        match tile_type {
            TileType::Floor => 112,
            TileType::Wall => rng.range(113, 115),
            TileType::Exit => 0,
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY20
    }
}
