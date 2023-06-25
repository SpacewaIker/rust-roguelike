use crate::prelude::*;
use log::debug;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<_>>();

    for (message, victim) in &victims {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            debug!(
                "Victim (player: {}) health: {} -> {}",
                is_player,
                health.current,
                health.current - 1
            );
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    }
}
