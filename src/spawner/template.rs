use crate::prelude::*;

use legion::systems::CommandBuffer;
use log::error;
use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::HashSet;

const TEMPLATE_FILE: &[u8] = include_bytes!("../../resources/template.ron");

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyphs: Vec<u16>,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub fov: Option<i32>,
    pub damage: Option<i32>,
    pub defense: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum EntityType {
    Enemy,
    Item,
    ChestItem,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        from_bytes(TEMPLATE_FILE).expect("Failed to load templates")
    }

    pub fn spawn_entities(
        &mut self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
        game_mode: GameMode,
    ) {
        let mut available_entities = Vec::new();

        self.entities
            .iter()
            .enumerate()
            .filter(|(_, entity)| entity.levels.contains(&level) || game_mode == GameMode::Endless)
            .for_each(|(idx, entity)| {
                for _ in 0..entity.frequency {
                    available_entities.push((idx, entity));
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        let mut spawned_chest = None;

        for &point in spawn_points.iter() {
            'try_spawn_entity: loop {
                let idx = rng
                    .random_slice_index(&available_entities)
                    .expect("No entities to spawn");

                if let Some((i, entity)) = available_entities.get(idx) {
                    if entity.entity_type == EntityType::ChestItem {
                        if spawned_chest.is_some() {
                            continue 'try_spawn_entity;
                        }
                        spawned_chest = Some(*i);
                    }
                    Self::spawn_entity(point, entity, ecs, &mut commands, rng);
                    break 'try_spawn_entity;
                }
            }
        }

        if let Some(idx) = spawned_chest {
            self.entities.swap_remove(idx);
        }

        commands.flush(ecs);
    }

    fn spawn_entity(
        point: Point,
        template: &Template,
        ecs: &mut World,
        commands: &mut CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let glyph = if template.entity_type == EntityType::ChestItem {
            22
        } else {
            *rng.random_slice_entry(&template.glyphs).unwrap()
        };

        let entity = commands.push((
            point,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item),
            EntityType::ChestItem => {
                commands.add_component(entity, Item);
                commands.add_component(entity, ChestItem);
            }
            EntityType::Enemy => {
                let reduce_fov = <&Player>::query()
                    .iter(ecs)
                    .map(|player| player.reduced_visibility)
                    .sum::<i32>();
                commands.add_component(entity, Enemy);
                commands.add_component(
                    entity,
                    FieldOfView::new(template.fov.unwrap_or(6) + reduce_fov),
                );
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.expect("Enemies must have HP"),
                        max: template.hp.expect("Enemies must have HP"),
                    },
                );
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "DungeonMap" => commands.add_component(entity, ChestItemAction::DungeonMap),
                    "ImproveFov" => commands.add_component(entity, ChestItemAction::ImproveFov(*n)),
                    "ImproveDamage" => {
                        commands.add_component(entity, ChestItemAction::ImproveDamage(*n));
                    }
                    "ImproveDefense" => {
                        commands.add_component(entity, ChestItemAction::ImproveDefense(*n));
                    }
                    "ReduceVisibility" => {
                        commands.add_component(entity, ChestItemAction::ReduceVisibility(*n));
                    }
                    "CanSeeEnemies" => {
                        commands.add_component(entity, ChestItemAction::CanSeeEnemies);
                    }
                    _ => error!("Unknown effect: {}", provides),
                });
        }

        if let Some(damage) = &template.damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon);
            }
        }

        if let Some(defense) = &template.defense {
            commands.add_component(entity, Defense(*defense));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Armor);
            }
        }
    }
}
