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
    draw_batch.print_centered(1, "Explore the Dungeon. Use WASD or arrows to move. Heal by 1hp by skipping your turn with SPACE");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (*entity, player.map_level))
        .next()
        .unwrap();

    // show map level
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
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
            Point::new(3, 6),
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
                Point::new(3, 3),
                format!("Current Weapon: {} ({})", name.0, damage.0),
            );
        });

    draw_batch.submit(10000).expect("Batch error");
}
