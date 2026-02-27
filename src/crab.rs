use raylib::prelude::*;

const CRAB_SPEED: f32 = 5.0;

pub struct Crab {
    pub model: Model,
    pub animations: Vec<ModelAnimation>,
    pub position: Vector3,
    pub rotation: f32,
    pub anim_frame_counter: i32,
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
            model: rl.load_model(thread, path).expect("Failed to load model"),
            animations: rl
                .load_model_animations(thread, path)
                .expect("Failed to load anims"),
            position,
            rotation,
            anim_frame_counter: 0,
        }
    }

    //----------------------------------------------------------------
    //
    // Crab Behavior
    //
    //----------------------------------------------------------------

    #[deprecated(note = "Use the new camera-relative update function instead")]
    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.handle_animation(rl, thread);
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
        self.handle_animation(rl, thread);

        let dt = rl.get_frame_time();

        let mut camera_direction = camera.target - camera.position;
        camera_direction.y = 0.0;

        let right_direction = camera_direction.cross(Vector3::new(0.0, 1.0, 0.0));
        let input = self.get_input_dir(rl);

        let mut move_vec = (camera_direction * input.z) + (right_direction * input.x);
        move_vec += input.y;
        move_vec = move_vec.normalize();

        if move_vec.length() > 0.0 {
            self.position += move_vec * CRAB_SPEED * dt;

            let angle_rad = move_vec.x.atan2(move_vec.z);
            self.rotation = lerp_angle(self.rotation, angle_rad.to_degrees(), 0.1);
        }
    }

    pub fn draw(&self, d3d: &mut RaylibMode3D<'_, impl RaylibDraw>) {
        d3d.draw_model_ex(
            &self.model,
            self.position,
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

    fn handle_animation(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.anim_frame_counter = (self.anim_frame_counter + 1) % self.animations[1].frameCount;

        rl.update_model_animation(
            thread,
            &mut self.model,
            &self.animations[1],
            self.anim_frame_counter,
        );
    }
}

fn lerp_angle(from: f32, to: f32, weight: f32) -> f32 {
    let mut diff = (to - from + 180.0) % 360.0 - 180.0;
    if diff < -180.0 {
        diff += 360.0;
    }
    from + diff * weight
}
