use crate::prelude::*;
use log::debug;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let attacks = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<_>>();

    for (message, attacker, victim) in &attacks {
        let base_damage = ecs.entry_ref(*attacker).map_or(0, |attacker| {
            attacker.get_component::<Damage>().map_or(0, |d| d.0)
        });

        let weapon_damage = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.by == *attacker)
            .map(|(_, damage)| damage.0)
            .sum::<i32>();

        let final_damage = base_damage + weapon_damage;

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
                health.current - final_damage
            );
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    }
}
