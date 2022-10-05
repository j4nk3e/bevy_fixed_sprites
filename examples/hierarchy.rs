use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_fixed_sprites::*;

const S: f32 = 32.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { 
    commands.spawn_bundle(Camera2dBundle::default());
    let fixed_sprite_id = commands.spawn_bundle(FixedSpriteBundle {
       sprite: FixedSprite { 
        color: Color::RED,
        custom_size: Some(S * Vec2::ONE),
        ..Default::default()
    },
        texture: asset_server.load("sprite.png"),
        transform: Transform::from_rotation(Quat::from_rotation_z(3f32.recip() * PI)),
        ..Default::default()
    })
    .id();

    let sprite_id = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite { 
                color: Color::WHITE,
                custom_size: Some(S * Vec2::ONE),
                ..Default::default()
            },
            texture: asset_server.load("sprite.png"),
            transform: Transform::from_translation(2. * S * Vec3::X),
            ..Default::default()
        })
        .id();

    let fixed_sprite_id_2 = commands.spawn_bundle(FixedSpriteBundle {
        sprite: FixedSprite { 
            color: Color::RED,
            custom_size: Some(S * Vec2::ONE),
            ..Default::default()
        },
        texture: asset_server.load("sprite.png"),
        transform: Transform::from_translation(2. * S * Vec3::X),
        ..Default::default()
    })
    .id();
    let sprite_id_2 = commands
    .spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(S * Vec2::ONE),
            ..Default::default()
        },
        texture: asset_server.load("sprite.png"),
        transform: Transform::from_translation(2. * S * Vec3::X),
        ..Default::default()
    })
    .id();
   
    commands.entity(fixed_sprite_id).add_child(sprite_id);
    commands.entity(sprite_id).add_child(fixed_sprite_id_2);
    commands.entity(fixed_sprite_id_2).add_child(sprite_id_2);
}

fn update(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    sprite_query: Query<Entity, With<Sprite>>,
    fixed_sprite_query: Query<Entity, With<FixedSprite>>,
) {
    if input.just_pressed(KeyCode::Space) {
        sprite_query.for_each(|entity| {
            commands.entity(entity)
            .remove::<Sprite>()
            .insert(FixedSprite { 
                color: Color::RED,
                custom_size: Some(S * Vec2::ONE),
                ..Default::default()
            });  
        });

        fixed_sprite_query.for_each(|entity| {
            commands.entity(entity)
            .remove::<FixedSprite>()
            .insert(Sprite { 
                color: Color::WHITE,
                custom_size: Some(S * Vec2::ONE),
                ..Default::default()
            });  
        });
    }

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