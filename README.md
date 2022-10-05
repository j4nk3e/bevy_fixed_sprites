# bevy_fixed_sprites

[![crates.io](https://img.shields.io/crates/v/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_fixed_sprites)
[![crates.io](https://img.shields.io/crates/d/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)

Bevy sprites that ignore rotation and scaling.

![image](/assets/fixed_sprite.png)

All these sprite's transforms have the same scale and rotation.

Supports Bevy 0.8

#
### Cargo 
```toml
[dependency]
bevy_fixed_sprites = 0.1
```

### Plugin
You need to add the `FixedSpritePlugin` to your Bevy App before you can draw a
`FixedSprite`

```rust
use bevy_fixed_sprites::*;
app.add_plugin(FixedSpritesPlugin);
```

### Usage

The interface is identical to normal Bevy sprites, just change the names:

* `bevy::sprite::Sprite` -> `bevy_fixed_sprites::FixedSprite`
* `bevy::sprite::SpriteBundle` -> `bevy_fixed_sprites::FixedSpriteBundle`
* `bevy::sprite::TextureAtlasSprite` -> `bevy_fixed_sprites::FixedTextureAtlasSprite`
* `bevy::sprite::SpriteSheetBundle` -> `bevy_fixed_sprites::FixedSpriteSheetBundle`

Although rotation and scale aren't applied to FixedSprites, the `flip_*` fields and the `custom_size` fields still work as normal.

FixedSprites respect the Bevy transform hierarchy, entities with a `FixedSprite` component can have children that rotate and scale relative to their parent's `Transform`.

### Examples

```
cargo run --example fixed_sprite
cargo run --example hierarchy
```