#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

impl Entity {
    pub fn none() -> Self {
        Self {
            id: 0,
            generation: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        self.id == 0 && self.generation == 0
    }
}

pub struct EntityManager {
    next_id: u32,
    returned_entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            returned_entities: Vec::new(),
        }
    }

    pub fn return_entity(&mut self, entity: Entity) {
        self.returned_entities.push(entity);
    }

    pub fn get_entity(&mut self) -> Entity {
        match self.returned_entities.pop() {
            Some(entity) => entity,
            None => {
                let id = self.next_id;
                self.next_id += 1;
                Entity { id, generation: 1 }
            }
        }
    }
}
