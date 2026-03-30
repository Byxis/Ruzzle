use crate::blocks::material::BlockMaterial;
use raylib::prelude::*;

use crate::blocks::modele::BlockType;
use crate::blocks::modele::GroupBlock;
use crate::levels::level::Level;

pub struct Level4 {
    pub groups: Vec<GroupBlock>,
    pub camera: Camera3D,
    pub selected_group: Option<usize>,
}

impl Level4 {
    pub fn new() -> Self {
        let mut groups = Vec::new();

        groups.push(GroupBlock::single(
            Vector3::new(-5.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(-4.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(-3.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(0.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(3.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(4.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));
        groups.push(GroupBlock::single(
            Vector3::new(5.0, 0.0, 0.0),
            BlockType::Fixe,
            BlockMaterial::sand(),
        ));

        Self {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            groups,
            selected_group: None,
        }
    }
}

impl Level for Level4 {
    fn groups_mut(&mut self) -> &mut Vec<GroupBlock> {
        &mut self.groups
    }
    fn camera(&self) -> &Camera3D {
        &self.camera
    }
    fn selected_group(&self) -> Option<usize> {
        self.selected_group
    }
    fn selected_group_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected_group
    }
}
