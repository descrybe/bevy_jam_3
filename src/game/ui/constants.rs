use bevy::{
    prelude::{Color, Style, Val},
    ui::{AlignSelf, JustifyContent, PositionType, UiRect, Size},
};

pub const EXPERIENCE_BAR_WIDTH: Val = Val::Px(550.0);
pub const EXPREIENCE_BAR_MAIN_COLOR: Color = Color::rgb(0.85, 0.0, 0.85);
pub const EXPREIENCE_BAR_BACKGROUND_COLOR: Color = Color::rgb(0.25, 0.8, 1.0);
pub const EXPERIENCE_BAR_BORDER_COLOR: Color = Color::rgb(0.65, 0.65, 0.65);
const EXPERIENCE_BAR_HEIGHT: Val = Val::Px(35.0);
pub const PLAYER_HEALTH_BAR_WIDTH:Val = Val::Px(100.0);
pub const PLAYER_HEALTH_BAR_HEIGHT:Val = Val::Px(10.0);

pub const EXPERIENCE_BAR_WRAPPER_STYLES: Style = Style {
    // size: Size {
    //     width: Val::Percent(100.0),
    //     height: EXPERIENCE_BAR_HEIGHT,
    // },
    size: Size {
        width: Val::Px(100.0),
        height: Val::Px(100.0),
    },
    position_type: PositionType::Absolute,
    position: UiRect {
        top: Val::Px(20.0),
        left: Val::Auto,
        right: Val::Auto,
        bottom: Val::Auto,
    },
    align_self: AlignSelf::FlexEnd,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};
