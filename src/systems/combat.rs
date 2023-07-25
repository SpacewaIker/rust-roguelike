use crate::prelude::*;
use log::debug;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Player)]
#[read_component(Damage)]
#[read_component(Defense)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let attacks = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<_>>();

    for (message, attacker, victim) in &attacks {
        let victim_is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let attacker_is_player = ecs
            .entry_ref(*attacker)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if !victim_is_player && !attacker_is_player {
            commands.remove(*message);
            continue;
        }

        let base_damage = ecs.entry_ref(*attacker).map_or(0, |attacker| {
            attacker.get_component::<Damage>().map_or(0, |d| d.0)
        });

        let weapon_damage = if attacker_is_player {
            <&Damage>::query()
                .filter(component::<EquippedWeapon>())
                .iter(ecs)
                .map(|damage| damage.0)
                .sum::<i32>()
        } else {
            0
        };

        let base_defense = ecs.entry_ref(*victim).map_or(0, |victim| {
            victim.get_component::<Defense>().map_or(0, |d| d.0)
        });

        let armor_defense = if victim_is_player {
            <&Defense>::query()
                .filter(component::<EquippedArmor>())
                .iter(ecs)
                .map(|defense| defense.0)
                .sum::<i32>()
        } else {
            0
        };

        let final_damage = i32::max(
            base_damage + weapon_damage - base_defense - armor_defense,
            0,
        );

        let mut score_gain = None;

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            debug!(
                "Victim (player: {}) health: {} -> {}",
                victim_is_player,
                health.current,
                health.current - final_damage
            );
            health.current -= final_damage;

            if health.current < 1 && !victim_is_player {
                score_gain = Some(health.max);
                commands.remove(*victim);
            }
        }

        if let Some(score_gain) = score_gain {
            if let Ok(mut player) = ecs
                .entry_mut(*attacker)
                .unwrap()
                .get_component_mut::<Player>()
            {
                debug!("Player gains {score_gain} score");
                player.score += score_gain;
            }
        }

        commands.remove(*message);
    }
}
