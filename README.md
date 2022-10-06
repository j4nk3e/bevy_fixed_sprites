# bevy_fixed_sprites

[![crates.io](https://img.shields.io/crates/v/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_fixed_sprites)
[![crates.io](https://img.shields.io/crates/d/bevy_fixed_sprites)](https://crates.io/crates/bevy_fixed_sprites)

Bevy sprites that have their own transform independent of the bevy transform hierarchy.

![image](/assets/example.png)

Each sprite's `Transform` has the same scale and rotation.

Supports Bevy 0.8

#
### Cargo 
```toml
[dependency]
bevy_fixed_sprites = 0.2
```

### Plugin
You need to add the `FixedSpritePlugin` to your Bevy App before you can draw a
`FixedSprite`

```rust
use bevy_fixed_sprites::*;
app.add_plugin(FixedSpritesPlugin);
```

### Usage

For bevy_fixed_sprites' equivalents to regular bevy sprites use:

* `bevy::sprite::Sprite` -> `bevy_fixed_sprites::FixedSprite`
* `bevy::sprite::SpriteBundle` -> `bevy_fixed_sprites::FixedSpriteBundle`
* `bevy::sprite::TextureAtlasSprite` -> `bevy_fixed_sprites::FixedTextureAtlasSprite`
* `bevy::sprite::SpriteSheetBundle` -> `bevy_fixed_sprites::FixedSpriteSheetBundle`

### Examples

```
cargo run --example fixed_sprite
cargo run --example hierarchy
```