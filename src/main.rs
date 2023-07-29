mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const UNWALKABLE_BORDER_WIDTH: i32 = DISPLAY_WIDTH / 4;
    pub const UNWALKABLE_BORDER_HEIGHT: i32 = DISPLAY_HEIGHT / 4;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::collections::HashSet;

use prelude::*;

embedded_resource!(DUNGEON_FONT, "../resources/mydungeonfont.png");
embedded_resource!(ALAGARD_FONT, "../resources/alagard_fontmap.png");

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    message_box_systems: Schedule,
    title_screen: Schedule,
    templates: Templates,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        let mut templates = Templates::load();
        templates.spawn_entities(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::TitleScreen);
        resources.insert(map_builder.theme);
        resources.insert(SelectedButton::Play);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            message_box_systems: build_message_box_scheduler(),
            title_screen: build_title_screen_scheduler(),
            templates,
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        self.templates
            .spawn_entities(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::TitleScreen);
        self.resources.insert(map_builder.theme);
        self.resources.insert(SelectedButton::Play);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);

        let text = r#"
          _______               ______  _________ _______  ______  
|\     /|(  ___  )|\     /|    (  __  \ \__   __/(  ____ \(  __  \ 
( \   / )| (   ) || )   ( |    | (  \  )   ) (   | (    \/| (  \  )
 \ (_) / | |   | || |   | |    | |   ) |   | |   | (__    | |   ) |
  \   /  | |   | || |   | |    | |   | |   | |   |  __)   | |   | |
   ) (   | |   | || |   | |    | |   ) |   | |   | (      | |   ) |
   | |   | (___) || (___) |    | (__/  )___) (___| (____/\| (__/  )
   \_/   (_______)(_______)    (______/ \_______/(_______/(______/ 
"#;

        for (y, line) in text.lines().enumerate() {
            ctx.print_color_centered(y + 15, RED, BLACK, line);
        }

        ctx.print_centered(SCREEN_HEIGHT - 2, "Press Space or Enter to restart.");

        if matches!(
            ctx.key,
            Some(VirtualKeyCode::Return | VirtualKeyCode::Space)
        ) {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);

        let text = r#"
             _________ _______ _________ _______  _______               
    |\     /|\__   __/(  ____ \\__   __/(  ___  )(  ____ )|\     /|     
    | )   ( |   ) (   | (    \/   ) (   | (   ) || (    )|( \   / )     
    | |   | |   | |   | |         | |   | |   | || (____)| \ (_) /      
    ( (   ) )   | |   | |         | |   | |   | ||     __)  \   /       
     \ \_/ /    | |   | |         | |   | |   | || (\ (      ) (        
      \   /  ___) (___| (____/\   | |   | (___) || ) \ \__   | |        
       \_/   \_______/(_______/   )_(   (_______)|/   \__/   \_/        
 _______  _______          _________ _______           _______  ______  
(  ___  )(  ____ \|\     /|\__   __/(  ____ \|\     /|(  ____ \(  __  \ 
| (   ) || (    \/| )   ( |   ) (   | (    \/| )   ( || (    \/| (  \  )
| (___) || |      | (___) |   | |   | (__    | |   | || (__    | |   ) |
|  ___  || |      |  ___  |   | |   |  __)   ( (   ) )|  __)   | |   | |
| (   ) || |      | (   ) |   | |   | (       \ \_/ / | (      | |   ) |
| )   ( || (____/\| )   ( |___) (___| (____/\  \   /  | (____/\| (__/  )
|/     \|(_______/|/     \|\_______/(_______/   \_/   (_______/(______/ 
"#;

        for (y, line) in text.lines().enumerate() {
            ctx.print_color_centered(y + 10, GOLD, BLACK, line);
        }

        ctx.print_centered(SCREEN_HEIGHT - 2, "Press Space or Enter to restart.");

        if matches!(
            ctx.key,
            Some(VirtualKeyCode::Return | VirtualKeyCode::Space)
        ) {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query().iter(&self.ecs).next().unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carried)| carried.by == player_entity)
            .for_each(|(e, _)| {
                entities_to_keep.insert(*e);
            });

        <Entity>::query()
            .filter(
                component::<EquippedWeapon>()
                    | component::<EquippedChestItem>()
                    | component::<EquippedArmor>(),
            )
            .iter(&self.ecs)
            .for_each(|&e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let player = <&Player>::query().iter(&self.ecs).next().unwrap();
        if player.has_dungeon_map {
            map_builder.map.revealed_tiles.iter_mut().for_each(|v| {
                *v = true;
            });
        }

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_victory_amulet(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        self.templates.spawn_entities(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::TitleScreen => self
                .title_screen
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            TurnState::MessageBox => self
                .message_box_systems
                .execute(&mut self.ecs, &mut self.resources),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    link_resource!(DUNGEON_FONT, "resources/mydungeonfont.png");
    link_resource!(ALAGARD_FONT, "resources/alagard_fontmap.png");

    env_logger::init();

    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("mydungeonfont.png", 32, 32)
        .with_font("alagard_fontmap.png", 16, 16)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "mydungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "mydungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "alagard_fontmap.png")
        .build()?;

    main_loop(context, State::new())
}
