use crate::prelude::*;

mod template;
pub use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
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
            current: 100,
            max: 100,
        },
        Name(String::from("You")),
        FieldOfView::new(8),
        Damage(1),
        Defense(1),
    ));
}

pub fn spawn_victory_amulet(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        VictoryAmulet,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name(String::from("Victory Amulet")),
    ));
}
