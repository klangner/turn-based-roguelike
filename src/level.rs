use crate::configs::TILE_SIZE;
use crate::resources::TilesTextureAtlas;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct LevelPlugin;

#[derive(Component)]
pub struct Tile {
    x: u32,
    y: u32
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GenerateLevel), spawn_tilemap);
    }
}

fn spawn_tilemap(mut commands: Commands, handle: Res<TilesTextureAtlas>) {
    for x in 0..20 {
        for y in 0..12 {
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
                Tile {x, y},
            ));
        }
    }
}