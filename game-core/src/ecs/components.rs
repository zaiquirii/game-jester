use crate::ecs::entity::Entity;
use std::any::type_name;

const SENTINEL: usize = usize::MAX;

pub trait TypeErasedComponentStore {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/**
 * ComponentStore stores components of a single type for entities. It is implemented as a sparse map.
 */
pub struct ComponentStore<ComponentType> {
    entities: Vec<Entity>,
    components: Vec<ComponentType>,
    // This could/should be a smaller int to save space, not worth doing right now though
    sparse: Vec<usize>,
}

impl<ComponentType: 'static> TypeErasedComponentStore for ComponentStore<ComponentType> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<ComponentType> ComponentStore<ComponentType> {
    pub fn new(max_entity_count: usize) -> Self {
        if max_entity_count >= SENTINEL as usize {
            panic!("max_entity_count must be less than {}", SENTINEL);
        }

        Self {
            entities: Vec::with_capacity(max_entity_count),
            components: Vec::with_capacity(max_entity_count),
            sparse: vec![SENTINEL; max_entity_count],
        }
    }

    pub fn emplace(&mut self, entity: Entity, component: ComponentType) {
        // This line will panic if entity id is greater than max_entity_count,
        // this will be enforced by the world to avoid perf hit to checking this for every insert
        let ent_index = self.sparse[entity.id as usize];
        if ent_index == SENTINEL {
            // not currently aware of this entity, let's add it to the end of the packed arrays
            self.sparse[entity.id as usize] = self.entities.len();
            self.entities.push(entity);
            self.components.push(component);
        } else {
            // update existing value
            assert!(entity.id == self.entities[ent_index].id);
            self.components[ent_index] = component;
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let ent_index = self.sparse[entity.id as usize];
        if ent_index == SENTINEL {
            println!(
                "attempted to remove {} from entity {:?}",
                type_name::<ComponentType>(),
                entity,
            );
        } else {
            self.sparse[ent_index] = SENTINEL;
            self.entities.swap_remove(ent_index);
            self.components.swap_remove(ent_index);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ComponentType> {
        self.components.iter()
    }

    pub fn iter_ent(&self) -> impl Iterator<Item = (Entity, &ComponentType)> {
        self.entities
            .iter()
            .zip(self.components.iter())
            .map(|(ent, comp)| (*ent, comp))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_component_store() {
        let mut store = ComponentStore::<i32>::new(10);
        let ent1 = Entity {
            id: 1,
            generation: 0,
        };
        let ent2 = Entity {
            id: 2,
            generation: 0,
        };

        store.emplace(ent1, 42);
        store.emplace(ent2, 99);

        let comps: Vec<_> = store.iter_ent().collect();
        assert_eq!(comps.len(), 2);
        assert_eq!(comps[0], (ent1, &42));
        assert_eq!(comps[1], (ent2, &99));

        store.remove(ent1);
        let comps: Vec<_> = store.iter_ent().collect();
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0], (ent2, &99));
    }
}
