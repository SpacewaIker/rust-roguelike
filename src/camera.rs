use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub const fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_position.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        if player_position.x < self.left_x + UNWALKABLE_BORDER_WIDTH {
            self.left_x = player_position.x - UNWALKABLE_BORDER_WIDTH;
            self.right_x = self.left_x + DISPLAY_WIDTH;
        } else if player_position.x > self.right_x - UNWALKABLE_BORDER_WIDTH {
            self.right_x = player_position.x + UNWALKABLE_BORDER_WIDTH;
            self.left_x = self.right_x - DISPLAY_WIDTH;
        }

        if player_position.y < self.top_y + UNWALKABLE_BORDER_HEIGHT {
            self.top_y = player_position.y - UNWALKABLE_BORDER_HEIGHT;
            self.bottom_y = self.top_y + DISPLAY_HEIGHT;
        } else if player_position.y > self.bottom_y - UNWALKABLE_BORDER_HEIGHT {
            self.bottom_y = player_position.y + UNWALKABLE_BORDER_HEIGHT;
            self.top_y = self.bottom_y - DISPLAY_HEIGHT;
        }
    }
}
