use raylib::prelude::*;

use crate::components::transform::Transform3D;

/// A simple collision primitive used by the physics/interaction system.
///
/// # Invariants:
/// - Sphere `radius` should be >= 0.0.
/// - Box `half_size` components should be >= 0.0 (they represent half-extents).
///
/// # Examples
///
/// ```
/// let shape = /* a `CollisionShape` value */ ;
/// match shape {
///     CollisionShape::Box { half_size } => { /* use half_size */ }
///     CollisionShape::Sphere { radius } => { /* use radius */ }
/// }
/// ```
#[derive(Clone, Copy)]
pub enum CollisionShape {
    Box { half_size: Vector3 },
    Sphere { radius: f32 },
}

/// A collider is a CollisionShape combined with an offset.
/// It's main purpose is to handle collision with others.
///
/// # Examples
///
/// Create a box collider from a full size:
/// ```
/// let c = Collider::with_box_size(2.0, 1.0, 4.0);
/// ```
///
/// Create a two sphere collision test:
/// ```
/// let s = Collider::with_sphere(0.5);
/// let other = Collider::with_sphere(0.5);
/// let colliding = s.collides_with(&other, Vector3::ZERO, Vector3::ONE);
/// ```
#[derive(Clone, Copy)]
pub struct Collider {
    pub offset: Vector3,
    pub shape: CollisionShape,
}

impl Collider {
    /// Creates a box collider with the given half size.
    pub fn with_box(half_size: Vector3) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Box { half_size },
        }
    }

    /// Creates a box collider with the given full size.
    pub fn with_box_from_size(width: f32, height: f32, depth: f32) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Box {
                half_size: Vector3::new(width / 2.0, height / 2.0, depth / 2.0),
            },
        }
    }

    /// Creates a box collider with the given full size and offset.
    pub fn with_box_from_size_offset(width: f32, height: f32, depth: f32, offset: Vector3) -> Self {
        Self {
            offset: offset,
            shape: CollisionShape::Box {
                half_size: Vector3::new(width / 2.0, height / 2.0, depth / 2.0),
            },
        }
    }

    /// Creates a sphere collider with the given radius.
    pub fn with_sphere(radius: f32) -> Self {
        Self {
            offset: Vector3::ZERO,
            shape: CollisionShape::Sphere { radius },
        }
    }

    /// Returns the effective position of the collider
    fn effective_position(&self, base_position: Vector3) -> Vector3 {
        base_position + self.offset
    }

    /// Returns the axis-aligned bounding box of the collider
    /// The axis-aligned bounding box is a rectangle that encloses the collider,
    /// with a minimum and maximum position.
    /// It provides a simple way to check for collisions without too much computation.
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

    /// Checks if this collider collides with another collider at a given base position.
    pub fn collides_with(
        &self,
        base_position: Vector3,
        other: &Collider,
        other_base_pos: Vector3,
    ) -> bool {
        // Check if the aabb are far away first
        let (self_min, self_max) = self.get_aabb(base_position);
        let (other_min, other_max) = other.get_aabb(other_base_pos);

        if !(self_min.x < other_max.x
            && self_max.x > other_min.x
            && self_min.y < other_max.y
            && self_max.y > other_min.y
            && self_min.z < other_max.z
            && self_max.z > other_min.z)
        {
            // The aabb are far away, so there can't be a collision
            return false;
        }

        // They are close, so we check more deeply
        self.detailed_collision(base_position, other, other_base_pos)
    }

    /// Checks if this collider collides with another collider at a given base position.
    /// This is a detailed collision check that considers the shape of both colliders.
    fn detailed_collision(
        &self,
        base_position: Vector3,
        other: &Collider,
        other_base_pos: Vector3,
    ) -> bool {
        let self_pos = self.effective_position(base_position);
        let other_pos = other.effective_position(other_base_pos);

        return match (self.shape, other.shape) {
            // Box vs Box
            (CollisionShape::Box { half_size: _hs1 }, CollisionShape::Box { half_size: _hs2 }) => {
                self.aabb_collision(base_position, other, other_pos)
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
        };
    }

    /// Checks if two AABB colliders are colliding.
    fn aabb_collision(
        &self,
        base_position: Vector3,
        other: &Collider,
        other_pose: Vector3,
    ) -> bool {
        let (min1, max1) = self.get_aabb(base_position);
        let (min2, max2) = other.get_aabb(other_pose);

        return min1.x < max2.x
            && max1.x > min2.x
            && min1.y < max2.y
            && max1.y > min2.y
            && min1.z < max2.z
            && max1.z > min2.z;
    }

    /// Checks if a sphere collider collides with a box collider.
    fn sphere_box_collision(
        &self,
        sphere_pos: Vector3,
        sphere_radius: f32,
        box_pos: Vector3,
        box_half_size: Vector3,
    ) -> bool {
        // Find the closest point on the box to the sphere's center
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

    /// Checks if a point is contained within this collider.
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

    /// Returns the penetration vector between two colliders, if they are colliding.
    ///
    /// # Warning
    ///
    /// This method currently only supports sphere-box collisions.
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

    /// Draws the collider using the given 3D mode and base position.
    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>, base_position: Transform3D) {
        let pos = self.effective_position(base_position.position);
        match self.shape {
            CollisionShape::Box { half_size } => {
                d3d.draw_cube_wires(
                    pos,
                    half_size.x * 2.0,
                    half_size.y * 2.0,
                    half_size.z * 2.0,
                    Color::GREEN,
                );
            }
            CollisionShape::Sphere { radius } => {
                d3d.draw_sphere_wires(pos, radius, 8, 8, Color::GREEN);
            }
        }
    }
}
