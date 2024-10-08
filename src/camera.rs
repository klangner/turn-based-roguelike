use bevy::{math::vec3, prelude::*, window::PrimaryWindow};

use crate::{
    configs::{TILE_SIZE, WORLD_COLS, WORLD_ROWS},
    level::MapLocation,
    player::Player,
    state::GameState,
};

pub struct FollowCameraPlugin;

#[derive(Component)]
pub struct FollowCamera;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::PlayerTurn)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(FollowCamera);
}

fn camera_follow_player(
    player_query: Query<&MapLocation, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if camera_query.is_empty() || player_query.is_empty() || window_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_location = player_query.single();
    let mut global_pos = player_location.global_position();

    let win = window_query.single();
    let half_win_width = win.width() / 2.0;
    let half_win_height = win.height() / 2.0;
    if global_pos.x < half_win_width {
        global_pos.x = half_win_width;
    } else if global_pos.x + half_win_width >= (WORLD_COLS * TILE_SIZE) as f32 {
        global_pos.x = (WORLD_COLS * TILE_SIZE) as f32 - half_win_width;
    }
    if global_pos.y < half_win_height {
        global_pos.y = half_win_height;
    } else if global_pos.y + half_win_height >= (WORLD_ROWS * TILE_SIZE) as f32 {
        global_pos.y = (WORLD_ROWS * TILE_SIZE) as f32 - half_win_height;
    }

    let new_translation = vec3(global_pos.x, global_pos.y, 0.);
    if camera_transform.translation.distance(new_translation) < 2.0 * TILE_SIZE as f32 {
        camera_transform.translation = camera_transform.translation.lerp(new_translation, 0.1);
    } else {
        camera_transform.translation = new_translation;
    }
}
