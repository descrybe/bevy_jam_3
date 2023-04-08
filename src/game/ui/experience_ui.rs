use bevy::{prelude::*};
use super::constants::*;

pub fn spawn_exp_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: EXPERIENCE_BAR_WRAPPER_STYLES,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(EXPERIENCE_BAR_WIDTH),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: EXPERIENCE_BAR_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: EXPREIENCE_BAR_BACKGROUND_COLOR.into(),
                            ..default()
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Percent(75.0), // сюда передавать Experience.value/EXPERIENCE_TO_LVL_UP
                                    height: Val::Px(31.0),
                                },
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Auto,
                                    left: Val::Undefined,
                                    right: Val::Auto,
                                    bottom: Val::Undefined,
                                },
                                ..default()
                            },
                            background_color: EXPREIENCE_BAR_MAIN_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "LVL 1", // тут вставлять значение текущего уровня
                                    TextStyle {
                                        font: asset_server.load("fonts/CyrillicPixel.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Auto,
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        bottom: Val::Px(20.0),
                                    },
                                    ..default()
                                }),
                                Label,
                            ));
                        });
                });
        });
}
