use log::{debug, error, info};

use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

#[allow(clippy::module_name_repetitions)]
pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        attempts += 1;

        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );
        debug!(
            "Attempting to place prefab at ({}, {})",
            dimensions.x1, dimensions.y1
        );

        let mut can_place = false;
        let mut overlaps_amulet = false;

        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];

            if pt == mb.amulet_start {
                info!("Overwriting amulet start, skipping current placement");
                overlaps_amulet = true;
            } else if distance < 2000.0 && distance > 20.0 {
                can_place = true;
            }
        });

        if overlaps_amulet {
            continue;
        }

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }
    }

    if placement.is_none() {
        info!("Unable to place fortress");
        return;
    }

    let placement = placement.unwrap();
    let string_vec = FORTRESS
        .0
        .chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect::<Vec<_>>();
    let mut i = 0;
    for ty in placement.y..placement.y + FORTRESS.2 {
        for tx in placement.x..placement.x + FORTRESS.1 {
            let idx = point_to_index(tx, ty);
            let c = string_vec[i];
            match c {
                'M' => {
                    mb.map.tiles[idx] = TileType::Floor;
                    mb.monster_spawns.push(Point::new(tx, ty));
                }
                '-' => mb.map.tiles[idx] = TileType::Floor,
                '#' => mb.map.tiles[idx] = TileType::Wall,
                _ => error!("Unknown glyph {}", c),
            }
            i += 1;
        }
    }
}