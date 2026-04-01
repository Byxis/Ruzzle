use raylib::prelude::*;
use std::f32::consts::PI;

/// Camera settings struct containing all tunable parameters.
pub struct CameraSettings;

impl CameraSettings {
    /// Pixels of mouse travel that equal one unit of pan in world-space.
    pub const PAN_SENSITIVITY: f32 = 0.0008;
    /// How many units the camera moves per trackpad scroll unit for horizontal pan.
    pub const TRACKPAD_PAN_SENSITIVITY: f32 = 8.0;
    /// How many units of distance change per scroll-wheel tick (positive = zoom in).
    pub const ZOOM_WHEEL_SPEED: f32 = 1.5;
    /// Zoom speed for trackpad pinch gestures (per frame while pinching).
    pub const ZOOM_PINCH_SPEED: f32 = 2.0;
    /// Radians per pixel while rotating with the middle mouse button.
    pub const ROTATION_SENSITIVITY: f32 = 0.002;
    /// Lowest allowed pitch angle above the horizon (radians). Prevents going under the ground.
    pub const PITCH_MIN: f32 = -PI / 2.0;
    /// Highest allowed pitch angle above the horizon (radians). Prevents going over the sky.
    pub const PITCH_MAX: f32 = PI / 2.0;
    /// Minimum allowed zoom distance (units). Prevents zooming out too far.
    pub const ZOOM_MIN: f32 = 2.0;
    /// Maximum allowed zoom distance (units). Prevents zooming in too far.
    pub const ZOOM_MAX: f32 = 80.0;
}

/// Camera controller that handles:
///
/// - **Pan** (right-mouse-button drag, or two-finger drag on a laptop trackpad):
///    Moves both the camera position and its look-at target by the same offset.
///
/// - **Zoom** (scroll wheel, or two-finger pinch on a trackpad):
///    Moves the camera along the vector between it and its target, keeping the
///    target fixed.
///
/// - **Rotation** (middle-mouse-button drag):
///    Rotates the camera around its target.
///    Horizontal drag = yaw (rotate left/right around world-up).
///    Vertical drag   = pitch (tilt up/down).
pub struct CameraController {
    /// Previous mouse position – used to compute per-frame deltas.
    prev_mouse: Vector2,
    /// Whether right-mouse was pressed in the previous frame (for first-frame
    /// delta suppression to avoid a sudden jump when the button is pressed).
    prev_rmb: bool,
    /// Whether middle-mouse was pressed in the previous frame.
    prev_mmb: bool,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            prev_mouse: Vector2::ZERO,
            prev_rmb: false,
            prev_mmb: false,
        }
    }

    /// Call once per frame, before drawing.
    pub fn update(&mut self, camera: &mut Camera3D, rl: &RaylibHandle) {
        let mouse = rl.get_mouse_position();

        let rmb = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);
        let mmb = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE);

        let delta = if (rmb && self.prev_rmb) || (mmb && self.prev_mmb) {
            mouse - self.prev_mouse
        } else {
            Vector2::ZERO
        };

        // Pan
        if rmb {
            self.apply_pan(camera, delta);
        }

        // Zoom
        self.handle_scroll_and_gesture(camera, rl);

        // Rotation
        if mmb {
            self.apply_rotation(camera, delta);
        }

        self.prev_mouse = mouse;
        self.prev_rmb = rmb;
        self.prev_mmb = mmb;
    }

    /// Translate both position and target by an offset in camera-space.
    /// `delta` is the 2-D mouse movement (pixels).
    fn apply_pan(&self, camera: &mut Camera3D, delta: Vector2) {
        if delta.x == 0.0 && delta.y == 0.0 {
            return;
        }

        let cam_dir = camera.target - camera.position;

        let forward_xz = {
            let v = Vector3::new(cam_dir.x, 0.0, cam_dir.z);
            if v.length() < 1e-3 {
                Vector3::new(0.0, 0.0, -1.0)
            } else {
                v.normalize()
            }
        };

        let right = forward_xz.cross(Vector3::new(0.0, 1.0, 0.0)).normalize();

        let distance = cam_dir.length().max(0.1);
        let scale = distance * CameraSettings::PAN_SENSITIVITY;

        let offset = right * (-delta.x * scale) + forward_xz * (delta.y * scale);

        camera.position += offset;
        camera.target += offset;
    }

    /// Move camera along the target→position direction.
    fn apply_zoom(&self, camera: &mut Camera3D, amount: f32) {
        let to_cam = camera.position - camera.target;
        let dist = to_cam.length();

        let new_dist = (dist - amount).clamp(CameraSettings::ZOOM_MIN, CameraSettings::ZOOM_MAX);
        camera.position = camera.target + to_cam.normalize() * new_dist;
    }

    /// Rotate the camera around its target.
    fn apply_rotation(&self, camera: &mut Camera3D, delta: Vector2) {
        if delta.x == 0.0 && delta.y == 0.0 {
            return;
        }

        let to_cam = camera.position - camera.target;
        let dist = to_cam.length();

        // Convert current position to spherical coordinates (relative to target)
        let yaw = to_cam.z.atan2(to_cam.x);
        let pitch = (to_cam.y / dist).asin();

        let new_yaw = yaw + delta.x * CameraSettings::ROTATION_SENSITIVITY;
        let new_pitch = (pitch + delta.y * CameraSettings::ROTATION_SENSITIVITY)
            .clamp(CameraSettings::PITCH_MIN, CameraSettings::PITCH_MAX);

        // Convert back to Cartesian
        camera.position = camera.target
            + Vector3::new(
                dist * new_pitch.cos() * new_yaw.cos(),
                dist * new_pitch.sin(),
                dist * new_pitch.cos() * new_yaw.sin(),
            );

        // Keep camera "up" always pointing toward world-up so the view
        camera.up = Vector3::new(0.0, 1.0, 0.0);
    }

    /// Handle scroll wheel events and trackpad gestures.
    ///
    /// Scroll X      ->  pan left/right (horizontal two-finger swipe; a physical
    ///                                      mouse wheel never produces X scroll)
    /// Scroll Y      ->  zoom in/out    (mouse wheel or vertical two-finger swipe)
    /// Pinch gesture ->  zoom           (touch screens; bonus on some trackpads)
    fn handle_scroll_and_gesture(&self, camera: &mut Camera3D, rl: &RaylibHandle) {
        let wheel = rl.get_mouse_wheel_move_v();

        // Horizontal scroll
        if wheel.x.abs() > 0.01 {
            let pan_delta = Vector2::new(wheel.x * CameraSettings::TRACKPAD_PAN_SENSITIVITY, 0.0);
            self.apply_pan(camera, pan_delta);
        }

        // Vertical scroll
        if wheel.y.abs() > 0.01 {
            self.apply_zoom(camera, wheel.y * CameraSettings::ZOOM_WHEEL_SPEED);
        }

        // Pinch gesture
        if rl.is_gesture_detected(Gesture::GESTURE_PINCH_IN) {
            self.apply_zoom(camera, CameraSettings::ZOOM_PINCH_SPEED);
        } else if rl.is_gesture_detected(Gesture::GESTURE_PINCH_OUT) {
            self.apply_zoom(camera, -CameraSettings::ZOOM_PINCH_SPEED);
        }
    }
}
