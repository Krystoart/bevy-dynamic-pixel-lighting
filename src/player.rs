use bevy::prelude::*;

use crate::components::{PlayerTimer, Velocity};
use crate::{GameTextures, SPRITE_SCALE, TIME_STEP};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_startup_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_startup_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.player.clone(),
            transform: Transform {
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerTimer::default())
        .insert(Velocity { x: 0., y: 0. });
}

fn player_keyboard_event_system(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let direction = {
            let right = (kb.pressed(KeyCode::D)).to_dir();
            let left = -(kb.pressed(KeyCode::A)).to_dir();
            let up = (kb.pressed(KeyCode::W)).to_dir();
            let down = -(kb.pressed(KeyCode::S)).to_dir();
            Vec2::new(left + right, up + down)
        };

        velocity.x = direction.x;
        velocity.y = direction.y;
    }
}

trait ToDirection {
    fn to_dir(self) -> f32;
}

impl ToDirection for bool {
    fn to_dir(self) -> f32 {
        if self {
            1_f32
        } else {
            0_f32
        }
    }
}
