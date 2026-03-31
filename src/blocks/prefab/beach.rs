use crate::blocks::material::BlockMaterial;
use crate::blocks::modele::{BlockPrefab, BlockType, GroupBlock};
use raylib::prelude::*;

/// Creates a vertical stack of three sand blocks at the specified position.
/// The group is initialized with `BlockType::All`, allowing both rotation and movement.
pub fn create_sand_row(pos: Vector3) -> GroupBlock {
    let mat = BlockMaterial::sand();
    let children = vec![
        BlockPrefab::new(Vector3::new(0.0, 0.0, 0.0), None, BlockType::Fixe, mat),
        BlockPrefab::new(Vector3::new(0.0, 1.0, 0.0), None, BlockType::Fixe, mat),
        BlockPrefab::new(Vector3::new(0.0, 2.0, 0.0), None, BlockType::Fixe, mat),
    ];
    GroupBlock::new(pos, children, BlockType::All)
}
