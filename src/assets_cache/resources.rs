use bevy::{
    prelude::{AssetServer, FromWorld, Handle, Image, Resource},
    text::Font,
};

#[derive(Resource)]
pub struct AssetsCache {
    pub sprites: Sprites,
    pub fonts: Fonts,
}

pub struct Fonts {
    pub cyrillic_pixel: Handle<Font>,
}

pub struct Sprites {
    pub characters: CharactersSprites,
    pub ui: UISprites,
    pub projectiles: ProjectileSprites,
}

pub struct ProjectileSprites {
    pub bottle: Handle<Image>,
    pub shuriken: Handle<Image>,
}
pub struct CharactersSprites {
    pub zombie: Handle<Image>,
    pub wizzard: Handle<Image>,
}
pub struct UISprites {}

impl FromWorld for AssetsCache {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let assets_loader = world.get_resource::<AssetServer>().unwrap();

        return AssetsCache {
            fonts: Fonts {
                cyrillic_pixel: assets_loader.load("fonts/CyrillicPixel.ttf"),
            },
            sprites: Sprites {
                characters: CharactersSprites {
                    zombie: assets_loader.load("sprites/zombie.png"),
                    wizzard: assets_loader.load("sprites/player.png"),
                },
                ui: UISprites {},
                projectiles: ProjectileSprites {
                    bottle: assets_loader.load("sprites/projectile.png"),
                    shuriken: assets_loader.load("sprites/shuriken.png"),
                },
            },
        };
    }
}
