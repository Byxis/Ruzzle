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

pub fn level_start(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];
    groups.push(BlockPrefab::new(
        Vector3::new(0.0, 0.0, -5.0),
        None,
        BlockType::Fixe,
        BlockMaterial::sand(),
    ));
    groups.push(BlockPrefab::new(
        Vector3::new(0.0, 0.0, -6.0),
        None,
        BlockType::Fixe,
        BlockMaterial::sand(),
    ));
    groups.push(BlockPrefab::new(
        Vector3::new(-1.0, 0.0, -5.0),
        None,
        BlockType::Fixe,
        BlockMaterial::sand(),
    ));
    groups.push(BlockPrefab::new(
        Vector3::new(-1.0, 0.0, -6.0),
        None,
        BlockType::Fixe,
        BlockMaterial::sand(),
    ));

    GroupBlock::new(pos, groups, BlockType::Fixe)
}

pub fn level1_moving_block(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for z in -3..=3 {
        groups.push(BlockPrefab::new(
            Vector3::new(0.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(1.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    GroupBlock::new(pos, groups, BlockType::Drag).with_end_pos(Vector3::new(0.0, 0.0, 0.0))
}

pub fn flag_block(pos: Vector3, rl: &mut RaylibHandle, thread: &RaylibThread) -> GroupBlock {
    let mat = BlockMaterial::sand();
    let children = vec![
        BlockPrefab::new(Vector3::new(-0.5, 0.0, -0.5), None, BlockType::Fixe, mat),
        BlockPrefab::new(Vector3::new(0.5, 0.0, -0.5), None, BlockType::Fixe, mat),
        BlockPrefab::new(Vector3::new(-0.5, 0.0, 0.5), None, BlockType::Fixe, mat),
        BlockPrefab::new(Vector3::new(0.5, 0.0, 0.5), None, BlockType::Fixe, mat),
    ];

    let mut group = GroupBlock::new(pos, children, BlockType::All);
    group.endpoint_local = Some(Vector3::new(0.0, 1.0, 0.0));

    group.model = rl.load_model(thread, "rsc/flag.glb").ok();
    group.model_offset = Vector3::new(0.0, 1.0, 0.0);

    group
}

pub fn level2_moving_block(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for z in -4..=-1 {
        groups.push(BlockPrefab::new(
            Vector3::new(0.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(1.0, 0.0, z as f32),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    GroupBlock::new(pos, groups, BlockType::Drag)
}

pub fn level3_rotating_block(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for z in -5..=5 {
        groups.push(BlockPrefab::new(
            Vector3::new(0.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(-1.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(1.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    GroupBlock::new(pos, groups, BlockType::RotationV)
}

pub fn level4_rotating_block(pos: Vector3) -> GroupBlock {
    let mut groups = vec![];

    for z in 0..=4 {
        groups.push(BlockPrefab::new(
            Vector3::new(0.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(-1.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(BlockPrefab::new(
            Vector3::new(1.0, z as f32, 0.0),
            None,
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
    }

    GroupBlock::new(pos, groups, BlockType::RotationV)
}
