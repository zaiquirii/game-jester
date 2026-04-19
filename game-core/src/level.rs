use crate::ecs::Entity;
use ggez::glam::uvec2;
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

#[derive(Clone, Copy)]
pub struct GridEntity {
    pub entity: Entity,
    pub position: UVec2,
}

pub struct SokoGrid {
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

    pub fn accept_action(&mut self, action: PlayerAction) -> Option<Vec<EntityUpdate>> {
        match action {
            PlayerAction::MoveLeft => self.move_player(-1, 0),
            PlayerAction::MoveRight => self.move_player(1, 0),
            PlayerAction::MoveUp => self.move_player(0, -1),
            PlayerAction::MoveDown => self.move_player(0, 1),
        }
    }

    fn move_player(&mut self, dx: i32, dy: i32) -> Option<Vec<EntityUpdate>> {
        self.player.position = UVec2::new(
            (self.player.position.x as i32 + dx) as u32,
            (self.player.position.y as i32 + dy) as u32,
        );
        Some(vec![EntityUpdate {
            entity: self.player,
        }])
    }
}

pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    // Undo,
}

pub struct EntityUpdate {
    pub entity: GridEntity,
}
