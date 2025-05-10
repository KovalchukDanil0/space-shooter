use godot::{
    classes::{
        CharacterBody2D, ICharacterBody2D, Input, Marker2D, RandomNumberGenerator, Timer, Window,
    },
    global::{absf, atan2, clampf, signf, wrapf},
    obj::BaseRef,
    prelude::*,
};
use std::{f32, f64};

use crate::{bullet::Bullet, ui::UI};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f32,
    speed_acceleration: f32,
    speed_rotation: f32,

    health: i32,

    root: OnReady<Gd<Window>>,
    ui_canvas: OnReady<Gd<UI>>,

    bullet_instance: OnReady<Gd<PackedScene>>,
    bullet_spawn: OnReady<Gd<Marker2D>>,

    spread: f32,

    fire_delay: f64,
    fire_delay_timer: OnReady<Gd<Timer>>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Player {
            speed: 400.0,
            speed_acceleration: 2.0,
            speed_rotation: f32::consts::TAU * 2.0,

            health: 3,

            root: OnReady::from_base_fn(|base: &Gd<Node>| {
                base.get_tree().unwrap().get_root().unwrap()
            }),
            ui_canvas: OnReady::from_base_fn(|base: &Gd<Node>| {
                base.get_tree()
                    .unwrap()
                    .get_first_node_in_group("ui")
                    .unwrap()
                    .cast::<UI>()
            }),

            bullet_instance: OnReady::new(|| load("res://instances/bullet.tscn")),
            bullet_spawn: OnReady::node("BulletSpawnPoint"),

            spread: 0.1,

            fire_delay: 0.2,
            fire_delay_timer: OnReady::node("FireDelayTimer"),

            base,
        }
    }

    fn ready(&mut self) {
        // setting up timer
        self.fire_delay_timer.set_wait_time(self.fire_delay);
    }

    fn process(&mut self, delta: f64) {
        let input: Gd<Input> = Input::singleton();

        // shoot with timeout
        if input.is_action_pressed("ui_accept") && self.fire_delay_timer.is_stopped() {
            self.fire_delay_timer.start();

            self.shoot();
        }

        let mut player: Gd<CharacterBody2D> = self.base_mut().clone();

        let velocity: Vector2 = self.get_input(&input);
        let current_velocity: Vector2 = player.get_velocity();
        let rotation: f64 = player.get_rotation() as f64;

        // rotation to player movement direction logic
        let theta: f64 = wrapf(
            atan2(current_velocity.y as f64, current_velocity.x as f64) - rotation,
            -f64::consts::PI,
            f64::consts::PI,
        );
        player.rotate(
            clampf(self.speed_rotation as f64 * delta, 0.0, absf(theta)) as f32
                * signf(theta) as f32,
        );

        // move player and check collision
        let new_velocity = Vector2::lerp(
            current_velocity,
            velocity * self.speed,
            self.speed_acceleration * delta as f32,
        );
        player.set_velocity(new_velocity);

        if let Some(collision) = player.move_and_collide(new_velocity * delta as f32) {
            self.take_damage(1);

            collision.get_collider().unwrap().free();
        }
    }
}

#[godot_api]
impl Player {
    pub fn get_health(&mut self) -> i32 {
        self.health
    }

    /// Player take damage based on given amount
    fn take_damage(&mut self, amount: i32) {
        self.ui_canvas.bind_mut().change_health(amount);

        self.health -= amount;

        if self.health <= 0 {
            self.base_mut().queue_free();
        }
    }

    /// Get vector of movement keys
    fn get_input(&mut self, input: &Gd<Input>) -> Vector2 {
        input.get_vector("move_left", "move_right", "move_up", "move_down")
    }

    /// Shoot bullet_instance, apply spreading
    fn shoot(&mut self) {
        let instance: Gd<Node> = self.bullet_instance.instantiate().unwrap();

        let base: BaseRef<'_, Player> = self.base();
        let player_rotation: f32 = base.get_rotation();

        let mut bullet: Gd<Bullet> = instance.try_cast::<Bullet>().unwrap();

        // setting global position of bullet spawn point
        bullet.set_global_position(self.bullet_spawn.get_global_position());

        // set random spreading
        let mut rng: Gd<RandomNumberGenerator> = RandomNumberGenerator::new_gd();
        let spread: f32 = rng.randf_range(-self.spread, self.spread);

        bullet.set_rotation(base.get_rotation() + f32::consts::PI / 2.0 + spread);
        bullet.set_velocity(
            Vector2::from_angle(player_rotation)
                .rotated(spread)
                .normalized_or_zero(),
        );

        self.root.add_child(&bullet);
    }
}
