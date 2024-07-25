use bevy::math::vec3;
use bevy::{prelude::*, sprite::Anchor};

use crate::actions::Actions;
use crate::level::{MapLocation, TileMap};
use crate::resources::RoguesTextureAtlas;
use crate::state::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(
                Update,
                handle_player_input.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_player(mut commands: Commands, handle: Res<RoguesTextureAtlas>, tilemap: Res<TileMap>) {
    let map_location = MapLocation {
        row: tilemap.start_pos.y,
        col: tilemap.start_pos.x,
    };
    let global_pos = map_location.global_position();
    commands
        .spawn((
            map_location,
            SpriteBundle {
                transform: Transform::from_xyz(global_pos.x, global_pos.y, 1.0),
                texture: handle.image.clone().unwrap(),
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
        ))
        .insert(Player);
}

fn handle_player_input(
    mut player_query: Query<(&mut MapLocation, &mut Transform), With<Player>>,
    tilemap: Res<TileMap>,
    actions: Res<Actions>,
) {
    if player_query.is_empty() {
        return;
    }

    if let Some(dir) = actions.player_movement {
        let (mut map_location, mut transform) = player_query.single_mut();
        let mut new_location = map_location.clone();
        if dir.x > 0. {
            new_location.col = map_location.col + 1;
        }
        if dir.x < 0. && map_location.col > 0 {
            new_location.col = map_location.col - 1;
        }
        if dir.y < 0. {
            new_location.row = map_location.row + 1;
        }
        if dir.y > 0. && map_location.row > 0 {
            new_location.row = map_location.row - 1;
        }

        if new_location != *map_location && tilemap.is_walkable(new_location.col, new_location.row)
        {
            map_location.col = new_location.col;
            map_location.row = new_location.row;
            let global_pos = map_location.global_position();
            transform.translation = vec3(global_pos.x, global_pos.y, 1.0);
        }
    }
}
