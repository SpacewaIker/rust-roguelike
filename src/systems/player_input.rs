use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    use VirtualKeyCode::*;

    if let Some(key) = key {
        let delta = match key {
            Left | A => Point::new(-1, 0),
            Right | D => Point::new(1, 0),
            Up | W => Point::new(0, -1),
            Down | S => Point::new(0, 1),
            _ => return,
        };

        <&mut Point>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|pos| {
                let destination = *pos + delta;

                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
    }
}
