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

pub struct EntityEditor<'a> {
    pub entity: Entity,
    world: &'a mut World,
}

impl EntityEditor<'_> {
    pub fn emplace<T: 'static>(&mut self, component: T) -> &mut Self {
        self.world.emplace(self.entity, component);
        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_stores: HashMap::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> EntityEditor {
        EntityEditor {
            entity: self.entity_manager.get_entity(),
            world: self,
        }
    }

    fn component_store<T: 'static>(&self) -> &ComponentStore<T> {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get(&type_id)
            .expect(&format!(
                "attempted to access unregistered component type: {}",
                type_name::<T>()
            ))
            .as_any()
            .downcast_ref::<ComponentStore<T>>()
            .expect("failed to downcast component store")
    }

    fn component_store_mut<T: 'static>(&mut self) -> &mut ComponentStore<T> {
        let type_id = TypeId::of::<T>();
        self.component_stores
            .get_mut(&type_id)
            .expect(&format!(
                "attempted to access unregistered component type: {}",
                type_name::<T>()
            ))
            .as_any_mut()
            .downcast_mut::<ComponentStore<T>>()
            .expect("failed to downcast component store")
    }

    pub fn emplace<T: 'static>(&mut self, entity: Entity, component: T) -> &mut Self {
        self.component_store_mut::<T>()
            .emplace(entity, component);
        self
    }

    pub fn get<T: 'static>(&self, entity: Entity) -> &T {
        self.component_store::<T>()
            .get(entity)
            .expect("attempted to get component which does not exist on entity")
    }

    pub fn get_opt<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_store::<T>().get(entity)
    }

    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> &mut T {
        self.component_store_mut::<T>()
            .get_mut(entity)
            .expect("attempted to get component which does not exist on entity")
    }

    pub fn register_component<T: 'static>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        if self.component_stores.contains_key(&type_id) {
            panic!("component type {} is already registered", type_name::<T>());
        }
        self.component_stores.insert(
            type_id,
            Box::new(ComponentStore::<T>::new(MAX_ENTITY_COUNT)),
        );
        self
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
