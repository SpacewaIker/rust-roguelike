use log::debug;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
#[read_component(Damage)]
#[read_component(EquippedWeapon)]
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
            Left | A => move_player(ecs, commands, Point::new(-1, 0), 227),
            Right | D => move_player(ecs, commands, Point::new(1, 0), 226),
            Up | W => move_player(ecs, commands, Point::new(0, -1), 225),
            Down | S => move_player(ecs, commands, Point::new(0, 1), 224),
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

            let entry = ecs.entry_ref(entity).expect("Unable to get entry ref");
            let is_weapon = entry.get_component::<Weapon>().is_ok();

            if is_weapon {
                debug!("Picking up weapon");
                let new_damage = entry.get_component::<Damage>().map_or(0, |d| d.0);
                let current_weapon = <(Entity, &Damage)>::query()
                    .filter(component::<EquippedWeapon>())
                    .iter(ecs)
                    .map(|(entity, damage)| (*entity, damage.0))
                    .next();

                if let Some((current_weapon, current_damage)) = current_weapon {
                    if new_damage > current_damage {
                        commands.remove(current_weapon);
                        commands.add_component(entity, EquippedWeapon);
                    }
                } else {
                    commands.add_component(entity, EquippedWeapon);
                }
            } else {
                commands.add_component(entity, Carried { by: player });
            }
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
