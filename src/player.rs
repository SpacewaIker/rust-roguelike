use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);

        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            use VirtualKeyCode::*;
            let delta = match key {
                Left | A => Point::new(-1, 0),
                Right | D => Point::new(1, 0),
                Up | W => Point::new(0, -1),
                Down | S => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;

            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
