use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Render)]
#[read_component(Point)]
#[allow(clippy::borrowed_box)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();

    draw_batch.target(0);
    let offset = Point::new(camera.left_x, camera.top_y);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let idx = point_to_index(x, y);

            if Map::in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) || map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    theme.get_darkness()
                };

                let glyph = theme.tile_to_render(map.tiles[idx], idx);
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
