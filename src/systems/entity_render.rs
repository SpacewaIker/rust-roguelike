use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .filter(!component::<Enemy>())
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    let can_see_enemies = <&Player>::query()
        .iter(ecs)
        .any(|player| player.can_see_enemies);

    <(&Point, &Render)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos) || can_see_enemies)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
