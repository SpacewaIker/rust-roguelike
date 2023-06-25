use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(VictoryAmulet)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    use TurnState::{AwaitingInput, GameOver, MonsterTurn, PlayerTurn, Victory};

    let (player_died, player_pos) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .map(|(health, pos)| (health.current < 1, pos))
        .unwrap();

    let amulet_pos = <&Point>::query()
        .filter(component::<VictoryAmulet>())
        .iter(ecs)
        .next()
        .unwrap();

    *turn_state = if player_died {
        GameOver
    } else if player_pos == amulet_pos {
        Victory
    } else {
        match turn_state {
            PlayerTurn => MonsterTurn,
            MonsterTurn => AwaitingInput,
            _ => return,
        }
    }
}
