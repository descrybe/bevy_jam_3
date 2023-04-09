use bevy::prelude::Plugin;

use self::resources::AssetsCache;

pub mod resources;

pub struct AssetsCachePlugin;

impl Plugin for AssetsCachePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<AssetsCache>();
    }
}
