use godot::{
    classes::{IMarker2D, Marker2D, RandomNumberGenerator, Timer, Viewport},
    obj::BaseRef,
    prelude::*,
};
use std::f32::consts::FRAC_PI_2;

use crate::{meteor::Meteor, player::Player};

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct MeteorSpawner {
    spawn_timer: OnReady<Gd<Timer>>,
    meteor_instance: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<Player>>,

    base: Base<Marker2D>,
}

#[godot_api]
impl IMarker2D for MeteorSpawner {
    fn init(base: Base<Marker2D>) -> Self {
        MeteorSpawner {
            spawn_timer: OnReady::node("SpawnTimer"),
            meteor_instance: OnReady::new(|| load("res://instances/meteor.tscn")),
            player: OnReady::from_base_fn(|base| {
                base.get_tree()
                    .unwrap()
                    .get_first_node_in_group("player")
                    .unwrap()
                    .cast::<Player>()
            }),

            base,
        }
    }

    fn ready(&mut self) {
        self.init_timer();
    }
}

impl MeteorSpawner {
    fn init_timer(&mut self) {
        let base: BaseRef<'_, MeteorSpawner> = self.base();

        let meteor_instance: Gd<PackedScene> = self.meteor_instance.clone();

        let on_timer_timeout = {
            let mut base: Gd<Marker2D> = base.clone();
            let mut spawn_timer: Gd<Timer> = self.spawn_timer.clone();
            let player: Gd<Player> = self.player.clone();

            let viewport: Gd<Viewport> = base.get_viewport().unwrap();
            let camera: Gd<Camera2D> = viewport.get_camera_2d().unwrap();

            let meteor_instance: Gd<PackedScene> = meteor_instance.clone();

            move |_: &[&Variant]| -> Result<Variant, ()> {
                let mut rng: Gd<RandomNumberGenerator> = RandomNumberGenerator::new_gd();

                // sets random time to the next timer iteration
                let random_wait_time: f64 = rng.randf_range(1.0, 3.0) as f64;
                spawn_timer.set_wait_time(random_wait_time);

                // get random position based on global viewport coordinates
                let visible_viewport: Vector2 = viewport.get_visible_rect().size;
                let random_pos: Vector2 = Vector2 {
                    x: rng.randf_range(0.0, visible_viewport.x) + camera.get_global_position().x,
                    y: rng.randf_range(0.0, visible_viewport.y) + camera.get_global_position().y,
                };

                // instantiate meteor with random position on viewport
                let mut meteor: Gd<Meteor> = meteor_instance.instantiate_as::<Meteor>();
                meteor.set_position(random_pos);

                // sets direction of meteor movement to player with spread of ~PI/2
                let angle_to_player: f32 = meteor.get_angle_to(player.get_global_position());
                let spread: f32 = rng.randf_range(-FRAC_PI_2, FRAC_PI_2);
                meteor.bind_mut().velocity = Vector2::RIGHT.rotated(angle_to_player + spread);

                base.add_child(&meteor);

                Ok(Variant::nil())
            }
        };

        let callable: Callable = Callable::from_local_fn("on_timer_timeout", on_timer_timeout);
        self.spawn_timer.connect("timeout", &callable);

        self.spawn_timer.start();
    }
}
