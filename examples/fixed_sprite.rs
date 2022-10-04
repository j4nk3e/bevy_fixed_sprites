use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_fixed_sprites::*;

// Spawns seven pairs of white (bevy sprites) and red (fixed sprites),
// rotates them around a point
// and oscillates the scale of each the sprite's transform.

#[derive(Component)]
pub struct Center;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let s = 64f32;
    let d = 3. * s * Vec3::X;                     
    let n = 7;                      
    let center_id = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Center)
        .id();
    for i in 0..n {
        let angle = i as f32 * (n as f32).recip() * PI;
        let translation = Quat::from_rotation_z(angle).mul_vec3(d) * (1. - 2. * (i % 2) as f32);
        let sprite_id = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(s * Vec2::ONE),
                    ..Default::default()
                },
                texture: asset_server.load("sprite.png"),
                transform: Transform::from_translation(-translation  + (i as f32) * Vec3::Z),
                ..Default::default()
            })
            .id();
        let fixed_sprite_id = commands.spawn_bundle(FixedSpriteBundle {
                sprite: FixedSprite { 
                    color: Color::RED,
                    custom_size: Some(s * Vec2::ONE),
                    ..Default::default()
                },
                texture: asset_server.load("sprite.png"),
                transform: Transform::from_translation(translation  + (i as f32) * Vec3::Z),
                ..Default::default()
            })
            .id();
        commands.entity(center_id).push_children(&[sprite_id, fixed_sprite_id]);
    }
}

fn update(
    time: Res<Time>,
    mut point_query: Query<&mut Transform, With<Center>>,
    mut transform_query: Query<&mut Transform, (Without<Center>, Without<Camera>)>
) {
    point_query.for_each_mut(|mut transform| {
        transform.rotate_z(0.3 * time.delta_seconds());
    });
    transform_query.for_each_mut(|mut transform| {
        transform.scale = (1. + time.seconds_since_startup().sin() as f32) * Vec3::ONE;
    });
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedSpritesPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}