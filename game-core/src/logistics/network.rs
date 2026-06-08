use std::collections::{HashMap, HashSet};

use sparsey::Entity;

use crate::engine;

pub type NetworkId = u32;

pub struct NetworkNode {
    pub network: NetworkId,
    pub range: f32,
}

pub fn update_network_system(world: &mut sparsey::World, debug: &mut engine::DebugLines) {
    world.for_each::<(&engine::Location, &NetworkNode)>(|(loc, node)| {
        // debug.add_circle(loc.0, 20., graphics::Color::GREEN);
    });
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
