#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnState {
    TitleScreen,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
    MessageBox,
}
