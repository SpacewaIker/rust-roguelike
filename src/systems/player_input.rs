use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[write_component(Render)]
#[write_component(Health)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    #[allow(clippy::enum_glob_use)]
    use VirtualKeyCode::*;

    if let Some(key) = key {
        match key {
            Left | A => move_player(ecs, commands, Point::new(-1, 0), to_cp437('l')),
            Right | D => move_player(ecs, commands, Point::new(1, 0), to_cp437('r')),
            Up | W => move_player(ecs, commands, Point::new(0, -1), to_cp437('u')),
            Down | S => move_player(ecs, commands, Point::new(0, 1), to_cp437('d')),
            // Space => heal_player(ecs),
            G => pick_up_item(ecs, commands),
            Key1 => use_item(0, ecs, commands),
            Key2 => use_item(1, ecs, commands),
            Key3 => use_item(2, ecs, commands),
            Key4 => use_item(3, ecs, commands),
            Key5 => use_item(4, ecs, commands),
            Key6 => use_item(5, ecs, commands),
            Key7 => use_item(6, ecs, commands),
            Key8 => use_item(7, ecs, commands),
            Key9 => use_item(8, ecs, commands),
            _ => return, // Do nothing, but don't skip the turn
        };

        *turn_state = TurnState::PlayerTurn;
    }
}

fn move_player(ecs: &mut SubWorld, commands: &mut CommandBuffer, delta: Point, glyph: u16) {
    let (player_entity, destination, render) = <(Entity, &Point, &mut Render)>::query()
        .filter(component::<Player>())
        .iter_mut(ecs)
        .map(|(entity, pos, render)| (*entity, *pos + delta, render))
        .next()
        .unwrap();

    render.glyph = glyph;

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
    }
}

fn pick_up_item(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let (player, player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .map(|(entity, pos)| (*entity, *pos))
        .next()
        .unwrap();

    <(Entity, &Point)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, &item_pos)| item_pos == player_pos)
        .for_each(|(&entity, _)| {
            commands.remove_component::<Point>(entity);
            commands.add_component(entity, Carried { by: player });
        });
}

fn heal_player(ecs: &mut SubWorld) {
    <&mut Health>::query()
        .filter(component::<Player>())
        .iter_mut(ecs)
        .for_each(|mut health| {
            health.current = i32::min(health.max, health.current + 1);
        });
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player_entity = *<Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let item_entity = <(Entity, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, carried)| carried.by == player_entity)
        .enumerate()
        .filter(|(item_count, _)| *item_count == n)
        .map(|(_, (item, _))| *item)
        .next();

    if let Some(item) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item,
            },
        ));
    }
}
