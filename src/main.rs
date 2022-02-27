use bevy::prelude::*;

const BASE_WIDTH: f32 = 640_f32;
const BASE_HEIGHT: f32 = 360_f32;

// const CHAR_RUN_FORWARD_SHEET: &str = include_str!("../assets/run_sheet.png");

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Pixel art dynamic lighting"),
            vsync: true,
            width: BASE_WIDTH,
            height: BASE_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite_system)
        .run();

    // canvas size: 640x360
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("../assets/run_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 128.0), 4, 1);
    let textures_handle = textures.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures_handle,
            // transform: Transform::from_scale(Vec3::splat(0.6)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
}
