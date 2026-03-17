use raylib::prelude::*;

use crate::components::transform::Transform3D;

#[derive(Clone, Copy)]
pub enum CollisionShape {
    Box { half_size: Vector3 },
    Sphere { radius: f32 },
}

#[derive(Clone, Copy)]
pub struct Collider {
    pub offset: Vector3,
    pub shape: CollisionShape,
}

impl Collider {
    pub fn new_box(half_size: Vector3) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Box { half_size },
        }
    }

    pub fn new_box_from_size(width: f32, height: f32, depth: f32) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Box {
                half_size: Vector3::new(width / 2.0, height / 2.0, depth / 2.0),
            },
        }
    }

    pub fn new_box_from_size_offset(width: f32, height: f32, depth: f32, offset: Vector3) -> Self {
        Self {
            offset: offset,
            shape: CollisionShape::Box {
                half_size: Vector3::new(width / 2.0, height / 2.0, depth / 2.0),
            },
        }
    }

    pub fn new_sphere(radius: f32) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Sphere { radius },
        }
    }

    fn effective_position(&self, base_position: Vector3) -> Vector3 {
        base_position + self.offset
    }

    // Axis-Aligned Bouding Box
    fn get_aabb(&self, base_position: Vector3) -> (Vector3, Vector3) {
        let pos = self.effective_position(base_position);
        match self.shape {
            CollisionShape::Box { half_size } => (pos - half_size, pos + half_size),
            CollisionShape::Sphere { radius } => {
                let r = Vector3::new(radius, radius, radius);
                (pos - r, pos + r)
            }
        }
    }

    pub fn collides_with(&self, base_position: Vector3, other: &Collider) -> bool {
        // Check if the aabb are far away first
        let (self_min, self_max) = self.get_aabb(base_position);
        let (other_min, other_max) = other.get_aabb(base_position);

        if !(self_min.x < other_max.x
            && self_max.x > other_min.x
            && self_min.y < other_max.y
            && self_max.y > other_min.y
            && self_min.z < other_max.z
            && self_max.z > other_min.z)
        {
            return false;
        }

        // They are close, so we check more deeply
        self.detailed_collision(base_position, other)
    }

    fn detailed_collision(&self, base_position: Vector3, other: &Collider) -> bool {
        let self_pos = self.effective_position(base_position);
        let other_pos = other.effective_position(base_position);

        match (self.shape, other.shape) {
            // Box vs Box
            (CollisionShape::Box { half_size: hs1 }, CollisionShape::Box { half_size: hs2 }) => {
                let self_min = self_pos - hs1;
                let self_max = self_pos + hs1;
                let other_min = other_pos - hs2;
                let other_max = other_pos + hs2;

                self_min.x < other_max.x
                    && self_max.x > other_min.x
                    && self_min.y < other_max.y
                    && self_max.y > other_min.y
                    && self_min.z < other_max.z
                    && self_max.z > other_min.z
            }

            // Sphere vs Sphere
            (CollisionShape::Sphere { radius: r1 }, CollisionShape::Sphere { radius: r2 }) => {
                let dist = self_pos.distance(other_pos);
                dist < r1 + r2
            }

            // Box vs Sphere
            (CollisionShape::Box { half_size }, CollisionShape::Sphere { radius }) => {
                self.sphere_box_collision(other_pos, radius, self_pos, half_size)
            }

            // Sphere vs Box
            (CollisionShape::Sphere { radius }, CollisionShape::Box { half_size }) => {
                self.sphere_box_collision(self_pos, radius, other_pos, half_size)
            }
        }
    }

    fn sphere_box_collision(
        &self,
        sphere_pos: Vector3,
        sphere_radius: f32,
        box_pos: Vector3,
        box_half_size: Vector3,
    ) -> bool {
        let closest_x = sphere_pos
            .x
            .clamp(box_pos.x - box_half_size.x, box_pos.x + box_half_size.x);
        let closest_y = sphere_pos
            .y
            .clamp(box_pos.y - box_half_size.y, box_pos.y + box_half_size.y);
        let closest_z = sphere_pos
            .z
            .clamp(box_pos.z - box_half_size.z, box_pos.z + box_half_size.z);

        let dist = Vector3::new(
            sphere_pos.x - closest_x,
            sphere_pos.y - closest_y,
            sphere_pos.z - closest_z,
        )
        .length();

        dist < sphere_radius
    }

    pub fn contains_point(&self, base_position: Vector3, point: Vector3) -> bool {
        let pos = self.effective_position(base_position);
        match self.shape {
            CollisionShape::Box { half_size } => {
                let min = pos - half_size;
                let max = pos + half_size;

                point.x >= min.x
                    && point.x <= max.x
                    && point.y >= min.y
                    && point.y <= max.y
                    && point.z >= min.z
                    && point.z <= max.z
            }
            CollisionShape::Sphere { radius } => pos.distance(point) <= radius,
        }
    }

    pub fn get_penetration_vector(
        &self,
        self_base_pos: Vector3,
        other: &Collider,
        other_base_pos: Vector3,
    ) -> Option<Vector3> {
        let self_pos = self.effective_position(self_base_pos);
        let other_pos = other.effective_position(other_base_pos);

        match (self.shape, other.shape) {
            (CollisionShape::Sphere { radius }, CollisionShape::Box { half_size }) => {
                let min = other_pos - half_size;
                let max = other_pos + half_size;

                let closest = Vector3::new(
                    self_pos.x.clamp(min.x, max.x),
                    self_pos.y.clamp(min.y, max.y),
                    self_pos.z.clamp(min.z, max.z),
                );

                let diff = self_pos - closest;
                let dist = diff.length();

                if dist < radius && dist > 0.0 {
                    Some((diff / dist) * (radius - dist))
                } else if dist == 0.0 {
                    let d_min_x = (self_pos.x - min.x).abs();
                    let d_max_x = (self_pos.x - max.x).abs();
                    let d_min_y = (self_pos.y - min.y).abs();
                    let d_max_y = (self_pos.y - max.y).abs();
                    let d_min_z = (self_pos.z - min.z).abs();
                    let d_max_z = (self_pos.z - max.z).abs();

                    let min_dist = d_min_x
                        .min(d_max_x)
                        .min(d_min_y)
                        .min(d_max_y)
                        .min(d_min_z)
                        .min(d_max_z);

                    if min_dist == d_max_x {
                        Some(Vector3::new(radius + min_dist, 0.0, 0.0))
                    } else if min_dist == d_min_x {
                        Some(Vector3::new(-(radius + min_dist), 0.0, 0.0))
                    } else if min_dist == d_max_y {
                        Some(Vector3::new(0.0, radius + min_dist, 0.0))
                    } else if min_dist == d_min_y {
                        Some(Vector3::new(0.0, -(radius + min_dist), 0.0))
                    } else if min_dist == d_max_z {
                        Some(Vector3::new(0.0, 0.0, radius + min_dist))
                    } else {
                        Some(Vector3::new(0.0, 0.0, -(radius + min_dist)))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>, base_position: Transform3D) {
        let pos = self.effective_position(base_position.position);
        match self.shape {
            CollisionShape::Box { half_size } => {
                self.draw_box(d3d, pos, half_size, Color::GREEN);
            }
            CollisionShape::Sphere { radius } => {
                d3d.draw_sphere_wires(pos, radius, 8, 8, Color::GREEN);
            }
        }
    }

    fn draw_box(
        &self,
        d3d: &mut RaylibMode3D<'_, impl RaylibDraw>,
        pos: Vector3,
        half_size: Vector3,
        color: Color,
    ) {
        let min = pos - half_size;
        let max = pos + half_size;

        // Bottom face
        d3d.draw_line3D(
            Vector3::new(min.x, min.y, min.z),
            Vector3::new(max.x, min.y, min.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, min.y, min.z),
            Vector3::new(max.x, min.y, max.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, min.y, max.z),
            Vector3::new(min.x, min.y, max.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(min.x, min.y, max.z),
            Vector3::new(min.x, min.y, min.z),
            color,
        );

        // Top face
        d3d.draw_line3D(
            Vector3::new(min.x, max.y, min.z),
            Vector3::new(max.x, max.y, min.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, max.y, min.z),
            Vector3::new(max.x, max.y, max.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, max.y, max.z),
            Vector3::new(min.x, max.y, max.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(min.x, max.y, max.z),
            Vector3::new(min.x, max.y, min.z),
            color,
        );

        // Vertical edges
        d3d.draw_line3D(
            Vector3::new(min.x, min.y, min.z),
            Vector3::new(min.x, max.y, min.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, min.y, min.z),
            Vector3::new(max.x, max.y, min.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(max.x, min.y, max.z),
            Vector3::new(max.x, max.y, max.z),
            color,
        );
        d3d.draw_line3D(
            Vector3::new(min.x, min.y, max.z),
            Vector3::new(min.x, max.y, max.z),
            color,
        );
    }
}
