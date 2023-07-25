pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub map_level: u32,
    pub score: i32,
    pub has_dungeon_map: bool,
    pub reduced_visibility: i32,
    pub can_see_enemies: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChestItem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChestItemAction {
    DungeonMap,
    ImproveFov(i32),
    ImproveDamage(i32),
    ImproveDefense(i32),
    ReduceVisibility(i32),
    CanSeeEnemies,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VictoryAmulet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Carried {
    pub by: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivateChestItem(pub ChestItemAction);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Defense(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weapon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Armor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquippedWeapon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquippedArmor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquippedChestItem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageBox(pub String);
