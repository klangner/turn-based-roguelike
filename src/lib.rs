#![allow(clippy::type_complexity)]

mod actions;
mod camera;
pub mod configs;
pub mod health;
mod level;
mod monsters;
mod player;
mod resources;
mod state;

use crate::resources::ResourcesPlugin;

use actions::ActionsPlugin;
use bevy::prelude::*;
use camera::FollowCameraPlugin;
use level::LevelPlugin;
use monsters::MonsterPlugin;
use player::PlayerPlugin;
use state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ResourcesPlugin,
            LevelPlugin,
            ActionsPlugin,
            MonsterPlugin,
            PlayerPlugin,
            FollowCameraPlugin,
        ));
    }
}
