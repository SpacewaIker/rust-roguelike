use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    use TurnState::*;

    *turn_state = match turn_state {
        AwaitingInput => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
    }
}
