use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const DEFAULT_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(280.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub fn get_text_style(
    asset_server: &Res<AssetServer>,
    font_size: f32,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/CyrillicPixel.ttf"),
        font_size: font_size,
        color: Color::WHITE,
    }
}