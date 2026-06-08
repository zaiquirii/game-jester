use std::collections::{HashMap, HashSet};

use ggez::graphics;
use sparsey::Entity;

use crate::engine;

pub type NetworkId = u32;

pub struct NetworkNode {
    pub network: NetworkId,
    pub range: f32,
}

pub struct Networks {
    edges: HashMap<Entity, HashSet<Entity>>,
}

impl Networks {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn track_entity(&mut self, world: &sparsey::World, ent: Entity) {
        assert!(
            !self.edges.contains_key(&ent),
            "entity should only be tracked once"
        );

        let mut edges = HashSet::new();
        let mut query = world.query_one::<(&engine::Location, &NetworkNode)>();
        let (ent_loc, ent_node) = query
            .get(ent)
            .expect("ent should have NetworkNode and location component");
        world.for_each::<(Entity, &engine::Location, &NetworkNode)>(
            |(other_ent, other_loc, other_node)| {
                // Smallest range wins
                let range = ent_node.range.min(other_node.range);
                let cutoff = range * range;
                let distance_squared = ent_loc.0.distance_squared(other_loc.0);
                if distance_squared <= cutoff {
                    edges.insert(other_ent);
                }
            },
        );
        self.edges.insert(ent, edges);
    }
}

pub fn debug_draw_system(world: &sparsey::World, resources: &engine::Resources) {
    let mut debug = resources.get_mut::<engine::DebugLines>();
    let networks = resources.get::<Networks>();

    let mut locations = world.query_one::<(&engine::Location)>();
    for (left_ent, others) in networks.edges.iter() {
        let left_loc = locations.get(*left_ent).unwrap().0;
        for right_ent in others.iter() {
            let right_loc = locations.get(*right_ent).unwrap().0;
            debug.add_line(left_loc, right_loc, graphics::Color::YELLOW);
        }
    }
}
