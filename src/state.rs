use bevy::state::state::States;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the ResourcePlugin will load our assets
    #[default]
    Loading,
    // Generate map and load level
    InitLevel,
    // Allow player to issue command
    PlayerTurn,
    // Run Monster AI
    MonsterTurn,
}
