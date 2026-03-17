use crate::components::collider::Collider;
use crate::components::transform::Transform3D;
use crate::crab_animator::CrabAnimation;
use crate::crab_animator::CrabAnimator;
use raylib::prelude::*;

const CRAB_SPEED: f32 = 7.0;
const MODEL_OFFSET: f32 = 0.2;
const JUMP_HIGH: f32 = 2.0;
const JUMP_SPEED: f32 = 4.0;
pub struct Crab {
    pub transform: Transform3D,
    pub collider: Collider,
    jump_timer: f32,
    crab_animator: CrabAnimator,
}

impl Crab {
    //----------------------------------------------------------------
    //
    // Constructor
    //
    //----------------------------------------------------------------

    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        position: Vector3,
        rotation: f32,
    ) -> Self {
        let mut collider = Collider::new_sphere(1.0);
        collider.offset = Vector3::new(0.0, 0.5, 0.0);

        Self {
            transform: Transform3D::new(position, rotation),
            jump_timer: 0.0,
            crab_animator: CrabAnimator::new(rl, thread, path),
            collider: collider,
        }
    }

    //----------------------------------------------------------------
    //
    // Crab Behavior
    //
    //----------------------------------------------------------------

    pub fn teleport(&mut self, _transform: Transform3D) {
        self.transform = _transform;
    }

    pub fn calculate_next_transform(
        &mut self,
        rl: &mut RaylibHandle,
        camera: &Camera3D,
        thread: &RaylibThread,
    ) -> Transform3D {
        let mut transform3D = self.transform.clone();

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
            transform3D.position += move_vec * CRAB_SPEED * dt;

            let angle_rad = move_vec.x.atan2(move_vec.z);
            transform3D.rotation = lerp_angle(transform3D.rotation, angle_rad.to_degrees(), 0.12);
        }

        // Y movement (jump mechanic)
        if rl.is_key_down(KeyboardKey::KEY_SPACE) && self.jump_timer <= 0.0 {
            self.jump_timer = std::f32::consts::PI;
            self.crab_animator.jump();
        }

        // Animation mechanic
        if self.jump_timer > 0.0 {
            self.jump_timer -= dt * JUMP_SPEED;
            transform3D.position.y = self.jump_timer.max(0.0).sin() * JUMP_HIGH;

            if self.jump_timer <= 3.0 * dt * JUMP_SPEED {
                self.crab_animator.land();
            }
        } else {
            transform3D.position.y = 0.0;

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
            self.crab_animator.change_animation(CrabAnimation::Emote);
        }

        return transform3D;
    }

    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>) {
        d3d.draw_model_ex(
            &self.crab_animator.model,
            self.transform.position + Vector3::new(0.0, MODEL_OFFSET, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            self.transform.rotation,
            Vector3::new(1.0, 1.0, 1.0),
            Color::WHITE,
        );
    }

    //----------------------------------------------------------------
    //
    // Utility Functions
    //
    //----------------------------------------------------------------

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
}

fn lerp_angle(from: f32, to: f32, weight: f32) -> f32 {
    let mut diff = (to - from + 180.0) % 360.0 - 180.0;
    if diff < -180.0 {
        diff += 360.0;
    }
    from + diff * weight
}
