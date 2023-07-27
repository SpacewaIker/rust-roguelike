use crate::prelude::*;

mod player_spawner;
mod template;
pub use player_spawner::spawn_player;
pub use template::Templates;

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
