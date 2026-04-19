use crate::ecs::Entity;
use glam::UVec2;

pub struct LevelData {
    pub dimensions: UVec2,
    pub boxes: Vec<UVec2>,
    pub targets: Vec<UVec2>,
    pub player: UVec2,
}

pub struct CurrentLevel {
    grid: SokoGrid,
}

pub enum GridEntityType {
    Player,
    Box,
}

struct GridEntity {
    entity: Entity,
    position: UVec2,
}

pub struct SokoGrid {
    width: usize,
    height: usize,
    player: GridEntity,
    boxes: Vec<GridEntity>,
}

pub struct SokoGridBuilder {
    width: usize,
    height: usize,
    player: GridEntity,
    boxes: Vec<GridEntity>,
}

impl SokoGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            player: GridEntity {
                entity: Entity::none(),
                position: UVec2::ZERO,
            },
            boxes: Vec::new(),
        }
    }

    pub fn add_box(&mut self, entity: Entity, position: UVec2) {
        self.boxes.push(GridEntity { entity, position });
    }

    pub fn set_player(&mut self, entity: Entity, position: UVec2) {
        self.player = GridEntity { entity, position };
    }
}
