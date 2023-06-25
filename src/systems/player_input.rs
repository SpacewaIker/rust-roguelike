use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Render)]
#[write_component(Health)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    use VirtualKeyCode::{Down, Left, Right, Space, Up, A, D, S, W};

    if let Some(key) = key {
        let (delta, glyph) = match key {
            Left | A => (Point::new(-1, 0), Some(to_cp437('l'))),
            Right | D => (Point::new(1, 0), Some(to_cp437('r'))),
            Up | W => (Point::new(0, -1), Some(to_cp437('u'))),
            Down | S => (Point::new(0, 1), Some(to_cp437('d'))),
            Space => (Point::zero(), None),
            _ => return,
        };

        let (player_entity, destination, render) = <(Entity, &Point, &mut Render)>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .map(|(entity, pos, render)| (*entity, *pos + delta, render))
            .next()
            .unwrap();

        if let Some(glyph) = glyph {
            render.glyph = glyph;
        }

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        } else if let Ok(mut health) = ecs
            .entry_mut(player_entity)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current = i32::min(health.max, health.current + 1);
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
