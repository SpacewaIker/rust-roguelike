use log::info;

use crate::prelude::*;

#[system]
#[read_component(ActivateChestItem)]
#[read_component(Enemy)]
#[write_component(Player)]
#[write_component(FieldOfView)]
#[write_component(Damage)]
#[write_component(Defense)]
pub fn use_chest_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let action = <(Entity, &ActivateChestItem)>::query()
        .iter(ecs)
        .map(|(entity, activate_item)| {
            commands.remove(*entity);
            activate_item.0
        })
        .next();

    if action.is_none() {
        return;
    }

    match action.unwrap() {
        ChestItemAction::DungeonMap => {
            info!("Activating dungeon map");
            <&mut Player>::query()
                .iter_mut(ecs)
                .for_each(|player| player.has_dungeon_map = true);
            map.revealed_tiles.iter_mut().for_each(|t| *t = true);
        }
        ChestItemAction::ImproveFov(fov) => {
            info!("Improving fov to {}", fov);
            <&mut FieldOfView>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .for_each(|fov_component| fov_component.radius = fov);
        }
        ChestItemAction::ImproveDamage(damage) => {
            info!("Improving damage by {}", damage);
            let player_damage = <&mut Damage>::query()
                .filter(component::<Damage>())
                .iter_mut(ecs)
                .next()
                .unwrap();
            player_damage.0 += damage;
        }
        ChestItemAction::ImproveDefense(defense) => {
            info!("Improving defense by {}", defense);
            let player_defense = <&mut Defense>::query()
                .filter(component::<Defense>())
                .iter_mut(ecs)
                .next()
                .unwrap();
            player_defense.0 += defense;
        }
        ChestItemAction::ReduceVisibility(amount) => {
            info!("Reducing visibility by {}", amount);
            <&mut Player>::query()
                .iter_mut(ecs)
                .for_each(|player| player.reduced_visibility = amount);

            <&mut FieldOfView>::query()
                .filter(component::<Enemy>())
                .iter_mut(ecs)
                .for_each(|fov_component| fov_component.radius -= amount);
        }
        ChestItemAction::CanSeeEnemies => {
            info!("Can see enemies");
            <&mut Player>::query()
                .iter_mut(ecs)
                .for_each(|player| player.can_see_enemies = true);
        }
    }
}
