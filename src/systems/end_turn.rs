use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(VictoryAmulet)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    #[allow(clippy::enum_glob_use)]
    use TurnState::*;

    let (player_died, player_pos) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .map(|(health, pos)| (health.current < 1, *pos))
        .unwrap();

    let player_pos_idx = map.point2d_to_index(player_pos);

    let amulet_pos = <&Point>::query()
        .filter(component::<VictoryAmulet>())
        .iter(ecs)
        .copied()
        .next()
        .unwrap_or_else(|| Point::new(-1, -1));

    *turn_state = if player_died {
        GameOver
    } else if player_pos == amulet_pos {
        Victory
    } else if map.tiles[player_pos_idx] == TileType::Exit {
        NextLevel
    } else {
        match turn_state {
            PlayerTurn => MonsterTurn,
            MonsterTurn => AwaitingInput,
            _ => return,
        }
    }
}
