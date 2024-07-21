use crate::configs::{TILES_COLS, TILE_SIZE, WORLD_COLS, WORLD_ROWS};
use crate::resources::TilesTextureAtlas;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use mapgen::*;

pub struct LevelPlugin;

#[derive(Resource)]
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub start_pos: UVec2,
    walkables: Vec<bool>,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MapLocation {
    pub row: u32,
    pub col: u32
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(TileMap::new())
        .add_systems(OnEnter(GameState::GenerateLevel), spawn_tilemap)
        .add_systems(OnEnter(GameState::GenerateLevel), spawn_tilemap);
    }
}

impl TileMap {
    fn new() -> Self {
        let map = MapBuilder::new(WORLD_COLS as usize, WORLD_ROWS as usize)
            .with(NoiseGenerator::uniform())
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .with(CullUnreachable::new())
            .build();  

        Self { 
            width: map.width, 
            height: map.height, 
            walkables: map.walkables,
            start_pos: map.starting_point.map(|p| UVec2::new(p.x as u32, p.y as u32)).unwrap_or(UVec2::ZERO),
        }
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        if x >= self.width as u32 || y >= self.height as u32 {
            false
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.walkables[idx]
        }
    }
}

fn spawn_tilemap(
    mut commands: Commands, 
    tilemap: Res<TileMap>,
    handle: Res<TilesTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for c in 0..tilemap.width as u32 {
        for r in 0..tilemap.height as u32 {
            let x: u32 = c as u32;
            let y: u32 = WORLD_ROWS - r as u32;
            let index: usize = if tilemap.is_walkable(c, r) {0 + 7 * TILES_COLS as usize} else {0};
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz((x * TILE_SIZE) as f32, ((y-1) * TILE_SIZE) as f32, 0.0),
                    texture: handle.image.clone().unwrap(),
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index,
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
        Vec2::new((self.col * TILE_SIZE) as f32, ((WORLD_ROWS - 1 - self.row) * TILE_SIZE) as f32)
    }
}