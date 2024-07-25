use bevy::prelude::*;

use crate::configs::*;
use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct RoguesTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilesTextureAtlas::default())
            .insert_resource(RoguesTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_rogues)
            .add_systems(OnEnter(GameState::Loading), load_tiles);
    }
}

fn load_rogues(
    mut handle: ResMut<RoguesTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    handle.image = Some(asset_server.load(ROGUES_IMAGE_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE, TILE_SIZE),
        ROGUES_COLS,
        ROGUES_ROWS,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));
}

fn load_tiles(
    mut handle: ResMut<TilesTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(TILE_IMAGE_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE, TILE_SIZE),
        TILES_COLS,
        TILES_ROWS,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GenerateLevel);
}

impl Default for TilesTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

impl Default for RoguesTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}
