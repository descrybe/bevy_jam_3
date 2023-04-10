use bevy::{
    prelude::{AssetServer, Assets, FromWorld, Handle, Image, Resource, Vec2},
    sprite::TextureAtlas,
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
    pub lightning: Handle<Image>,
}
pub struct CharactersSprites {
    pub zombie: Handle<Image>,
    pub wizzard: Handle<Image>,
}
pub struct UISprites {
    pub dices: Dices,
}

pub struct Dices {
    pub bottle: Handle<Image>,
    pub heart: Handle<Image>,
    pub radiance: Handle<Image>,
    pub flame: Handle<Image>,
    pub shuriken: Handle<Image>,
    pub lightning: Handle<Image>,
}

impl FromWorld for AssetsCache {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let assets_loader = world.get_resource::<AssetServer>().unwrap();
        let texture_atlas = world.get_resource::<Assets<TextureAtlas>>();

        return AssetsCache {
            fonts: Fonts {
                cyrillic_pixel: assets_loader.load("fonts/CyrillicPixel.ttf"),
            },
            sprites: Sprites {
                characters: CharactersSprites {
                    zombie: assets_loader.load("sprites/zombie.png"),
                    wizzard: assets_loader.load("sprites/player.png"),
                },
                ui: UISprites {
                    dices: Dices {
                        bottle: assets_loader.load("sprites/dice/dice1.png"),
                        heart: assets_loader.load("sprites/dice/dice2.png"),
                        radiance: assets_loader.load("sprites/dice/dice3.png"),
                        flame: assets_loader.load("sprites/dice/dice4.png"),
                        shuriken: assets_loader.load("sprites/dice/dice5.png"),
                        lightning: assets_loader.load("sprites/dice/dice6.png"),
                    },
                },
                projectiles: ProjectileSprites {
                    bottle: assets_loader.load("sprites/projectile.png"),
                    shuriken: assets_loader.load("sprites/shuriken.png"),
                    lightning: assets_loader.load("sprites/lightning.png"),
                },
            },
        };
    }
}
