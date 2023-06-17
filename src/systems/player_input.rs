use crate::prelude::*;

#[system]
#[write_component(Point)]
#[write_component(Render)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    use VirtualKeyCode::*;

    if let Some(key) = key {
        let (delta, glyph) = match key {
            Left | A => (Point::new(-1, 0), to_cp437('l')),
            Right | D => (Point::new(1, 0), to_cp437('r')),
            Up | W => (Point::new(0, -1), to_cp437('u')),
            Down | S => (Point::new(0, 1), to_cp437('d')),
            _ => return,
        };

        <(&mut Point, &mut Render)>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|(pos, render)| {
                let destination = *pos + delta;

                render.glyph = glyph;

                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
    }
}
