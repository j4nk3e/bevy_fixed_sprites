# bevy_fixed_sprites

[![crates.io](https://img.shields.io/crates/v/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_fixed_sprites)
[![crates.io](https://img.shields.io/crates/d/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)

Bevy sprites that don't rotate or scale with their transform.

![image](/assets/fixed_sprite.PNG)

All these sprites have the same scale and rotation.

Supports Bevy 0.8

#
### Cargo
```toml
bevy_fixed_sprites = 0.1
```

### Plugin
You need to add the `BevyFixedSpritePlugin` to your Bevy App before you can draw a
`FixedSprite`

```rust
use bevy_fixed_sprites::*;
app.add_plugin(BevyFixedSpritesPlugin);
```

### Usage

The interface is identical to the normal Bevy sprites, just change the names:

* bevy::sprite::Sprite -> bevy_fixed_sprites::FixedSprite
* bevy::sprite::SpriteBundle -> bevy_fixed_sprites::FixedSpriteBundle
* bevy::sprite::TextureAtlasSprite -> bevy_fixed_sprites::FixedTextureAtlasSprite
* bevy::sprite::SpriteSheetBundle -> bevy_fixed_sprites::FixedSpriteSheetBundle

Although rotation and scale aren't applied to FixedSprites, the `flip_*` fields and the `custom_size` field still work as normal.

### Example

```
cargo run --example fixed_sprite
```