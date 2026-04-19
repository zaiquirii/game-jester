use std::any::{TypeId, type_name};
use std::collections::HashMap;

use crate::ecs::components::ComponentStore;
use crate::ecs::{
    components::TypeErasedComponentStore,
    entity::{Entity, EntityManager},
};

const MAX_ENTITY_COUNT: usize = 10000;

pub struct World {
    entity_manager: EntityManager,
    component_stores: HashMap<TypeId, Box<dyn TypeErasedComponentStore>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_stores: HashMap::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> Entity {
        self.entity_manager.get_entity()
    }

    pub fn emplace<T: 'static>(&mut self, entity: Entity, component: T) -> &mut Self {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get_mut(&type_id)
            .expect("attempted to emplace component type which does not exist in the world")
            .as_any_mut()
            .downcast_mut::<ComponentStore<_>>()
            .expect("failed to downcast component store")
            .emplace(entity, component);
        self
    }
    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get_mut(&type_id)
            .expect("attempted to get component type which does not exist in the world")
            .as_any_mut()
            .downcast_mut::<ComponentStore<T>>()
            .expect("failed to downcast component store")
            .get_mut(entity)
    }

    pub fn register_component<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        if self.component_stores.contains_key(&type_id) {
            panic!("component type {} is already registered", type_name::<T>());
        }
        self.component_stores.insert(
            type_id,
            Box::new(ComponentStore::<T>::new(MAX_ENTITY_COUNT)),
        );
    }

    pub fn iter<T: 'static>(&self) -> impl Iterator<Item = &T> {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get(&type_id)
            .expect("attempted to iterate over component type which does not exist in the world")
            .as_any()
            .downcast_ref::<ComponentStore<T>>()
            .expect("failed to downcast component store")
            .iter()
    }

    pub fn iter_ent<T: 'static>(&self) -> impl Iterator<Item = (Entity, &T)> {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get(&type_id)
            .expect("attempted to iterate over component type which does not exist in the world")
            .as_any()
            .downcast_ref::<ComponentStore<T>>()
            .expect("failed to downcast component store")
            .iter_ent()
    }
}
