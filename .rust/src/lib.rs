use godot::prelude::*;

mod bullet;
mod player;

struct SpaceShooter;

#[gdextension]
unsafe impl ExtensionLibrary for SpaceShooter {}
