use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, SceneTreeTimer},
    obj::BaseRef,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Bullet {
    speed: f32,
    lifetime: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Bullet {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Bullet {
            speed: 400.0,
            lifetime: 5.0,
            base,
        }
    }

    fn ready(&mut self) {
        let bullet: BaseRef<'_, Bullet> = self.base();

        let on_timer_timeout = {
            let mut bullet: Gd<CharacterBody2D> = bullet.clone();
            move |_: &[&Variant]| -> Result<Variant, ()> {
                // attempted to free already freed reference
                bullet.queue_free();

                Ok(Variant::nil())
            }
        };

        // Create a properly connected timer
        let mut timer: Gd<SceneTreeTimer> = bullet
            .get_tree()
            .unwrap()
            .create_timer(self.lifetime)
            .unwrap();

        // Create a proper callable that can be used with connect
        let callable: Callable = Callable::from_local_fn("on_timer_timeout", on_timer_timeout);
        timer.connect("timeout", &callable);
    }

    fn process(&mut self, delta: f64) {
        let mut bullet: Gd<CharacterBody2D> = self.base_mut().clone();
        let velocity: Vector2 = bullet.get_velocity();

        // Perform collision detection
        if let Some(collision) = bullet.move_and_collide(velocity * self.speed * delta as f32) {
            collision.get_collider().unwrap().free();
            bullet.queue_free();
        }
    }
}
