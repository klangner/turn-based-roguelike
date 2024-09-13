use crate::GameState;
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), game_over);
    }
}

fn game_over(mut _next_state: ResMut<NextState<GameState>>) {
    // next_state.set(GameState::PlayerTurn);
}
