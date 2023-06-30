use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        Name("You"),
        FieldOfView::new(8),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (name, hp, glyph, fov) = match rng.roll_dice(1, 10) {
        1..=8 => ("Goblin", 2, to_cp437('g'), 6),
        _ => ("Orc", 4, to_cp437('o'), 6),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(fov),
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
        Name("Victory Amulet"),
    ));
}
