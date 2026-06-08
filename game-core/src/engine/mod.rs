mod debug;
mod sprites;

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub use debug::*;
pub use sprites::*;

pub struct Location(pub glam::Vec2);

pub struct Resources {
    data: HashMap<TypeId, Rc<RefCell<dyn Any>>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, value: T) -> &mut Self {
        assert!(
            !self.data.contains_key(&TypeId::of::<T>()),
            "resource of this type already exists"
        );
        self.data
            .insert(TypeId::of::<T>(), Rc::new(RefCell::new(value)));
        self
    }

    pub fn get<T: 'static>(&self) -> Ref<T> {
        let d = self.data.get(&TypeId::of::<T>()).unwrap();
        Ref::map(d.borrow(), |b| {
            b.downcast_ref::<T>().expect("type should match")
        })
    }

    pub fn get_mut<T: 'static>(&self) -> RefMut<T> {
        let d = self.data.get(&TypeId::of::<T>()).unwrap();
        RefMut::map(d.borrow_mut(), |b| {
            b.downcast_mut::<T>().expect("type should match")
        })
    }
}
