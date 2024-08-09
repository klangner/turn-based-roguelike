use bevy::math::vec3;
use bevy::prelude::*;

use crate::camera::FollowCamera;
use crate::configs::{TILE_SIZE, WINDOW_WIDTH};
use crate::health::Health;
use crate::level::MapLocation;
use crate::monsters::Monster;
use crate::player::Player;
use crate::state::GameState;

pub struct HudPlugin;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
pub struct Tooltip;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitLevel), spawn_hud)
            .add_systems(OnEnter(GameState::PlayerTurn), update_hud)
            .add_systems(
                Update,
                update_tooltip.run_if(in_state(GameState::PlayerTurn)),
            );
    }
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                // display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(WINDOW_WIDTH),
                        height: Val::Px(32.0),
                        margin: UiRect::px(0.0, 0.0, 0., 0.),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::srgb(0.3, 0., 0.)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(3.0 * WINDOW_WIDTH / 4.0),
                                height: Val::Px(32.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.),
                                top: Val::Px(0.),
                                ..default()
                            },
                            background_color: BackgroundColor::from(Color::srgb(1.0, 0., 0.)),
                            ..default()
                        },
                        HealthBar,
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Health: 20 / 20",
                            TextStyle {
                                font: asset_server.load("monogram.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            align_self: AlignSelf::Center,
                            ..default()
                        })
                        .with_text_justify(JustifyText::Center),
                        HealthText,
                    ));
                });
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Tooltip",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    align_self: AlignSelf::Center,
                    ..default()
                }),
                Tooltip,
            ));
        });
}

fn update_hud(
    mut bar_query: Query<&mut Style, With<HealthBar>>,
    mut label_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<&Health, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let health = player_query.single();
    // Health bar length
    let mut bar_style = bar_query.get_single_mut().unwrap();
    bar_style.width = Val::Px(health.current_hp as f32 * WINDOW_WIDTH / health.max_hp as f32);
    // Health text
    let mut text = label_query.get_single_mut().unwrap();
    text.sections[0].value = format!("Health: {} / {}", health.current_hp, health.max_hp);
}

fn update_tooltip(
    mut tooltip_query: Query<(&mut Transform, &mut Style, &mut Text), (With<Text>, With<Tooltip>)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<FollowCamera>>,
    monster_query: Query<(&MapLocation, &Health), With<Monster>>,
) {
    if tooltip_query.is_empty() {
        return
    }

    let (mut tt_tx, mut tt_style, mut tt_text) = tooltip_query.single_mut();
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    tt_style.display = Display::None;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let tile_col = (world_position.x / TILE_SIZE as f32) as u32;
        let tile_row = (world_position.y / TILE_SIZE as f32) as u32;

        for (monster_loc, monster_health) in &monster_query {
            if monster_loc.col == tile_col && monster_loc.row == tile_row {
                tt_style.display = Display::Block;
                tt_text.sections = vec![TextSection::from(format!("Goblin, heatlh: {}", monster_health.current_hp))];
                tt_tx.translation = vec3(
                    (monster_loc.col * TILE_SIZE) as f32,
                    (monster_loc.row * TILE_SIZE) as f32,
                    10.);
            }
        }
    }
}
