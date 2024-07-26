use bevy::prelude::*;

use crate::configs::*;
use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct TilesTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct RoguesTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct MonstersTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilesTextureAtlas::default())
            .insert_resource(RoguesTextureAtlas::default())
            .insert_resource(MonstersTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_rogues)
            .add_systems(OnEnter(GameState::Loading), load_monsters)
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

fn load_monsters(
    mut handle: ResMut<MonstersTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    handle.image = Some(asset_server.load(MONSTERS_IMAGE_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE, TILE_SIZE),
        MONSTERS_COLS,
        MONSTERS_ROWS,
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
