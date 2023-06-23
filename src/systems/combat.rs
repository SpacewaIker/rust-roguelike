use crate::prelude::*;
use log::debug;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<_>>();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            debug!("Health before: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            debug!("Health after: {}", health.current);
        }
        commands.remove(*message);
    });
}
