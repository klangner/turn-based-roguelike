use bevy::prelude::*;

use crate::state::GameState;


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(Player);
}