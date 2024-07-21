use crate::configs::{TILE_SIZE, WORLD_COLS, WORLD_ROWS};
use crate::resources::TilesTextureAtlas;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct LevelPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Component, Debug)]
pub struct MapLocation {
    pub row: u32,
    pub col: u32
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GenerateLevel), spawn_tilemap);
    }
}

fn spawn_tilemap(
    mut commands: Commands, 
    handle: Res<TilesTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for x in 0..WORLD_COLS {
        for y in 0..WORLD_ROWS {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(vec3((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.0)),
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
                MapLocation {row: y, col: x},
            ))
            .insert(Tile);
        }
    }

    next_state.set(GameState::Playing);
}


impl MapLocation {
    pub fn global_position(&self) -> Vec2 {
        Vec2::new((self.col * TILE_SIZE) as f32, ((WORLD_ROWS - self.row) * TILE_SIZE) as f32)
    }
}