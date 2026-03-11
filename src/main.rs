use raylib::prelude::*;

mod crab;
mod crab_animator;
use crate::crab::Crab;


mod menu;
use menu::Menu;


const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ruzzle")
        .build();


    let mut current_menu = Menu::Title;
    let settings_btn = Rectangle::new(100.0, 200.0, 200.0, 60.0);
    let game_btn = Rectangle::new(100.0, 300.0, 200.0, 60.0);
    let button_width = 200.0;
    let button_height = 60.0;
    let spacing = 30.0;
    let num_buttons = 2;


    let camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(0.0, 0.0, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let mut crab = Crab::new(
        &mut rl,
        &thread,
        "rsc/crab.glb",
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
    );

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Handle menu transitions
         match current_menu {
            Menu::Title => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_menu = Menu::Select;
                }
            }

            Menu::Select => {
                let mouse_pos = rl.get_mouse_position();
                
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
                    if settings_btn.check_collision_point_rec(mouse_pos){
                        current_menu = Menu::Settings;
                    }
                     if game_btn.check_collision_point_rec(mouse_pos){
                        current_menu = Menu::Game;
                    }
                }
            }
            Menu::Game => {
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    current_menu = Menu::Title;
                }
            }
            Menu::Settings => {
                // Ajoute ici d'autres transitions
            }
            Menu::Loading => {
                // Ajoute ici d'autres transitions
            }
        }
        crab.update_with_camera(&mut rl, &camera, &thread);

        let mut d = rl.begin_drawing(&thread);




        d.clear_background(Color::BLACK);

        match current_menu{
            Menu::Title =>{
                d.draw_text("        Ruzzle      \n \n \nAppuyez sur Entrée", SCREEN_WIDTH/2 -160 , SCREEN_HEIGHT /2 -120, 40, Color::WHITE);
            }
            Menu::Select =>{
                // Bouton Play
                d.draw_rectangle_rec(game_btn, Color::LIGHTGRAY);
                let text_play = "Play";
                let text_width_play = d.measure_text(text_play, 30);
                d.draw_text(
                    text_play, 
                    (game_btn.x + (button_width - text_width_play as f32) / 2.0) as i32, 
                    (game_btn.y + (button_height - 30.0) / 2.0) as i32, 
                    30, 
                    Color::BLACK
                );

                // Bouton Settings
                d.draw_rectangle_rec(settings_btn, Color::LIGHTGRAY);
                let text_settings = "Settings";
                let text_width_settings = d.measure_text(text_settings, 30);
                d.draw_text(
                    text_settings, 
                    (settings_btn.x + (button_width - text_width_settings as f32) / 2.0) as i32, 
                    (settings_btn.y + (button_height - 30.0) / 2.0) as i32, 
                    30, 
                    Color::BLACK
                );
            }
            Menu::Settings => d.draw_text("Settings Menu", 100, 100, 40, Color::DARKGRAY),
            Menu::Game => {
                        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
        }

        let coordonnees = format!(
            "({:.2}, {:.2}, {:.2})",
            crab.position.x, crab.position.y, crab.position.z
        );


        
        d.draw_text(&coordonnees, 10, 40, 20, Color::DARKGRAY);
        d.draw_fps(10, 10);
    }

            Menu::Loading => d.draw_text("Loading Menu", 100, 100, 40, Color::BLUE),
        }

    }
}




fn draw_text_center(
    d: &mut RaylibDrawHandle,
    text: &str,
    y : i32,
    font_size : i32,
    color : Color
){
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32)/ 2 - (text_length /2),
        y,
        font_size,
        color
    );

}