use raylib::{ffi::KeyboardKey, RaylibHandle};
use raylib::prelude::*;

pub enum Menu {
    Title,
    Select,
    Settings,
    Game,
    Loading,
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
}

impl MenuManager {
    pub fn new() -> Self {

        let button_width = 200.0;
        let button_height = 60.0;
        let game_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/ 2) as f32, 200.0, button_width, button_height);
        let settings_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/2)  as f32, 300.0, button_width, button_height);
        let credit_btn = Rectangle::new((SCREEN_WIDTH / 2 - button_width as i32/2) as f32, 400.0, button_width, button_height);



        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            game_btn,
            settings_btn,
            credit_btn,
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
}
