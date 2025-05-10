use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, Timer},
    obj::BaseRef,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Bullet {
    speed: f32,
    life_time: f64,

    delete_timer: OnReady<Gd<Timer>>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Bullet {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Bullet {
            speed: 800.0,
            life_time: 5.0,
            delete_timer: OnReady::node("DeleteTimer"),
            base,
        }
    }

    fn ready(&mut self) {
        let bullet: BaseRef<'_, Bullet> = self.base();

        let on_timer_timeout = {
            let mut bullet: Gd<CharacterBody2D> = bullet.clone();
            move |_: &[&Variant]| -> Result<Variant, ()> {
                bullet.queue_free();

                Ok(Variant::nil())
            }
        };

        self.delete_timer.set_wait_time(self.life_time);

        // Create a proper callable that can be used with connect
        let callable: Callable = Callable::from_local_fn("on_timer_timeout", on_timer_timeout);
        self.delete_timer.connect("timeout", &callable);

        self.delete_timer.start();
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
