use crate::components::{collider::Collider, transform::Transform3D};
use raylib::prelude::*;

/// Represents a 3D map with a model and a list of colliders.
///
/// # Examples
///
/// ```
/// use raylib::prelude::*;
/// use ruzzle::components::map::Map;
///
/// let mut rl = RaylibHandle::new();
/// let thread = RaylibThread::new();
/// let mut map = Map::new(&mut rl, &thread, "path/to/model.obj");
/// let mut map = Map::set_position(&mut map, Vector3::ONE);
/// ```
pub struct Map {
    pub model: Model,
    pub colliders: Vec<Collider>,
    pub transform: Transform3D,
    pub spawn_point: Transform3D,
}

impl Map {
    /// Creates a new `Map` with the given model path.
    /// The transform is initialized to identity (position: `Vector3::ZERO`, rotation: `0.0`).
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        Self {
            model: rl.load_model(thread, path).expect("Failed to load model"),
            colliders: Vec::new(),
            transform: Transform3D::IDENTITY,
            spawn_point: Transform3D::IDENTITY,
        }
    }

    /// Sets the position of the map's transform.
    pub fn set_position(&mut self, new_position: Vector3) {
        self.transform.position = new_position;
    }

    pub fn set_spawn_point(&mut self, spawn_point: Transform3D) {
        self.spawn_point = spawn_point;
    }

    /// Adds a collider to the map.
    pub fn add_collider(&mut self, collider: Collider) {
        self.colliders.push(collider);
    }

    /// Returns `true` if the map collides with the given collider at the given position.
    pub fn collides_with(&self, other: &Collider, other_position: Vector3) -> bool {
        self.colliders
            .iter()
            .any(|c| c.collides_with(self.transform.position, other, other_position))
    }

    /// Draws the map using the given 3D drawing context.
    pub fn draw(&self, d3d: &mut impl RaylibDraw3D) {
        d3d.draw_model_ex(
            &self.model,
            self.transform.position,
            Vector3::new(0.0, 1.0, 0.0),
            self.transform.rotation,
            Vector3::ONE,
            Color::WHITE,
        );
    }

    /// Draws the map's colliders using the given 3D drawing context.
    pub fn draw_collider(&self, d3d: &mut impl RaylibDraw3D) {
        for collider in &self.colliders {
            collider.draw(d3d, self.transform);
        }
    }

    /// Resolves collisions for the given collider at the given position, returning the new position.
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

    /// Returns `true` if the given collider is grounded (touching a map collider below).
    pub fn is_grounded(&self, collider: &Collider, position: Vector3) -> bool {
        let ground_check_pos = position - Vector3::new(0.0, 0.05, 0.0);
        self.collides_with(collider, ground_check_pos)
    }

    /// Check if the given position is out of the map bounds, and returns the spawn point position in that case.
    pub fn handle_out_of_map(&self, position: Vector3) -> Vector3 {
        if position.x < -50.0
            || position.x > 50.0
            || position.z < -50.0
            || position.z > 50.0
            || position.y < -20.0
        {
            return self.spawn_point.position;
        }
        position
    }
}
