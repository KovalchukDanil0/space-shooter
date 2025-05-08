use godot::{
    classes::{CanvasLayer, HBoxContainer, ICanvasLayer, TextureRect},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct UI {
    health_instance: Gd<PackedScene>,
    health_container: Option<Gd<HBoxContainer>>,

    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for UI {
    fn init(base: Base<CanvasLayer>) -> Self {
        UI {
            health_instance: load("res://instances/health.tscn"),
            health_container: None,
            base,
        }
    }

    fn ready(&mut self) {
        let health_instance: Gd<PackedScene> = self.health_instance.clone();

        let mut health_container: Gd<HBoxContainer> = self
            .base_mut()
            .get_node_as::<HBoxContainer>("HealthContainer");

        let mut tree: Gd<SceneTree> = health_container.get_tree().unwrap();

        let mut player: Gd<Player> = tree
            .get_first_node_in_group("player")
            .unwrap()
            .cast::<Player>();

        let player_health: i32 = player.bind_mut().get_health();

        for x in 1..player_health + 1 {
            let mut health_instance: Gd<TextureRect> =
                health_instance.instantiate().unwrap().cast::<TextureRect>();
            health_instance.set_position(Vector2 {
                x: 20.0 * x as f32,
                y: 20.0,
            });

            health_container.add_child(&health_instance);
        }

        self.health_container = Some(health_container);
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl UI {
    #[func]
    pub fn change_health(&mut self, amount: i32) {
        // TODO: debug

        let health_container: &mut Gd<HBoxContainer> = self.health_container.as_mut().unwrap();

        let current_health: i32 = health_container.get_child_count();

        if amount > 0 {
            // Remove health nodes
            for _ in 0..amount {
                if let Some(mut last_child) = health_container.get_child(current_health - 1) {
                    last_child.queue_free();
                }
            }
        } else if amount < 0 {
            // Add health nodes
            for _ in 0..amount {
                let health_instance: Gd<TextureRect> = self
                    .health_instance
                    .instantiate()
                    .unwrap()
                    .cast::<TextureRect>();
                health_container.add_child(&health_instance);
            }
        }
    }
}
