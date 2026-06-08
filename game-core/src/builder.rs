use ggez::graphics::{self, Color};
use glam::Vec2;
use sparsey::Entity;

use crate::{engine, input, logistics};

pub struct State {
    // TODO: find a better name for the indicator for where things will be built
    ghost: Entity,
}

impl State {
    pub fn new(world: &mut sparsey::World) -> Self {
        // Initialize the ghost entity here
        let ghost = world.create((
            engine::Location(Vec2::ZERO),
            engine::SpriteShape {
                shape: engine::Shape::Circle { radius: 25.0 },
                color: graphics::Color::MAGENTA,
                filled: true,
            },
        ));

        Self { ghost }
    }
}

pub fn update_state_system(world: &mut sparsey::World, resources: &engine::Resources) {
    let input = resources.get::<input::State>();
    let state = resources.get::<State>();

    world
        .query_one::<&mut engine::Location>()
        .get(state.ghost)
        .map(|loc| {
            loc.0 = input.selector_pos();
        });

    if input.select_pressed() {
        world.create((
            engine::Location(input.selector_pos()),
            engine::SpriteShape {
                shape: engine::Shape::Circle { radius: 25.0 },
                color: graphics::Color::GREEN,
                filled: false,
            },
            logistics::NetworkNode {
                network: 1,
                range: 100.,
            },
        ));
    }
}
