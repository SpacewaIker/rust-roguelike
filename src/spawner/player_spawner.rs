use crate::prelude::*;

use ron::de::from_bytes;
use serde::Deserialize;

const PLAYER_TEMPLATE: &[u8] = include_bytes!("../../resources/player.ron");

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct PlayerTemplate {
    pub health: i32,
    pub fov: i32,
    pub base_damage: i32,
    pub base_defense: i32,
}

impl PlayerTemplate {
    pub fn load() -> Self {
        from_bytes(PLAYER_TEMPLATE).expect("Failed to load player template")
    }
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let player_template = PlayerTemplate::load();

    ecs.push((
        Player {
            map_level: 0,
            score: 0,
            has_dungeon_map: false,
            reduced_visibility: 0,
            can_see_enemies: false,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: 224,
        },
        Health {
            current: player_template.health,
            max: player_template.health,
        },
        Name(String::from("You")),
        FieldOfView::new(player_template.fov),
        Damage(player_template.base_damage),
        Defense(player_template.base_defense),
    ));
}
