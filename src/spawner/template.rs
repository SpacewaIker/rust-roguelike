use crate::prelude::*;

use legion::systems::CommandBuffer;
use log::error;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

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
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening template file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|entity| entity.levels.contains(&level))
            .for_each(|entity| {
                for _ in 0..entity.frequency {
                    available_entities.push(entity);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        for &point in spawn_points.iter() {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                Self::spawn_entity(point, entity, &mut commands, rng);
            }
        }
        commands.flush(ecs);
    }

    fn spawn_entity(
        point: Point,
        template: &Template,
        commands: &mut CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let entity = commands.push((
            point,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: *rng.random_slice_entry(&template.glyphs).unwrap(),
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, FieldOfView::new(template.fov.unwrap_or(6)));
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
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap),
                    _ => error!("Unknown effect: {}", provides),
                });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon);
            }
        }
    }
}
