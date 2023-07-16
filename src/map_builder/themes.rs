use crate::prelude::*;

pub struct DungeonTheme;

impl DungeonTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType, _random: usize) -> FontCharType {
        match tile_type {
            TileType::Floor => 105,
            TileType::Wall => 106,
            TileType::Exit => 0,
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY20
    }
}

pub struct ForestTheme;

impl ForestTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    fn tile_to_render(&self, tile_type: TileType, random: usize) -> FontCharType {
        let mut rng = RandomNumberGenerator::seeded(random as u64);
        match tile_type {
            TileType::Floor => 96,
            TileType::Wall => rng.range(97, 102), // 97-101
            TileType::Exit => 0,
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY80
    }
}
