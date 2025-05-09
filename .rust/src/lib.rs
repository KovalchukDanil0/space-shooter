use godot::prelude::*;

mod bullet;
mod meteor;
mod meteor_spawner;
mod player;
mod ui;

struct SpaceShooter;

#[gdextension]
unsafe impl ExtensionLibrary for SpaceShooter {}
