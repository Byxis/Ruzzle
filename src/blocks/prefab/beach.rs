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

pub fn create_level1(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for x in -5..=5 {
        groups.push(BlockPrefab::new(
            Vector3::new(x as f32, 0.0, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }
    GroupBlock::new(pos, groups, BlockType::All)
}

pub fn create_level2(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for x in -5..=-3 {
        groups.push(BlockPrefab::new(
            Vector3::new(x as f32, 0.0, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }
    for z in 1..=2 {
        groups.push(BlockPrefab::new(
            Vector3::new(-3.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    for x in -3..=3 {
        groups.push(BlockPrefab::new(
            Vector3::new(x as f32, 0.0, 2.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    for z in 1..=2 {
        groups.push(BlockPrefab::new(
            Vector3::new(3.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    for x in 3..=5 {
        groups.push(BlockPrefab::new(
            Vector3::new(x as f32, 0.0, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    GroupBlock::new(pos, groups, BlockType::Fixe)
}
