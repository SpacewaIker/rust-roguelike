use crate::prelude::*;

mod template;
use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player {
            map_level: 0,
            score: 0,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        Name(String::from("You")),
        FieldOfView::new(8),
        Damage(1),
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

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}
