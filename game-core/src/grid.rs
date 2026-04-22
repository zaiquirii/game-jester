use crate::ecs;
use glam::{IVec2, ivec2};

pub struct LevelData {
    pub boxes: Vec<IVec2>,
    pub walls: Vec<IVec2>,
    pub targets: Vec<IVec2>,
    pub player: IVec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Player,
    Box,
    Wall,
    Target,
}

pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    // Undo,
}

pub enum ActionResult {
    Success(Vec<EntityUpdate>),
    Failure { blocked_by: ecs::Entity },
}

pub struct EntityUpdate {
    pub entity: ecs::Entity,
    pub prev_pos: IVec2,
    pub new_pos: IVec2,
}

pub struct Location(pub IVec2);

pub struct Box {
    pub covering_target: bool,
}

pub struct Target {}
