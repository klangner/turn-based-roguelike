use bevy::prelude::*;

use crate::configs::*;
use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct TilesTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilesTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}


fn load_assets(
    mut handle: ResMut<TilesTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(TILE_IMAGE_PATH));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE, TILE_SIZE),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::Playing);
}

impl Default for TilesTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}
