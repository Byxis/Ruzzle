use raylib::prelude::*;

mod crab;
mod crab_animator;
use crate::crab::Crab;
mod menu;
use menu::Menu;
use menu::MenuManager;


mod menu;
use menu::Menu;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ruzzle")
        .build();

    // let mut current_menu = Menu::Title;
    let mut menu_manager = MenuManager::new();

    // let button_width = 200.0;
    // let button_height = 60.0;
    // let game_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/ 2) as f32, 200.0, button_width, button_height);
    // let settings_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/2)  as f32, 300.0, button_width, button_height);
    // let credit_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/2) as f32, 400.0, button_width, button_height);
    

    let camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(0.0, 0.0, 0.5),
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
    let mut crab = Crab::new(
        &mut rl,
        &thread,
        "rsc/crab.glb",
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
    );

    let mut frame_count = 0;
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        /*--UPDATE--*/
        //Updating the game 
        menu_manager.update(&rl);
        
        crab.update_with_camera(&mut rl, &camera, &thread);

        crab.update_with_camera(&mut rl, &camera, &thread);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.clear_background(Color::BLACK);

        match current_menu{
            Menu::Title =>{
                d.draw_text("        Ruzzle      \n \n \nAppuyez sur Entrée", SCREEN_WIDTH/2 -160 , SCREEN_HEIGHT /2 -120, 40, Color::WHITE);
                
            }
            Menu::Select =>{
                d.draw_rectangle_rec(game_btn, Color::LIGHTGRAY);
                d.draw_text("Play", 120, 120, 30, Color::BLACK);

                d.draw_rectangle_rec(settings_btn, Color::LIGHTGRAY);
                d.draw_text("Settings", 120, 220, 30, Color::BLACK);

            }
            Menu::Settings => d.draw_text("Settings Menu", 100, 100, 40, Color::DARKGRAY),
            Menu::Game => {
                {
                    let mut d3d = d.begin_mode3D(camera);

            d3d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            d3d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
            d3d.draw_grid(10, 1.0);
        }

                let coordonnees = format!(
                    "({:.2}, {:.2}, {:.2})",
                    crab.position.x, crab.position.y, crab.position.z
                );

                d.draw_text(&coordonnees, 10, 40, 20, Color::DARKGRAY);
                d.draw_fps(10, 10);
            }

            Menu::Loading => {
                draw_text_center(
                    &mut d,
                    "Chargement...",
                    (SCREEN_HEIGHT as i32) / 2 - 60,
                    50,
                    Color::WHITE,
                );
            }
            Menu::Credit => {
                draw_text_center(
                    &mut d,
                    "Jeu réalisé par :",
                    (SCREEN_HEIGHT as i32) / 2 - 60,
                    50,
                    Color::WHITE,
                );
                draw_text_center(
                    &mut d,
                    "Alexey Serrané, Allessandraaaaaa, Carolayne, Max La Menax, André saitpascodé",
                    (SCREEN_HEIGHT as i32) / 2,
                    20,
                    Color::WHITE,
                );
                                draw_text_center(
                    &mut d,
                    "Max La Menax, André saitpascodé",
                    (SCREEN_HEIGHT as i32) / 2 +40,
                    20,
                    Color::WHITE,
                );
                
            }
        }
    }
}
