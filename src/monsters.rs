use bevy::{prelude::*, sprite::Anchor};

use crate::level::{MapLocation, TileMap};
use crate::resources::MonstersTextureAtlas;
use crate::state::GameState;

pub struct MonsterPlugin;

#[derive(Component)]
pub struct Monster;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_monster)
            .add_systems(
                Update,
                make_move.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_monster(mut commands: Commands, handle: Res<MonstersTextureAtlas>, tilemap: Res<TileMap>) {
    let spawn_pos = spawn_location(10, &tilemap, &tilemap.start_pos);

    for sp in spawn_pos {
        let map_location = MapLocation {
            col: sp.x,
            row: sp.y,
        };
        let global_pos = map_location.global_position();
        commands
            .spawn((
                map_location,
                SpriteBundle {
                    transform: Transform::from_xyz(global_pos.x, global_pos.y, 0.0),
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
            .insert(Monster);
    }
}

fn spawn_location(_num: u32, tilemap: &Res<TileMap>, player_pos: &UVec2) -> Vec<UVec2> {
    // Spawn some near player
    let points = [
       UVec2::new(player_pos.x - 3, player_pos.y), 
       UVec2::new(player_pos.x + 3, player_pos.y), 
       UVec2::new(player_pos.x, player_pos.y - 3), 
       UVec2::new(player_pos.x, player_pos.y + 3), 
       UVec2::new(player_pos.x-3, player_pos.y - 3), 
       UVec2::new(player_pos.x+3, player_pos.y + 3), 
    ];

    points.iter()
        .filter(|&p| tilemap.is_walkable(p.x, p.y))
        .copied()
        .collect()
}

fn make_move(
    mut _monster_query: Query<(&mut MapLocation, &mut Transform), With<Monster>>,
    _tilemap: Res<TileMap>,
) {
    // if _monster_query.is_empty() {
    //     return;
    // }
}
