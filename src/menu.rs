use raylib::ffi::{CSSPalette, RaylibPalette};
use raylib::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::crab::Crab;

use crate::config::Config;
pub enum Menu {
    Title,
    Select,
    Settings,
    Game,
    Loading,
    Credit,
}
pub enum HoveredButton {
    None,
    Game,
    Settings,
    Credit,
}

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

pub struct MenuManager {
    pub current_menu: Menu,
    pub frame_count: i32,
    pub game_btn: Rectangle,
    pub settings_btn: Rectangle,
    pub credit_btn: Rectangle,
    pub config: Config,
    pub hovered_button: HoveredButton,
}

impl MenuManager {
    pub fn new(config: Config) -> Self {
        let button_width = 200.0;
        let button_height = 60.0;
        let game_btn = Rectangle::new(
            (SCREEN_WIDTH / 2 - button_width as i32 / 2) as f32,
            200.0,
            button_width,
            button_height,
        );
        let settings_btn = Rectangle::new(
            (SCREEN_WIDTH / 2 - button_width as i32 / 2) as f32,
            300.0,
            button_width,
            button_height,
        );
        let credit_btn = Rectangle::new(
            (SCREEN_WIDTH / 2 - button_width as i32 / 2) as f32,
            400.0,
            button_width,
            button_height,
        );

        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            game_btn,
            settings_btn,
            credit_btn,
            config,
            hovered_button: HoveredButton::None,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.frame_count += 1;
        match self.current_menu {
            Menu::Title => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.current_menu = Menu::Loading;
                }
            }
            Menu::Select => {
                let mouse_pos = rl.get_mouse_position();

                if self.game_btn.check_collision_point_rec(mouse_pos) {
                    self.hovered_button = HoveredButton::Game;
                } else if self.settings_btn.check_collision_point_rec(mouse_pos) {
                    self.hovered_button = HoveredButton::Settings;
                } else if self.credit_btn.check_collision_point_rec(mouse_pos) {
                    self.hovered_button = HoveredButton::Credit;
                } else {
                    self.hovered_button = HoveredButton::None;
                }
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    if self.settings_btn.check_collision_point_rec(mouse_pos) {
                        self.current_menu = Menu::Settings;
                    }
                    if self.game_btn.check_collision_point_rec(mouse_pos) {
                        self.current_menu = Menu::Game;
                    }
                    if self.credit_btn.check_collision_point_rec(mouse_pos) {
                        self.current_menu = Menu::Credit;
                    }
                }
            }
            Menu::Game => {
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    self.current_menu = Menu::Title;
                }
            }
            Menu::Settings => {
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

    pub fn draw(&self, mut d: &mut RaylibDrawHandle, crab: &Crab, camera: &Camera3D) {
        d.clear_background(Color::BLACK);

        let color_hovered = Color::RED;
        let color_button = Color::DARKGRAY;
        match self.current_menu {
            Menu::Title => {
                draw_text_center(
                    &mut d,
                    "Ruzzle",
                    (SCREEN_HEIGHT as i32) / 2 - 60,
                    50,
                    Color::WHITE,
                );
                draw_text_center(
                    &mut d,
                    "Appuyez sur Entrée",
                    (SCREEN_HEIGHT as i32) / 2,
                    50,
                    Color::WHITE,
                );
            }
            Menu::Select => {
                draw_text_center(&mut d, "RUZZLE", 30, 50, Color::WHITE);
                // Bouton Play
                match self.hovered_button {
                    HoveredButton::None => {
                        draw_button(d, self.game_btn, "Jouer", color_hovered, color_button, false);
                        draw_button(d, self.settings_btn, "Options", color_hovered, color_button, false);
                        draw_button(d, self.credit_btn, "Crédits", color_hovered, color_button, false);
                    }
                    HoveredButton::Game => {
                        draw_button(d, self.game_btn, "Jouer", color_hovered, color_button, true);
                        draw_button(d, self.settings_btn, "Options", color_hovered, color_button, false);
                        draw_button(d, self.credit_btn, "Crédits", color_hovered, color_button, false);
                    }
                    HoveredButton::Settings => {
                        draw_button(d, self.game_btn, "Jouer", color_hovered, color_button, false);
                        draw_button(d, self.settings_btn, "Options", color_hovered, color_button, true);
                        draw_button(d, self.credit_btn, "Crédits", color_hovered, color_button, false);
                    }
                    HoveredButton::Credit => {
                       draw_button(d, self.game_btn, "Jouer", color_hovered, color_button, false);
                        draw_button(d, self.settings_btn, "Options", color_hovered, color_button, false);
                        draw_button(d, self.credit_btn, "Crédits", color_hovered, color_button, true);
                    }
                }
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
                    (SCREEN_HEIGHT as i32) / 2 + 40,
                    20,
                    Color::WHITE,
                );
            }
        }
    }
}


fn draw_button(d: &mut RaylibDrawHandle, button : Rectangle, text : &str, color_hovered  : Color,color : Color, hovered : bool, ){

                        //let text_play = "Jouer";
                        let text_width_play = d.measure_text(text, 30);
                        if hovered{
                            d.draw_rectangle_rec(button, color_hovered);
                            d.draw_text(
                            text,
                            (button.x + (button.width- text_width_play as f32) / 2.0) as i32,
                            (button.y + (60.0 - 30.0) / 2.0) as i32,
                            30,
                            Color::WHITE,
                        );
                        }else{
                            d.draw_rectangle_rec(button, color);
                            d.draw_text(
                            text,
                            (button.x + (200.0 - text_width_play as f32) / 2.0) as i32,
                            (button.y + (60.0 - 30.0) / 2.0) as i32,
                            30,
                            Color::BLACK
                        );
                        }
                        
}
fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32) / 2 - (text_length / 2),
        y,
        font_size,
        color,
    );
}
