use raylib::prelude::*;

mod crab;
mod crab_animator;
use crate::crab::Crab;
mod menu;
use menu::Menu;
use menu::MenuManager;


const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ruzzle")
        .build();

    // let mut current_menu = Menu::Title;
    let mut menu_manager = MenuManager::new();



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


    let mut frame_count = 0;
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        /*--UPDATE--*/
        //Updating the game 
        //Updating the game 
        menu_manager.update(&rl);
        
        
        crab.update_with_camera(&mut rl, &camera, &thread);
        //Drawing
        let mut d = rl.begin_drawing(&thread);
        menu_manager.draw(&mut d, &crab, &camera);
      
    }
}
