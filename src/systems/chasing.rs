use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let player_idx = point_to_index(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    <(Entity, &Point)>::query()
        .filter(component::<ChasingPlayer>())
        .iter(ecs)
        .for_each(|(entity, pos)| {
            let idx = point_to_index(pos.x, pos.y);

            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };

                let mut attacked = false;
                <(Entity, &Point)>::query()
                    .filter(component::<Health>())
                    .iter(ecs)
                    .filter(|(_, target_pos)| **target_pos == destination)
                    .for_each(|(victim, _)| {
                        if ecs
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *entity,
                                    victim: *victim,
                                },
                            ));
                        }
                        attacked = true;
                    });

                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        });
}
