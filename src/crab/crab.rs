use crate::components::collider::Collider;
use crate::components::transform::Transform3D;
use crate::crab::crab_animator::CrabAnimation;
use crate::crab::crab_animator::CrabAnimator;
use crate::crab::crab_stats::CrabStats;
use crate::sound_manager::sound_manager::{SoundEffect, SoundManager};
use raylib::prelude::*;
use std::f32::consts::PI;

/// Represents a crab character in the game world.
///
/// Contains a transform and collider for positioning and collision detection,
/// as well as an animator for controlling the crab's animation.
///
/// # Examples
///
/// ```
/// use ruzzle::crab::Crab;
/// use raylib::prelude::*;
///
/// let mut rl = RaylibHandle::new();
/// let thread = &rl.get_thread();
/// let crab = Crab::new(&mut rl, thread, "path/to/model", Vector3::new(0.0, 0.0, 0.0), 0.0);
/// ```
pub struct Crab {
    pub transform: Transform3D,
    pub collider: Collider,
    jump_timer: f32,
    jump_start_y: f32,
    pub crab_animator: CrabAnimator,
    has_landed: bool,
}

impl Crab {
    //----------------------------------------------------------------
    //
    //                          Constructor
    //
    //----------------------------------------------------------------

    /// Creates a new `Crab` instance with the given parameters.
    /// The collider is initialized with a sphere shape and an offset of (0.0, 0.5, 0.0).
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        let mut collider = Collider::with_sphere(1.0);
        collider.offset = Vector3::new(0.0, 0.5, 0.0);

        Self {
            transform: Transform3D::IDENTITY,
            jump_timer: 0.0,
            jump_start_y: 0.0,
            crab_animator: CrabAnimator::new(rl, thread, path),
            has_landed: false,
            collider: collider,
        }
    }

    /// Creates a new `Crab` instance with the given parameters.
    /// The collider is initialized with a sphere shape and an offset of (0.0, 0.5, 0.0).
    pub fn with_position(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        position: Vector3,
        rotation: f32,
    ) -> Self {
        let mut collider = Collider::with_sphere(1.0);
        collider.offset = Vector3::new(0.0, 0.5, 0.0);

        Self {
            transform: Transform3D::new(position, rotation),
            jump_timer: 0.0,
            jump_start_y: 0.0,
            crab_animator: CrabAnimator::new(rl, thread, path),
            has_landed: false,
            collider: collider,
        }
    }

    //----------------------------------------------------------------
    //
    //                         Crab Behavior
    //
    //----------------------------------------------------------------

    /// Teleports the crab to the given transform.
    pub fn teleport(&mut self, transform: Transform3D) {
        self.transform = transform;
    }

    /// Calculates the next transform for the crab based on the camera and input.
    pub fn calculate_next_transform(
        &mut self,
        rl: &mut RaylibHandle,
        camera: &Camera3D,
        thread: &RaylibThread,
        is_grounded: bool,
        will_grounded: bool,
        sound_manager: &mut SoundManager,
    ) -> Transform3D {
        let mut transform = self.transform.clone();

        self.crab_animator.handle_animation(rl, thread);
        let dt = rl.get_frame_time();

        // XZ movement
        let mut camera_direction = camera.target - camera.position;
        camera_direction.y = 0.0;

        let right_direction = camera_direction.cross(Vector3::new(0.0, 1.0, 0.0));
        let input = self.get_input_direction(rl);

        let mut move_vec = (camera_direction * input.z) + (right_direction * input.x);
        move_vec = move_vec.normalize();

        if move_vec.length() > 0.0 {
            sound_manager.play_sound_effect(SoundEffect::Walking);
            transform.position += move_vec * CrabStats::CRAB_SPEED * dt;

            let angle_rad = move_vec.x.atan2(move_vec.z);
            transform.rotation = self.lerp_angle(transform.rotation, angle_rad.to_degrees(), 0.12);
        }

        // Y movement (jump mechanic)
        if rl.is_key_down(KeyboardKey::KEY_SPACE) && self.jump_timer <= 0.0 && is_grounded {
            sound_manager.play_sound_effect(SoundEffect::Jump);
            self.jump_timer = PI;
            self.jump_start_y = transform.position.y;
            self.has_landed = false;
            self.crab_animator.jump();
        }

        // Animation mechanic
        if self.jump_timer > 0.0 {
            self.jump_timer -= dt * CrabStats::JUMP_SPEED;
            let jump_displacement = self.jump_timer.max(0.0).sin() * CrabStats::JUMP_HIGH;
            transform.position.y = self.jump_start_y + jump_displacement;

            let is_descending = self.jump_timer < PI / 2.0;

            if is_descending && will_grounded && !self.has_landed {
                self.crab_animator.land();
                self.has_landed = true;
            }
        } else {
            transform.position.y -= CrabStats::GRAVITY * dt;

            if move_vec.length() > 0.0 {
                self.crab_animator
                    .change_animation(CrabAnimation::MoveFront);
            } else if !matches!(
                self.crab_animator.current,
                CrabAnimation::LandJump | CrabAnimation::Emote
            ) {
                self.crab_animator.change_animation(CrabAnimation::Idle);
            }
        }

        if self.crab_animator.current == CrabAnimation::Idle && rl.is_key_down(KeyboardKey::KEY_E) {
            sound_manager.play_sound_effect(SoundEffect::Boing);
            self.crab_animator.change_animation(CrabAnimation::Emote);
        }

        transform
    }

    /// Get crab effective position (position - model offset)
    pub fn effective_position(&self) -> Vector3 {
        self.transform.position - Vector3::new(0.0, CrabStats::MODEL_OFFSET, 0.0)
    }

    /// Draws the crab model using the given `RaylibMode3D` draw handle.
    pub fn draw(&self, d3d: &mut impl RaylibDraw3D) {
        d3d.draw_model_ex(
            &self.crab_animator.model,
            self.transform.position + Vector3::new(0.0, CrabStats::MODEL_OFFSET, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            self.transform.rotation,
            Vector3::new(1.0, 1.0, 1.0),
            Color::WHITE,
        );
    }

    //----------------------------------------------------------------
    //
    //                      Utility Functions
    //
    //----------------------------------------------------------------

    /// Returns the input direction based on the current keyboard state.
    fn get_input_direction(&self, rl: &RaylibHandle) -> Vector3 {
        let mut input_dir = Vector3::new(0.0, 0.0, 0.0);

        if rl.is_key_down(KeyboardKey::KEY_W) {
            input_dir.z += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            input_dir.z -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            input_dir.x += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            input_dir.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            input_dir.y += 1.0;
        }

        return input_dir;
    }

    /// Linearly interpolates between two angles.
    fn lerp_angle(&self, from: f32, to: f32, weight: f32) -> f32 {
        let mut diff = (to - from + 180.0) % 360.0 - 180.0;
        if diff < -180.0 {
            diff += 360.0;
        }
        from + diff * weight
    }
}
