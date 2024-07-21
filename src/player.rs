use bevy::prelude::*;

use crate::{actions::Actions, configs::{WORLD_COLS, WORLD_ROWS}, level::MapLocation, state::GameState};


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(Update, handle_player_input.run_if(in_state(GameState::Playing)));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn( MapLocation {row: 0, col: 0})
        .insert(Player);
}


fn handle_player_input(
    mut player_query: Query<&mut MapLocation, With<Player>>,
    actions: Res<Actions>,
) {
    if player_query.is_empty() {
        return;
    }

    if let Some(dir) = actions.player_movement {
        let mut map_location = player_query.single_mut();
        if dir.x > 0. && map_location.col < WORLD_COLS - 1 {
            map_location.col += 1;
        } if dir.x < 0.  && map_location.col > 0 {
            map_location.col -= 1;
        } if dir.y < 0. && map_location.row < WORLD_ROWS - 1 {
            map_location.row += 1;
        } if dir.y > 0. && map_location.row > 0 {
            map_location.row -= 1;
        }
    }
}