use crate::prelude::*;

pub struct DungeonTheme;

impl DungeonTheme {
    pub fn new_boxed() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
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
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
        }
    }

    fn get_darkness(&self) -> (u8, u8, u8) {
        GREY80
    }
}
