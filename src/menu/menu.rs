

use raylib::ffi::{CSSPalette, GetScreenHeight, RaylibPalette, ToggleFullscreen};
use raylib::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::crab::Crab;

use crate::config::Config;

//// Enum for the differents states displayed currently by the application
pub enum Menu {
    Title,
    Select,
    Settings,
    Game,
    Loading,
    Credit,
}


#[derive(Copy, Clone, PartialEq)]
pub enum HoveredButton {
    None,
    Game,
    Settings,
    Credit,
}

//// Struct for the button.
pub struct Button {
    pub rectangle : Rectangle,
    pub label : String,
    pub id : HoveredButton,
}


pub struct MenuManager {
    pub current_menu: Menu,
    pub frame_count: i32,
    pub buttons : Vec<Button>,
    pub button_fullscreen : Button,
    pub config: Config,
    pub hovered_button: HoveredButton,
}


//// Menu Manager : creates a Menu
/// #Arguments : 
/// * Config (enum Config)
impl MenuManager {
    pub fn new(config: Config) -> Self {
        // let button_width = 200.0;
        // let button_height = 60.0;
        let button_width = (config.screen_width as f32)  * 0.2 ;
        let button_height = (config.screen_height as f32) * 0.1 ;
        
        let my_buttons_select = vec![
        Button{
            rectangle : Rectangle::new((config.screen_width / 2 - button_width as i32 / 2) as f32,
            (config.screen_height as f32) * 0.3,
            button_width,button_height),
            label : "Jouer".to_string(),
            id : HoveredButton::Game
        },
        Button{
            rectangle : Rectangle::new((config.screen_width / 2 - button_width as i32 / 2) as f32,
            (config.screen_height as f32) * 0.5,
            button_width,button_height),
            label : "Options".to_string(),
            id : HoveredButton::Settings
        },
        Button{
            rectangle : Rectangle::new((config.screen_width / 2 - button_width as i32 / 2) as f32,
            (config.screen_height as f32) * 0.7,
            button_width,button_height),
            label : "Crédits".to_string(),
            id : HoveredButton::Credit
        }
        ];
        let button_fullscreen  = Button{
            rectangle : Rectangle::new((config.screen_width - (config.screen_width as f32  * 0.1 ) as i32) as f32,
            0.0,
            config.screen_width as f32  * 0.1, config.screen_width as f32 / 10.0),
            label : "Plein écran".to_string(),
            id : HoveredButton::None
        };
        
        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            buttons : my_buttons_select,
            button_fullscreen : button_fullscreen,
            config,
            hovered_button: HoveredButton::None,
        }
    }
    
    
    
    
    pub fn update(&mut self, rl: &RaylibHandle) {
        // Update the current state of the game, and the state variables
        //Borrow Raylibhandle Pointer
        // #Arguments 
        // * rl - raylib handler, handle the raylib librairie
        self.frame_count += 1;
        match self.current_menu {
            Menu::Title => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.current_menu = Menu::Loading;
                }
            }
            Menu::Select => {
                let mouse_pos = rl.get_mouse_position();
                self.hovered_button = HoveredButton::None; 
                for button in & self.buttons{
                    if button.rectangle.check_collision_point_rec(mouse_pos){
                        self.hovered_button = button.id;
                        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                            match button.id {
                                HoveredButton::Game => self.current_menu = Menu::Game,
                                HoveredButton::Settings => self.current_menu = Menu::Settings,
                                HoveredButton::Credit => self.current_menu = Menu::Credit,
                                HoveredButton::None => {},
                            }
                        }
                    }
                }
            }
            
            
            Menu::Game => {
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    self.current_menu = Menu::Title;
                }
            }
            Menu::Settings => {
                let mouse_pos = rl.get_mouse_position();
                if self.button_fullscreen.rectangle.check_collision_point_rec(mouse_pos){
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
                        
                        //TODO : put an option to put in fullscreen but properly or somehting to resize the game 
                    }
                }
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    self.current_menu = Menu::Title;
                    
                }
            }
            Menu::Loading => {
                if self.frame_count % 100 == 0 {
                    self.current_menu = Menu::Select;
                    self.frame_count = 0;
                }
            }
            Menu::Credit => {
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    self.current_menu = Menu::Title;
                }
            }
        }
    }
    //
    pub fn draw(&self, mut d: &mut RaylibDrawHandle, crab: &Crab, camera: &Camera3D) {
        //Draw the graphical elements 
        //
        // # Arguments : 
        // * d : rayLIbDrawHandle, borrows it to draw graphical elemetns 
        // * c : crab : Crab (alexei's crabito) 
        // * camera : Camera3D (not used for now)
        d.clear_background(Color::BLACK);
        
        let color_hovered = Color::DARKORANGE;
        let color_button = Color::DARKGRAY;
        let font_size_h1 = (self.config.screen_height / 14) as i32;
        let font_size_h2 = (self.config.screen_height / 23) as i32;
        match self.current_menu {
            Menu::Title => {
                
                draw_text_center(
                    &mut d,
                    "Ruzzle",
                    self.config.screen_width,
                    (self.config.screen_height as i32) / 2 - (self.config.screen_height / 10) as i32,
                    font_size_h1 ,
                    Color::WHITE,
                );
                draw_text_center(
                    &mut d,
                    "Appuyez sur Entrée",
                    self.config.screen_width,
                    (self.config.screen_height as i32) / 2,
                    font_size_h1,
                    Color::WHITE,
                );
            }
            Menu::Select => {
                draw_text_center(&mut d, "RUZZLE",self.config.screen_width, (self.config.screen_height /10) as i32,
                font_size_h1, Color::WHITE);
                for button in &self.buttons{
                    let is_hovered = self.hovered_button == button.id;
                    draw_button(d, button.rectangle, &button.label, color_hovered, color_button, is_hovered, font_size_h2);
                }
                
            }
            Menu::Settings => {
                draw_text_center(d, "Settings Menu", self.config.screen_width,
                (self.config.screen_height / 7) as i32,
                font_size_h2, Color::WHITE);
                //TODO : put a full screen image or somehting to resize the game 
                draw_button(d, self.button_fullscreen.rectangle, 
                    &self.button_fullscreen.label, 
                    color_hovered, Color::RED, false, font_size_h2 /3);
                }
                
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
                    
                    d.draw_text(&coordonnees, 10, 40, (self.config.screen_height /36 as i32), Color::DARKGRAY);
                    d.draw_fps(10, 10);
                }
                
                Menu::Loading => {
                    draw_text_center(
                        &mut d,
                        "Chargement...",
                        self.config.screen_width,
                        (self.config.screen_height as i32) / 2 - (self.config.screen_height /12 as i32),
                        font_size_h1,
                        Color::WHITE,
                    );
                }
                Menu::Credit => {
                    draw_text_center(
                        &mut d,
                        "Jeu réalisé par :",
                        self.config.screen_width,
                        (self.config.screen_height as i32) / 2 - (self.config.screen_height /12 as i32),
                        font_size_h1,
                        Color::WHITE,
                    );
                    draw_text_center(
                        &mut d,
                        "Alexey Serrané, Allessandraaaaaa, Carolayne, Max La Menax, André saitpascodé",
                        self.config.screen_width,
                        (self.config.screen_height as i32) / 2,
                        (self.config.screen_height /36 as i32),
                        Color::WHITE,
                    );
                    draw_text_center(
                        &mut d,
                        "Max La Menax, André saitpascodé",
                        self.config.screen_width,
                        (self.config.screen_height as i32) / 2 + (self.config.screen_height /18 as i32),
                        (self.config.screen_height /12 as i32),
                        Color::WHITE,
                    );
                }
            }
        }
    }
    
    
    fn draw_button(d: &mut RaylibDrawHandle, button : Rectangle, text : &str, color_hovered  : Color, color : Color, hovered : bool, font_size: i32 ){
        
        //let text_play = "Jouer";
        let text_width_play = d.measure_text(text, font_size);
        if hovered{
            d.draw_rectangle_rec(button, color_hovered);
            d.draw_text(
                text,
                (button.x + (button.width- text_width_play as f32) / 1.8) as i32,
                (button.y + (button.height - (button.height /2.0 )) / 2.0) as i32,
                font_size,
                Color::WHITE,
            );
        }else{
            d.draw_rectangle_rec(button, color);
            d.draw_text(
                text,
                (button.x + (button.width- text_width_play as f32) / 2.0) as i32,
                (button.y + (button.height - (button.height /2.0 )) / 2.0) as i32,
                font_size,
                Color::BLACK
            );
        }
        
    }
    fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, x : i32,  y: i32, font_size: i32, color: Color) {
        //given a text, draws it centered based on the coordinate
        // * x :i32 x coordinates, if you want at the middle of the screen screen resolution /2 i   (can be anything but it will not be centered if not the =current screen resolution)
        // * y : i32 y coordinate
        // fontsize : i32 necessary to compute the center of the displayed text
        //  color : Color of the text 
        let text_length = d.measure_text(text, font_size);
        d.draw_text(
            text,
            (x / 2) - (text_length / 2),
            y,
            font_size,
            color,
        );
    }
    