use godot::{
    classes::{IRigidBody2D, RigidBody2D, Timer, VisibleOnScreenNotifier2D},
    obj::BaseRef,
    prelude::*,
};
use std::sync::{Arc, Mutex};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Meteor {
    speed: f32,
    pub velocity: Vector2,
    screen_notifier: OnReady<Gd<VisibleOnScreenNotifier2D>>,

    lifetime: f64,
    delete_timer: OnReady<Gd<Timer>>,

    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for Meteor {
    fn init(base: Base<RigidBody2D>) -> Self {
        Meteor {
            speed: 100.0,
            velocity: Vector2::ZERO,
            screen_notifier: OnReady::node("VisibleOnScreenNotifier2D"),

            lifetime: 5.0,
            delete_timer: OnReady::node("DeleteTimer"),

            base,
        }
    }

    fn ready(&mut self) {
        self.init_screen_notifier();
    }

    fn process(&mut self, _delta: f64) {
        let velocity: Vector2 = self.velocity.normalized_or_zero() * self.speed;
        self.base_mut().apply_force(velocity);
    }
}

impl Meteor {
    fn init_screen_notifier(&mut self) {
        self.delete_timer.set_wait_time(self.lifetime);
        self.delete_timer.start();

        let not_visible: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let base: BaseRef<'_, Meteor> = self.base();

        let on_screen_exited = {
            let not_visible: Arc<Mutex<bool>> = not_visible.clone();

            move |_: &[&Variant]| -> Result<Variant, ()> {
                not_visible.lock().unwrap().set_property(true);

                Ok(Variant::nil())
            }
        };

        let on_timer_timeout = {
            let mut base: Gd<RigidBody2D> = base.clone();

            let not_visible: Arc<Mutex<bool>> = not_visible.clone();
            let mut delete_timer: Gd<Timer> = self.delete_timer.clone();

            move |_: &[&Variant]| -> Result<Variant, ()> {
                if not_visible.lock().unwrap().get_property() {
                    base.queue_free();
                    godot_print!("I am not on the screen");
                } else {
                    delete_timer.start();
                }

                Ok(Variant::nil())
            }
        };

        let callable: Callable = Callable::from_local_fn("on_screen_exited", on_screen_exited);
        self.screen_notifier.connect("screen_exited", &callable);

        let callable: Callable = Callable::from_local_fn("on_timer_timeout", on_timer_timeout);
        self.delete_timer.connect("timeout", &callable);
    }
}
