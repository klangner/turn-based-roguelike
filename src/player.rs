use bevy::math::vec3;
use bevy::{prelude::*, sprite::Anchor};

use crate::actions::Actions;
use crate::health::Health;
use crate::level::{MapLocation, TileMap};
use crate::monsters::Monster;
use crate::resources::RoguesTextureAtlas;
use crate::state::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitLevel), setup_player)
            .add_systems(Update, player_turn.run_if(in_state(GameState::PlayerTurn)));
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
            Health::new(10),
        ))
        .insert(Player);
}

fn player_turn(
    mut player_query: Query<(&mut MapLocation, &mut Transform), With<Player>>,
    mut monster_query: Query<(&MapLocation, &mut Health), (With<Monster>, Without<Player>)>,
    tilemap: Res<TileMap>,
    actions: Res<Actions>,
    mut next_state: ResMut<NextState<GameState>>,
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
            if let Some((_, mut monster)) = monster_query
                .iter_mut()
                .find(|(loc, _)| *loc == &new_location)
            {
                monster.damage(1);
            } else {
                map_location.col = new_location.col;
                map_location.row = new_location.row;
                let global_pos = map_location.global_position();
                transform.translation = vec3(global_pos.x, global_pos.y, 1.0);
            }
            next_state.set(GameState::MonsterTurn);
        }
    }
}
