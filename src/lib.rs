#![allow(clippy::type_complexity)]

mod camera;
pub mod configs;
mod level;
mod player;
mod resources;
mod state;

use crate::resources::ResourcesPlugin;

use bevy::prelude::*;
use camera::FollowCameraPlugin;
use player::PlayerPlugin;
use state::GameState;
use level::LevelPlugin;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ResourcesPlugin,
            PlayerPlugin,
            FollowCameraPlugin,
            LevelPlugin,
        ));
    }
}