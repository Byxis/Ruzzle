use crate::components::{collider::Collider, transform::Transform3D};
use raylib::{math::glam::Vec3, prelude::*};

pub struct Map {
    pub model: Model,
    pub colliders: Vec<Collider>,
    pub transform: Transform3D,
}

impl Map {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        Self {
            model: rl.load_model(thread, path).expect("Failed to load model"),
            colliders: Vec::new(),
            transform: Transform3D::new(Vec3::ZERO, 0.0),
        }
    }

    pub fn set_position(&mut self, new_position: Vector3) {
        self.transform.position = new_position;
    }

    pub fn add_collider(&mut self, collider: Collider) {
        self.colliders.push(collider);
    }

    pub fn collides_with(&self, other: &Collider, other_position: Vector3) -> bool {
        self.colliders
            .iter()
            .any(|c| c.collides_with(self.transform.position, other, other_position))
    }

    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>) {
        d3d.draw_model_ex(
            &self.model,
            self.transform.position,
            Vector3::new(0.0, 1.0, 0.0),
            self.transform.rotation,
            Vector3::ONE,
            Color::WHITE,
        );
    }

    pub fn draw_collider(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>) {
        for collider in &self.colliders {
            collider.draw(d3d, self.transform);
        }
    }

    pub fn resolve_collisions(&self, collider: &Collider, mut position: Vector3) -> Vector3 {
        for other in &self.colliders {
            if let Some(push_vec) =
                collider.get_penetration_vector(position, other, self.transform.position)
            {
                position += push_vec;
            }
        }
        position
    }
}
