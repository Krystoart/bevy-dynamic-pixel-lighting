#![allow(unused)] // temp

use bevy::prelude::*;

use components::{PlayerTimer, Velocity};

use player::PlayerPlugin;

mod components;
mod player;

const BASE_SCREEN_SIZE: (f32, f32) = (640., 360.);
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);
const TIME_STEP: f32 = 1. / 60.;
const SPEED: f32 = 500.;

const PLAYER_SHEET: &str = "run_sheet.png";
const PLAYER_RUN_LEN: usize = 4;
const SPRITE_SCALE: f32 = 1.2;

struct GameTextures {
    player: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Pixel art dynamic lighting"),
            // vsync: true,
            width: BASE_SCREEN_SIZE.0,
            height: BASE_SCREEN_SIZE.1,
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(animate_sprite_system)
        .add_system(movable_system)
        .run();

    // canvas size: 640x360
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player animation atlas
    let texture_handle = asset_server.load(PLAYER_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 128.0), 4, 1);
    let player = texture_atlases.add(texture_atlas);

    // Adding game textures as a resource
    let game_textures = GameTextures { player };
    commands.insert_resource(game_textures);
}

fn movable_system(mut query: Query<(&Velocity, &mut Transform)>) {
    if let Ok((velocity, mut tf)) = query.get_single_mut() {
        let translation = &mut tf.translation;
        translation.x += velocity.x * TIME_STEP * SPEED;
        translation.y += velocity.y * TIME_STEP * SPEED;
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut PlayerTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= PLAYER_RUN_LEN {
                sprite.index = 0;
            }
        }
    }
}
