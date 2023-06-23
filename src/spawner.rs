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
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (name, hp, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => ("Goblin", 2, to_cp437('g')),
        _ => ("Orc", 4, to_cp437('o')),
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
    ));
}
