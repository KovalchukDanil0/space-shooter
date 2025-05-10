use godot::{
    classes::{CanvasLayer, HBoxContainer, ICanvasLayer, TextureRect},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct UI {
    health_instance: OnReady<Gd<PackedScene>>,
    health_container: OnReady<Gd<HBoxContainer>>,
    player: OnReady<Gd<Player>>,

    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for UI {
    fn init(base: Base<CanvasLayer>) -> Self {
        UI {
            health_instance: OnReady::new(|| load("res://instances/health.tscn")),
            health_container: OnReady::node("HealthContainer"),
            player: OnReady::from_base_fn(|base: &Gd<Node>| {
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
        self.init_health();
    }
}

#[godot_api]
impl UI {
    fn init_health(&mut self) {
        let player_health: i32 = self.player.bind_mut().get_health();

        for x in 1..player_health + 1 {
            let mut health_instance: Gd<TextureRect> =
                self.health_instance.instantiate_as::<TextureRect>();

            health_instance.set_position(Vector2 {
                x: 20.0 * x as f32,
                y: 20.0,
            });

            self.health_container.add_child(&health_instance);
        }
    }

    pub fn change_health(&mut self, amount: i32) {
        // TODO: debug

        let current_health: i32 = self.health_container.get_child_count();

        if amount > 0 {
            // Remove health nodes
            for _ in 0..amount {
                if let Some(mut last_child) = self.health_container.get_child(current_health - 1) {
                    last_child.queue_free();
                }
            }
        } else if amount < 0 {
            // Add health nodes
            for _ in 0..amount {
                let health_instance: Gd<TextureRect> =
                    self.health_instance.instantiate_as::<TextureRect>();

                self.health_container.add_child(&health_instance);
            }
        }
    }
}
