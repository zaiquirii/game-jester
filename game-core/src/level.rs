use crate::ecs::Entity;
use glam::{IVec2, ivec2};

pub struct LevelData {
    pub dimensions: IVec2,
    pub boxes: Vec<IVec2>,
    pub targets: Vec<IVec2>,
    pub player: IVec2,
}

#[derive(Clone, Copy, Debug)]
pub enum GridEntityType {
    Player,
    Box,
    Space,
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

    pub fn accept_action(&mut self, action: PlayerAction) -> ActionResult {
        match action {
            PlayerAction::MoveLeft => self.move_player(-1, 0),
            PlayerAction::MoveRight => self.move_player(1, 0),
            PlayerAction::MoveUp => self.move_player(0, -1),
            PlayerAction::MoveDown => self.move_player(0, 1),
        }
    }

    fn move_player(&mut self, dx: i32, dy: i32) -> ActionResult {
        let mut updates = Vec::new();

        let delta = ivec2(dx, dy);
        let target_player_pos = self.player.position + delta;

        match self.entity_type_at(target_player_pos) {
            GridEntityType::Player => unreachable!(),
            GridEntityType::Space => {
                self.player.position = target_player_pos;
                updates.push(EntityUpdate {
                    entity: self.player,
                    previous_position: self.player.position,
                });
            }
            GridEntityType::Box => {
                let target_box_pos = target_player_pos + delta;
                let next_entity_type = self.entity_type_at(target_box_pos);

                match next_entity_type {
                    GridEntityType::Player => unreachable!(),
                    GridEntityType::Box => {
                        return ActionResult::Failure {
                            blocked_by: self.boxes[self.box_at(target_box_pos).unwrap()],
                        };
                    }
                    GridEntityType::Space => {
                        self.player.position = target_player_pos;
                        updates.push(EntityUpdate {
                            entity: self.player,
                            previous_position: self.player.position,
                        });

                        let box_index = self.box_at(target_player_pos).unwrap();
                        self.boxes[box_index].position = target_box_pos;
                        updates.push(EntityUpdate {
                            entity: self.boxes[box_index],
                            previous_position: target_player_pos,
                        })
                    }
                }
            }
        }

        // let check_open_pos = target_player_pos;

        // match self.entity_type_at(check_open_pos) {
        //     GridEntityType::Player => unreachable!(),
        //     GridEntityType::Box => {
        //         let check_box_pos = check_open_pos + delta;
        //         if !self.is_open(check_box_pos) {
        //             return ActionResult::Failure {
        //                 blocked_by: self.entity_type_at(check_box_pos).unwrap(),
        //             };
        //         }
        //     }
        // }

        // if !self.is_open(check_open_pos) {
        //     return ActionResult::Failure {
        //         blocked_by: self.entity_type_at(check_open_pos).unwrap(),
        //     };
        // }

        // if let Some(box_index) = self.box_at(target_player_pos) {
        //     let b = &mut self.boxes[box_index];
        //     let prev = b.position;
        //     b.position += delta;
        //     updates.push(EntityUpdate {
        //         entity: *b,
        //         previous_position: prev,
        //     });
        // }

        // let previous_position = self.player.position;
        // self.player.position = target_player_pos;
        // updates.push(EntityUpdate {
        //     entity: self.player,
        //     previous_position,
        // });

        ActionResult::Success(updates)
    }

    fn box_at(&self, pos: IVec2) -> Option<usize> {
        self.boxes.iter().position(|b| b.position == pos)
    }

    fn is_open(&self, pos: IVec2) -> bool {
        match self.entity_type_at(pos) {
            GridEntityType::Player => false,
            GridEntityType::Box => false,
            GridEntityType::Space => true,
        }
    }

    fn entity_type_at(&self, pos: IVec2) -> GridEntityType {
        if let Some(_) = self.box_at(pos) {
            GridEntityType::Box
        } else if self.player.position == pos {
            GridEntityType::Player
        } else {
            GridEntityType::Space
        }
    }
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
    Failure { blocked_by: GridEntity },
}

pub struct EntityUpdate {
    pub entity: GridEntity,
    pub previous_position: IVec2,
}
