use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
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

    let player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut y = 4;

    <(&Name, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, &carried)| carried.by == *player)
        .for_each(|(name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 3, name.0));
            y += 1;
        });

    if y > 4 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
