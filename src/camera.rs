use bevy::{math::vec3, prelude::*};

use crate::{level::MapLocation, player::Player, state::GameState};


pub struct FollowCameraPlugin;

#[derive(Component)]
struct FollowCamera;


impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(FollowCamera);
}

fn camera_follow_player(
    player_query: Query<&MapLocation, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_location = player_query.single();
    let global_pos = player_location.global_position();

    camera_transform.translation = vec3(global_pos.x, global_pos.y, 0.);
    // camera_transform.translation = camera_transform.translation.lerp(camera_pos, 0.1);
}