#![allow(clippy::type_complexity)]

pub mod configs;
mod state;
mod resources;
mod tile_map;

use crate::resources::ResourcesPlugin;

use bevy::prelude::*;
use state::GameState;
use tile_map::TilemapPlugin;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ResourcesPlugin,
            TilemapPlugin,
        ));
    }
}