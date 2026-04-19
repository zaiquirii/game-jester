use crate::ecs::Entity;
use glam::{IVec2, ivec2};

pub struct LevelData {
    pub dimensions: IVec2,
    pub boxes: Vec<IVec2>,
    pub targets: Vec<IVec2>,
    pub player: IVec2,
}

pub struct CurrentLevel {
    grid: SokoGrid,
}

#[derive(Clone, Copy, Debug)]
pub enum GridEntityType {
    Player,
    Box,
}

#[derive(Clone, Copy)]
pub struct GridEntity {
    pub entity: Entity,
    pub entity_type: GridEntityType,
    pub position: IVec2,
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
                position: IVec2::ZERO,
                entity_type: GridEntityType::Player,
            },
            boxes: Vec::new(),
        }
    }

    pub fn add_box(&mut self, entity: Entity, position: IVec2) {
        self.boxes.push(GridEntity {
            entity,
            position,
            entity_type: GridEntityType::Box,
        });
    }

    pub fn set_player(&mut self, entity: Entity, position: IVec2) {
        self.player = GridEntity {
            entity,
            position,
            entity_type: GridEntityType::Player,
        };
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
        let mut updates = Vec::new();

        let delta = ivec2(dx, dy);
        let target_player_position = self.player.position + delta;
        if let Some(box_index) = self.box_at(target_player_position) {
            let b = &mut self.boxes[box_index];
            let prev = b.position;
            b.position += delta;
            updates.push(EntityUpdate {
                entity: *b,
                previous_position: prev,
            });
        }

        let previous_position = self.player.position;
        self.player.position = target_player_position;
        updates.push(EntityUpdate {
            entity: self.player,
            previous_position,
        });

        if updates.len() > 0 {
            Some(updates)
        } else {
            None
        }
    }

    fn box_at(&mut self, pos: IVec2) -> Option<usize> {
        self.boxes.iter_mut().position(|b| b.position.eq(&pos))
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
    pub previous_position: IVec2,
}
