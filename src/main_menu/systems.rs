use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::components::MainMenu;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn main_menu_interaction(
    mut commands: Commands,
    mut game_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    // main_menu_query: Query<Entity, With<MainMenu>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        // let main_menu_entity = main_menu_query.get_single().unwrap();
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = "Start game".to_string();
        *color = NORMAL_BUTTON.into();

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                game_state.set(AppState::Game);
                // commands.entity(main_menu_entity).despawn();
            }
            Interaction::Hovered => {}
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    main_menu_setup(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn main_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/Adumu.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    ));
                });
        }).id();

        main_menu_entity
}