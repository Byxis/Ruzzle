use raylib::prelude::*;

// Fonction pour gérer les rotations de caméra
pub fn handle_camera(rl: &RaylibHandle, mut camera: &mut Camera3D) {

    let speed = 0.2;

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            camera.position.z -= speed;
            camera.target.z -= speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.position.z += speed;
            camera.target.z += speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.position.x -= speed;
            camera.target.x -= speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.position.x += speed;
            camera.target.x += speed;
        }
        // Monter/descendre
        if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            camera.position.y += speed;
            camera.target.y += speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            camera.position.y -= speed;
            camera.target.y -= speed;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            rl.update_camera_pro(&mut camera, Vector3::zero(), Vector3::new(0.0, 0.0, -45.0), 0.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_E) {
            rl.update_camera_pro(&mut camera, Vector3::zero(), Vector3::new(0.0, 0.0, 45.0), 0.0);
        }
}




pub fn display_coordinates(d: &mut RaylibDrawHandle, camera: &mut Camera3D, show_coords:bool)
{
    if show_coords {
        let pos = camera.position;
        let text = format!("Position: x={:.2} y={:.2} z={:.2}", pos.x, pos.y, pos.z);
        d.draw_text(&text, 10, 40, 20, Color::BLACK);
    }
}


// Fonction pour gérer le toggle des coordonnées
pub fn handle_coords_toggle(rl: &RaylibHandle, show_coords: &mut bool) {
    if rl.is_key_pressed(KeyboardKey::KEY_C) {
        *show_coords = !*show_coords;
    }
}