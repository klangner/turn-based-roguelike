use bevy::math::vec3;
use bevy::{prelude::*, sprite::Anchor};

use crate::health::Health;
use crate::level::{MapLocation, TileMap};
use crate::player::Player;
use crate::resources::MonstersTextureAtlas;
use crate::state::GameState;

pub struct MonsterPlugin;

#[derive(Component)]
pub struct Monster;

impl Monster {
}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitLevel), setup_monster)
            .add_systems(Update, make_move.run_if(in_state(GameState::MonsterTurn)))
            .add_systems(OnEnter(GameState::MonsterTurn), despawn_dead_enemies);
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
        commands.spawn((
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
            Health::new(1),
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
        UVec2::new(player_pos.x - 3, player_pos.y - 3),
        UVec2::new(player_pos.x + 3, player_pos.y + 3),
    ];

    points
        .iter()
        .filter(|&p| tilemap.is_walkable(p.x, p.y))
        .copied()
        .collect()
}

fn make_move(
    mut monsters_query: Query<(&mut MapLocation, &mut Transform), With<Monster>>,
    player_query: Query<&mut MapLocation, (With<Player>, Without<Monster>)>,
    tilemap: Res<TileMap>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_location = player_query.single();
    let mut taken_locations: Vec<MapLocation> =
        monsters_query.iter().map(|(loc, _)| loc.clone()).collect();

    taken_locations.push(player_location.clone());

    for (mut monster_location, mut transform) in monsters_query.iter_mut() {
        let new_location = match fastrand::choice(0..5).unwrap() {
            1 => MapLocation {
                col: monster_location.col + 1,
                row: monster_location.row,
            },
            2 => MapLocation {
                col: monster_location.col,
                row: monster_location.row + 1,
            },
            3 => MapLocation {
                col: monster_location.col - 1,
                row: monster_location.row,
            },
            4 => MapLocation {
                col: monster_location.col,
                row: monster_location.row - 1,
            },
            _ => MapLocation {
                col: monster_location.col,
                row: monster_location.row,
            },
        };

        if new_location != *monster_location
            && tilemap.is_walkable(new_location.col, new_location.row)
            && taken_locations.iter().all(|l| &new_location != l)
        {
            monster_location.col = new_location.col;
            monster_location.row = new_location.row;
            let global_pos = monster_location.global_position();
            transform.translation = vec3(global_pos.x, global_pos.y, 1.0);
        }
    }

    next_state.set(GameState::PlayerTurn);
}

fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Health, Entity), With<Monster>>,
) {
    for (enemy, entity) in enemy_query.iter() {
        if enemy.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}
