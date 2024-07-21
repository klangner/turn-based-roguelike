use bevy::state::state::States;


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the ResourcePlugin will load our assets
    #[default]
    Loading,
    // Generate map and load level
    GenerateLevel,
    // During this State the actual game logic is executed
    Playing,
}