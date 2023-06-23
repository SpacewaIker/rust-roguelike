use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Render)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    use VirtualKeyCode::*;

    if let Some(key) = key {
        let (delta, glyph) = match key {
            Left | A => (Point::new(-1, 0), Some(to_cp437('l'))),
            Right | D => (Point::new(1, 0), Some(to_cp437('r'))),
            Up | W => (Point::new(0, -1), Some(to_cp437('u'))),
            Down | S => (Point::new(0, 1), Some(to_cp437('d'))),
            Space => (Point::zero(), None),
            _ => return,
        };

        <(Entity, &Point, &mut Render)>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|(entity, pos, render)| {
                let destination = *pos + delta;

                if let Some(glyph) = glyph {
                    render.glyph = glyph;
                }

                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));

                *turn_state = TurnState::PlayerTurn;
            })
    }
}
