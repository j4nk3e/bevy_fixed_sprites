# bevy_fixed_sprites

Bevy sprites that don't rotate or scale with their transform.

Supports Bevy 0.8
#
### Cargo
```toml
bevy_fixed_sprites = 0.1
```

### Plugin
```rust
use bevy_fixed_sprites::*;
app.add_plugin(BevyFixedSpritesPlugin);
```

### Usage

Identical to ordinary Bevy sprites except for the renamings:

* Sprite -> FixedSprite
* SpriteBundle -> FixedSpriteBundle
* TextureAtlasSprite -> FixedTextureAtlasSprite
* SpriteSheetBundle -> FixedSpriteSheetBundle

### Example
```
cargo run --example rotating
```