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
    Failure { blocked_by: sparsey::Entity },
}

pub struct EntityUpdate {
    pub entity: sparsey::Entity,
    pub prev_pos: IVec2,
    pub new_pos: IVec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Location(pub IVec2);

pub struct Box {}

pub struct Target {
    pub covered: bool,
}

pub struct Player {}
