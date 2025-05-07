use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, Input, InputEvent},
    global::{absf, atan2, clampf, signf, wrapf},
    prelude::*,
};
use std::{f32::consts::TAU, f64::consts::PI};

use crate::bullet::Bullet;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f32,

    #[export]
    speed_acceleration: f32,

    #[export]
    rotation_speed: f32,

    bullet: Gd<PackedScene>,

    base: Base<CharacterBody2D>,
}

fn get_input() -> Vector2 {
    let input: Gd<Input> = Input::singleton();
    input.get_vector("move_left", "move_right", "move_up", "move_down")
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Player {
            speed: 400.0,
            speed_acceleration: 2.0,
            rotation_speed: TAU * 2.0,
            bullet: load::<PackedScene>("res://instances/bullet.tscn"),
            base,
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let player: Gd<CharacterBody2D> = self.base_mut().clone();

        if event.is_action_pressed("ui_accept") {
            // Fix: use self.bullet instead of undefined scene variable
            let instance: Gd<Node> = self.bullet.instantiate().unwrap();

            // Assuming the instantiated scene root is a Node2D
            let mut bullet: Gd<Bullet> = instance.try_cast::<Bullet>().unwrap();

            let player_rotation: f32 = player.get_rotation();

            bullet.set_global_position(player.get_global_position());
            bullet.set_rotation_degrees(player.get_rotation_degrees() + 90.0);
            bullet.set_velocity(Vector2::from_angle(player_rotation).normalized_or_zero());

            player
                .get_tree()
                .unwrap()
                .get_root()
                .unwrap()
                .add_child(&bullet);
        }
    }

    fn process(&mut self, delta: f64) {
        // character body properties
        let mut player: Gd<CharacterBody2D> = self.base_mut().clone();

        let current_velocity: Vector2 = player.get_velocity();
        let velocity: Vector2 = get_input();
        let rotation: f64 = player.get_rotation() as f64;

        // rotation to player movement direction logic
        let theta: f64 = wrapf(
            atan2(current_velocity.y as f64, current_velocity.x as f64) - rotation,
            -PI,
            PI,
        );
        player.rotate(
            clampf(self.get_rotation_speed() as f64 * delta, 0.0, absf(theta)) as f32
                * signf(theta) as f32,
        );

        // move player with controls using move_and_collide
        let new_velocity = Vector2::lerp(
            current_velocity,
            velocity * self.get_speed(),
            self.get_speed_acceleration() * delta as f32,
        );
        player.set_velocity(new_velocity);

        // Perform collision detection
        if let Some(collision) = player.move_and_collide(new_velocity * delta as f32) {
            // Handle collision logic here if needed
            godot_print!(
                "Collision detected with: {:?}",
                collision.get_collider().unwrap()
            );
        }
    }
}
