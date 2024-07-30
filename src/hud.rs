use bevy::prelude::*;

use crate::configs::WINDOW_WIDTH;
use crate::health::Health;
use crate::player::Player;
use crate::state::GameState;

pub struct HudPlugin;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthText;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitLevel), spawn_hud)
            .add_systems(OnEnter(GameState::PlayerTurn), update_hud);
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
