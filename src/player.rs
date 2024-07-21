use bevy::{prelude::*, sprite::Anchor};

use crate::{actions::Actions, configs::{TILE_SIZE, WORLD_COLS, WORLD_ROWS}, level::{MapLocation, TileMap}, resources::RoguesTextureAtlas, state::GameState};


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(Update, handle_player_input.run_if(in_state(GameState::Playing)));
    }
}

fn setup_player(
    mut commands: Commands,
    tilemap: Res<TileMap>,
    handle: Res<RoguesTextureAtlas>,
) {
    let row: u32 = 5;
    let col: u32 = 5;
    let x = col * TILE_SIZE;
    let y = (tilemap.height as u32 - row) * TILE_SIZE;
    commands.spawn(
        (MapLocation {row, col},
            SpriteBundle {
                transform: Transform::from_xyz(x as f32, y as f32, 1.0),
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