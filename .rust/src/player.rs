use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, Input, InputEvent, Marker2D, Window},
    global::{absf, atan2, clampf, signf, wrapf},
    obj::BaseRef,
    prelude::*,
};
use std::{f32::consts::TAU, f64::consts::PI};

use crate::{bullet::Bullet, ui::UI};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f32,
    speed_acceleration: f32,
    speed_rotation: f32,

    health: i32,

    root: Option<Gd<Window>>,
    ui_canvas: Option<Gd<UI>>,

    bullet: Gd<PackedScene>,
    bullet_spawn: Option<Gd<Marker2D>>,

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
            speed_rotation: TAU * 2.0,

            health: 3,

            root: None,
            ui_canvas: None,

            bullet: load::<PackedScene>("res://instances/bullet.tscn"),
            bullet_spawn: None,

            base,
        }
    }

    fn ready(&mut self) {
        let mut tree: Gd<SceneTree> = self.base().get_tree().unwrap();
        self.root = tree.get_root();

        self.ui_canvas = Some(tree.get_first_node_in_group("ui").unwrap().cast::<UI>());

        self.bullet_spawn = Some(self.base().get_node_as::<Marker2D>("BulletSpawn"));
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let player: BaseRef<'_, Player> = self.base();

        if event.is_action_pressed("ui_accept") {
            // Fix: use self.bullet instead of undefined scene variable
            let instance: Gd<Node> = self.bullet.instantiate().unwrap();

            // Assuming the instantiated scene root is a Node2D
            let mut bullet: Gd<Bullet> = instance.try_cast::<Bullet>().unwrap();

            let player_rotation: f32 = player.get_rotation();

            bullet.set_global_position(self.bullet_spawn.clone().unwrap().get_global_position());
            bullet.set_rotation_degrees(player.get_rotation_degrees() + 90.0);
            bullet.set_velocity(Vector2::from_angle(player_rotation).normalized_or_zero());

            self.root.as_mut().unwrap().add_child(&bullet);
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
            clampf(self.speed_rotation as f64 * delta, 0.0, absf(theta)) as f32
                * signf(theta) as f32,
        );

        // move player with controls using move_and_collide
        let new_velocity = Vector2::lerp(
            current_velocity,
            velocity * self.speed,
            self.speed_acceleration * delta as f32,
        );
        player.set_velocity(new_velocity);

        // Perform collision detection
        if let Some(collision) = player.move_and_collide(new_velocity * delta as f32) {
            self.take_damage(1);

            collision.get_collider().unwrap().free();
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    pub fn get_health(&mut self) -> i32 {
        self.health
    }

    #[func]
    fn take_damage(&mut self, amount: i32) {
        self.ui_canvas
            .as_mut()
            .unwrap()
            .bind_mut()
            .change_health(amount);

        self.health -= amount
    }
}
