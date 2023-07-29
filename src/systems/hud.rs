use std::collections::HashMap;

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Damage)]
#[read_component(Defense)]
#[read_component(EquippedWeapon)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // show health bar
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH / 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    let (player, score, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (*entity, player.score, player.map_level))
        .next()
        .unwrap();

    // show map level
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH, 0),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    // show score
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH, 1),
        format!("Score: {score:3}"),
        ColorPair::new(YELLOW, BLACK),
    );

    // show inventory
    let mut items_count = HashMap::new();

    <(&Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, &carried)| carried.by == player)
        .for_each(|(name, _)| {
            if items_count.contains_key(&name.0) {
                *items_count.get_mut(&name.0).unwrap() += 1;
            } else {
                items_count.insert(name.0.clone(), 1);
            }
        });

    let mut sorted = items_count.keys().collect::<Vec<_>>();
    sorted.sort();

    let mut y = 7;
    for name in sorted {
        let count = items_count.get(name).unwrap();
        draw_batch.print_color(
            Point::new(1, y),
            format!("{}: {} (x{})", y - 6, name, count),
            ColorPair::new(WHITE, BLACK),
        );
        y += 1;
    }

    if y > 7 {
        draw_batch.print_color(
            Point::new(1, 6),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    // show equipped weapon
    <(&Name, &Damage)>::query()
        .filter(component::<EquippedWeapon>())
        .iter(ecs)
        .for_each(|(name, damage)| {
            draw_batch.print(
                Point::new(1, 3),
                format!("Current Weapon: {} (+{})", name.0, damage.0),
            );
        });

    // show equipped armor
    <(&Name, &Defense)>::query()
        .filter(component::<EquippedArmor>())
        .iter(ecs)
        .for_each(|(name, defense)| {
            draw_batch.print(
                Point::new(1, 4),
                format!("Current Armor: {} (+{})", name.0, defense.0),
            );
        });

    // show chest items
    let mut y = 4;
    <&Name>::query()
        .filter(component::<EquippedChestItem>())
        .iter(ecs)
        .for_each(|name| {
            draw_batch.print_right(Point::new(SCREEN_WIDTH, y), name.0.clone());
            y += 1;
        });

    if y > 4 {
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH, 3),
            "Special Items",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
