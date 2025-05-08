use godot::prelude::*;

mod bullet;
mod player;
mod ui;

struct SpaceShooter;

#[gdextension]
unsafe impl ExtensionLibrary for SpaceShooter {}
