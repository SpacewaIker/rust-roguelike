use crate::prelude::*;

pub use title_screen::SelectedButton;

mod chasing;
mod chest_item;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod message_box;
mod movement;
mod player_input;
mod random_move;
mod title_screen;
mod tooltips;
mod use_item;

pub fn build_title_screen_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(title_screen::input_system())
        .flush()
        .add_system(title_screen::render_system())
        .build()
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_item::use_items_system())
        .add_system(chest_item::use_chest_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_item::use_items_system())
        .add_system(chest_item::use_chest_items_system())
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_message_box_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(message_box::input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(message_box::message_box_system())
        .build()
}
