use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Damage)]
#[read_component(EquippedWeapon)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // help message
    draw_batch.print_centered(0, "Explore the Dungeon. Use WASD or arrows to move.");

    // show health bar
    draw_batch.bar_horizontal(
        Point::new(0, 2),
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
        Point::new(SCREEN_WIDTH, 2),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    // show score
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH, 3),
        format!("Score: {score:3}"),
        ColorPair::new(YELLOW, BLACK),
    );

    // show inventory
    let mut y = 1;

    <(&Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, &carried)| carried.by == player)
        .for_each(|(name, _)| {
            draw_batch.print(Point::new(3, y + 7), format!("{} : {}", y, name.0));
            y += 1;
        });

    if y > 1 {
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
                Point::new(1, 4),
                format!("Current Weapon: {} ({})", name.0, damage.0),
            );
        });

    // show chest items
    let mut y = 6;
    <&Name>::query()
        .filter(component::<EquippedChestItem>())
        .iter(ecs)
        .for_each(|name| {
            draw_batch.print_right(Point::new(SCREEN_WIDTH, y), name.0.clone());
            y += 1;
        });

    if y > 6 {
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH, 5),
            "Special Items",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
