use crate::crab_animator::CrabAnimation;
use crate::crab_animator::CrabAnimator;
use raylib::prelude::*;

const CRAB_SPEED: f32 = 0.7;
const MODEL_OFFSET: f32 = 0.2;
const JUMP_HIGH: f32 = 2.0;
const JUMP_SPEED: f32 = 4.0;

pub struct Crab {
    pub position: Vector3,
    pub rotation: f32,
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
        Self {
            position: position,
            rotation: rotation,
            jump_timer: 0.0,
            crab_animator: CrabAnimator::new(rl, thread, path),
        }
    }

    //----------------------------------------------------------------
    //
    // Crab Behavior
    //
    //----------------------------------------------------------------

    pub fn teleport(&mut self, rl: &mut RaylibHandle) {
        let dt = rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.position.x += CRAB_SPEED * dt;
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.position.x -= CRAB_SPEED * dt;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.position.z += CRAB_SPEED * dt;
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.position.z -= CRAB_SPEED * dt;
        }
    }

    pub fn update_with_camera(
        &mut self,
        rl: &mut RaylibHandle,
        camera: &Camera3D,
        thread: &RaylibThread,
    ) {
        self.crab_animator.handle_animation(rl, thread);
        let dt = rl.get_frame_time();

        // XZ movement
        let mut camera_direction = camera.target - camera.position;
        camera_direction.y = 0.0;

        let right_direction = camera_direction.cross(Vector3::new(0.0, 1.0, 0.0));
        let input = self.get_input_dir(rl);

        let mut move_vec = (camera_direction * input.z) + (right_direction * input.x);
        move_vec.normalize();

        if move_vec.length() > 0.0 {
            self.position += move_vec * CRAB_SPEED * dt;

            let angle_rad = move_vec.x.atan2(move_vec.z);
            self.rotation = lerp_angle(self.rotation, angle_rad.to_degrees(), 0.12);
        }

        // Y movement (jump mechanic)
        if rl.is_key_down(KeyboardKey::KEY_SPACE) && self.jump_timer <= 0.0 {
            self.jump_timer = std::f32::consts::PI;
            self.crab_animator.jump();
        }

        if self.jump_timer > 0.0 {
            self.jump_timer -= dt * JUMP_SPEED;
            self.position.y = self.jump_timer.max(0.0).sin() * JUMP_HIGH;

            if self.jump_timer <= 3.0 * dt * JUMP_SPEED {
                self.crab_animator.land();
            }
        } else {
            self.position.y = 0.0;

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
    }

    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>) {
        d3d.draw_model_ex(
            &self.crab_animator.model,
            self.position + Vector3::new(0.0, MODEL_OFFSET, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            self.rotation,
            Vector3::new(1.0, 1.0, 1.0),
            Color::WHITE,
        );
    }

    //----------------------------------------------------------------
    //
    // Utility Functions
    //
    //----------------------------------------------------------------

    fn get_input_dir(&self, rl: &RaylibHandle) -> Vector3 {
        let mut input_dir = Vector3::new(0.0, 0.0, 0.0);

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            input_dir.z += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            input_dir.z -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            input_dir.x += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
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
