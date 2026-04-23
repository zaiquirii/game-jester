use std::any::{TypeId, type_name};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::ecs::components::ComponentStore;
use crate::ecs::{
    components::TypeErasedComponentStore,
    entity::{Entity, EntityManager},
};

const MAX_ENTITY_COUNT: usize = 10000;

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

// pub struct Components<T> {
//     store: Ref<ComponentStore<T>>,
// }

// impl<T: 'static> Components<T> {
//     pub fn new(store: Ref<ComponentStore) -> Self {
//         Self {
//             store,
//         }
//     }

//     pub fn borrow(&self) -> std::cell::Ref<ComponentStore<T>> {
//         std::cell::Ref::map(self.store.borrow(), |boxed_store| {
//             boxed_store.as_any().downcast_ref::<ComponentStore<T>>().expect("failed to downcast component store")
//         })
//     }
// }

pub struct World {
    entity_manager: EntityManager,
    component_stores: HashMap<TypeId, Rc<RefCell<Box<dyn TypeErasedComponentStore>>>>,
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

    pub fn components<T: 'static>(&self) -> Ref<ComponentStore<T>> {
        self.component_store::<T>()
    }

    pub fn components_mut<T: 'static>(&self) -> RefMut<ComponentStore<T>> {
        self.component_store_mut()
    }

    fn component_store<T: 'static>(&self) -> Ref<ComponentStore<T>> {
        let type_id = TypeId::of::<T>();
        let store = self
            .component_stores
            .get(&type_id)
            .expect(&format!(
                "attempted to access unregistered component type: {}",
                type_name::<T>()
            ))
            .borrow();
        Ref::map(store, |boxed_store| {
            boxed_store
                .as_any()
                .downcast_ref::<ComponentStore<T>>()
                .expect("failed to downcast component store")
        })
    }

    fn component_store_mut<T: 'static>(&self) -> RefMut<ComponentStore<T>> {
        let type_id = TypeId::of::<T>();
        let store = self
            .component_stores
            .get(&type_id)
            .expect(&format!(
                "attempted to access unregistered component type: {}",
                type_name::<T>()
            ))
            .borrow_mut();
        RefMut::map(store, |boxed_store| {
            boxed_store
                .as_any_mut()
                .downcast_mut::<ComponentStore<T>>()
                .expect("failed to downcast component store")
        })
    }

    pub fn emplace<T: 'static>(&mut self, entity: Entity, component: T) -> &mut Self {
        self.component_store_mut::<T>().emplace(entity, component);
        self
    }

    // pub fn get<T: 'static>(&self, entity: Entity) -> &T {
    //     self.component_store::<T>()
    //         .get(entity)
    //         .expect("attempted to get component which does not exist on entity")
    // }

    // pub fn get_opt<T: 'static>(&self, entity: Entity) -> Option<&T> {
    //     self.component_store::<T>().get(entity)
    // }

    // pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> &mut T {
    //     self.component_store_mut::<T>()
    //         .get_mut(entity)
    //         .expect("attempted to get component which does not exist on entity")
    // }

    pub fn register_component<T: 'static>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        if self.component_stores.contains_key(&type_id) {
            panic!("component type {} is already registered", type_name::<T>());
        }
        self.component_stores.insert(
            type_id,
            Rc::new(RefCell::new(Box::new(ComponentStore::<T>::new(
                MAX_ENTITY_COUNT,
            )))),
        );
        self
    }

    // pub fn for_each<T1: 'static, T2: 'static>(&self, f: FnMut(&T1, &T2)) {
    //     let store1 = self.component_store::<T1>();
    //     let store2 = self.component_store::<T2>();
    // }
}
