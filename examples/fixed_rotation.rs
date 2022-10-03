use bevy::prelude::*;
use bevy_fixed_sprites::*;

#[derive(Component)]
pub struct Point;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let d = 100.0 * Vec3::X;

    let point_id = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Point)
        .id();

    let sprite_id = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(64.0 * Vec2::ONE),
                ..Default::default()
            },
            transform: Transform::from_translation(-d),
            ..Default::default()
        })
        .id();

    let fixed_sprite_id = commands.spawn_bundle(FixedSpriteBundle {
            sprite: FixedSprite { 
                color: Color::GREEN,
                custom_size: Some(64.0 * Vec2::ONE),
                ..Default::default()
            },
            transform: Transform::from_translation(d),
            ..Default::default()
        })
        .id();

    commands.entity(point_id)
        .add_child(sprite_id)
        .add_child(fixed_sprite_id);
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Point>>
) {
    query.for_each_mut(|mut transform| {
        transform.rotate_z(0.3 * time.delta_seconds());
    });

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedSpritesPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}