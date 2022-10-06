use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::Anchor;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::*;

/// rotation of the sprite in radians
#[derive(Component, Debug, Default, Clone, Reflect, Deref, DerefMut)]
pub struct FixedTransform(pub Transform);

/// rotation matrix for the sprite, computed internally by the plugin's systems
#[derive(Component, Debug, Default, Clone, Reflect, Deref, DerefMut)]
pub struct FixedGlobalTransform(pub GlobalTransform);

#[derive(Bundle, Clone, Default)]
pub struct SpriteRotationBundle {
    transform: FixedTransform,
    global: FixedGlobalTransform,
}

impl From<Transform> for FixedTransform {
    #[inline]
    fn from(transform: Transform) -> Self {
        Self(transform)
    }
}

impl From<FixedTransform> for Transform {
    #[inline]
    fn from(fixed_transform: FixedTransform) -> Self {
        fixed_transform.0
    }
}

/// A sprite that doesn't rotate or scale 
#[derive(Component, Debug, Default, Clone, Reflect)]
#[repr(C)]
pub struct FixedSprite {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
    pub anchor: Anchor,
}

/// A sprite from a texture atlas that doesn't rotate or scale 
#[derive(Component, Debug, Clone, Reflect)]
pub struct FixedTextureAtlasSprite {
    pub color: Color,
    pub index: usize,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
    pub anchor: Anchor,
}

impl Default for FixedTextureAtlasSprite {
    fn default() -> Self {
        Self {
            index: 0,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            custom_size: None,
            anchor: Anchor::default(),
        }
    }
}

impl FixedTextureAtlasSprite {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }
}

#[derive(Bundle, Clone)]
pub struct FixedSpriteBundle {
    pub sprite: FixedSprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub rotation: FixedTransform,
    pub rotation_matrix: FixedGlobalTransform,
}

impl Default for FixedSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            rotation: Default::default(),
            rotation_matrix: Default::default(),
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct FixedSpriteSheetBundle {
    pub sprite: FixedTextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub rotation: FixedTransform,
    pub rotation_matrix: FixedGlobalTransform,
}

impl From<Sprite> for FixedSprite {
    fn from(sprite: Sprite) -> Self {
        Self {
            color: sprite.color,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            custom_size: sprite.custom_size,
            anchor: sprite.anchor,
        }
    }
}

impl From<FixedSprite> for Sprite {
    fn from(sprite: FixedSprite) -> Self {
        Self {
            color: sprite.color,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            custom_size: sprite.custom_size,
            anchor: sprite.anchor,
        }
    }
}

impl From<TextureAtlasSprite> for FixedTextureAtlasSprite {
    fn from(sprite: TextureAtlasSprite) -> Self {
        Self {
            color: sprite.color,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            custom_size: sprite.custom_size,
            anchor: sprite.anchor,
            index: sprite.index,
        }
    }
}

impl From<FixedTextureAtlasSprite> for TextureAtlasSprite {
    fn from(sprite: FixedTextureAtlasSprite) -> Self {
        Self {
            color: sprite.color,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            custom_size: sprite.custom_size,
            anchor: sprite.anchor,
            index: sprite.index,
        }
    }
}

pub fn update_fixed_transforms(
    mut query: Query<(&FixedTransform, &mut FixedGlobalTransform), Changed<FixedTransform>>, 
) {
    query.for_each_mut(|(ftf, mut fgtf)| {
        fgtf.0 = ftf.0.into();
    });
}

pub fn extract_fixed_sprites(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    sprite_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &FixedSprite,
            &GlobalTransform,
            &FixedGlobalTransform,
            &Handle<Image>,
        )>,
    >,
    atlas_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &FixedTextureAtlasSprite,
            &GlobalTransform,
            &FixedGlobalTransform,
            &Handle<TextureAtlas>,
        )>,
    >,
) {
    for (entity, visibility, sprite, global_transform, fixed_transform, handle) in sprite_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        let transform = Affine3A {
            matrix3: fixed_transform.affine().matrix3,
            translation: fixed_transform.translation_vec3a() + global_transform.translation_vec3a(),
        }.into();
        extracted_sprites.sprites.alloc().init(ExtractedSprite {
            entity,
            color: sprite.color,
            transform,
            rect: None,
            custom_size: sprite.custom_size,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            image_handle_id: handle.id,
            anchor: sprite.anchor.as_vec(),
        });
    }
    for (entity, visibility, atlas_sprite, global_transform, fixed_transform, texture_atlas_handle) in atlas_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
            let rect = Some(texture_atlas.textures[atlas_sprite.index as usize]);
            let transform = Affine3A {
                matrix3: fixed_transform.affine().matrix3,
                translation: fixed_transform.translation_vec3a() + global_transform.translation_vec3a(),
            }.into();
            
            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                entity,
                color: atlas_sprite.color,
                transform,
                rect,
                custom_size: atlas_sprite.custom_size,
                flip_x: atlas_sprite.flip_x,
                flip_y: atlas_sprite.flip_y,
                image_handle_id: texture_atlas.texture.id,
                anchor: atlas_sprite.anchor.as_vec(),
            });
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum FixedSpriteSystem {
    UpdateFixedTransforms
}

pub struct FixedSpritesPlugin;

impl Plugin for FixedSpritesPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<FixedSprite>()
        .register_type::<FixedTextureAtlasSprite>()
        .register_type::<FixedTransform>()
        .register_type::<FixedGlobalTransform>()
        .add_system_to_stage(CoreStage::PostUpdate, update_fixed_transforms.label(FixedSpriteSystem::UpdateFixedTransforms))
        ;
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
            .add_system_to_stage(
                RenderStage::Extract,
                extract_fixed_sprites
                    .after(SpriteSystem::ExtractSprites),
            );
        }
    }
}