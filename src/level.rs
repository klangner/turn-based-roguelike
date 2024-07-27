use crate::configs::{TILES_COLS, TILE_SIZE, WORLD_COLS, WORLD_ROWS};
use crate::resources::TilesTextureAtlas;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use fastrand::Rng;
use layer::WalkableLayer;
use mapgen::*;
use poi::{AreaStartingPosition, XStart, YStart};
use rooms::{NearestCorridors, SimpleRooms};

pub struct LevelPlugin;

#[derive(Resource)]
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub start_pos: UVec2,
    walkables: Vec<bool>,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MapLocation {
    pub row: u32,
    pub col: u32,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::new())
            .add_systems(OnEnter(GameState::InitLevel), spawn_tilemap)
            .add_systems(Update, finish_update.run_if(in_state(GameState::InitLevel)));
    }
}

impl TileMap {
    fn new() -> Self {
        let walkable_layer = Self::generate_map();
        let starting_point = AreaStartingPosition::find(XStart::LEFT, YStart::TOP, &walkable_layer);

        Self {
            width: walkable_layer.width,
            height: walkable_layer.height,
            walkables: walkable_layer.tiles,
            start_pos: UVec2::new(starting_point.x, starting_point.y),
        }
    }

    fn generate_map() -> WalkableLayer {
        let mut rng = Rng::with_seed(907647352);
        let sr = SimpleRooms::new(30, 5, 20);
        let corridors = NearestCorridors::new();
        let rooms = sr.generate(WORLD_COLS, WORLD_ROWS, &mut rng);
        let map = corridors.generate(&rooms);
        map.walkable_layer
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = (y * self.width + x) as usize;
            self.walkables[idx]
        }
    }
}

fn spawn_tilemap(mut commands: Commands, tilemap: Res<TileMap>, handle: Res<TilesTextureAtlas>) {
    for c in 0..tilemap.width {
        for r in 0..tilemap.height {
            let x: u32 = c;
            let y: u32 = WORLD_ROWS - r;
            let index: usize = if tilemap.is_walkable(c, r) {
                7 * TILES_COLS as usize
            } else {
                0
            };
            commands
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (x * TILE_SIZE) as f32,
                            ((y - 1) * TILE_SIZE) as f32,
                            0.0,
                        ),
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
                    MapLocation { row: y, col: x },
                ))
                .insert(Tile);
        }
    }
}

impl MapLocation {
    pub fn global_position(&self) -> Vec2 {
        Vec2::new(
            (self.col * TILE_SIZE) as f32,
            ((WORLD_ROWS - 1 - self.row) * TILE_SIZE) as f32,
        )
    }
}

fn finish_update(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}
