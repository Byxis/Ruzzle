use raylib::ffi::{CSSPalette, GetScreenHeight, RaylibPalette, ToggleFullscreen};
use raylib::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::config::Config;
use crate::crab::Crab;

/// Enum for the differents states displayed currently by the application
pub enum Menu {
    Title,
    Select,
    LevelSelection,
    Settings,
    Game,
    Loading,
    Credit,
}

#[derive(Copy, Clone, PartialEq)]
/// Enum for the differents buttons in the select menu, allowing to know what's hovered by the mouse.
pub enum SelectMenuHoveredButtons {
    None,
    Game,
    LevelSelection,
    Settings,
    Credit,
}

/// A Button is a Rectangle combined with the text displayed in it and the enum stating if
/// it's hovered currently or not.
/// It's purpose is to clicked on.
///
/// # Arguments
/// * rectanggle
/// * label as String
/// * id as SelectMenuHoveredButtons #FIXME : i need to find a way to be more abstract on the button
///
///
/// #Examples :
///
/// Create a button displaying hello
/// let rec = Rectangle::new((config.screen_width / 2 - button_width as i32 / 2) as f32, (config.screen_height as f32) * 0.3,  button_width,button_height)
/// let Button = Button::new(rec, "Hello guys".to_string(), SelectMenuHoveredButtons::None)
///
pub struct Button {
    pub rectangle: Rectangle,
    pub label: String,
    pub id: SelectMenuHoveredButtons,
}

impl Button {
    pub fn new(rectangle: Rectangle, label: String, id: SelectMenuHoveredButtons) -> Self {
        Button {
            rectangle: rectangle,
            label: label,
            id: id,
        }
    }
}

/// The Game Manager is designed to display the current state of the game.
/// It takes into arugment only the Config enum, and is in called in the main application.
/// It has the current_menu, and enum with the different menu possible, the frame_count used for artificial loading time,
/// the buttons which is a vec of buttons for the select menu, the button for the fullscreen, the config taken as an argument to know
/// how to display things accordingly to the screen size, and the hovered button to know which one is  an hovered button.
///
/// It is used with two functions :
/// * update to affect game logic and variables
/// * draw to draw the graphical elements
pub struct MenuManager {
    pub current_menu: Menu,
    pub frame_count: i32,
    pub buttons: Vec<Button>,
    pub back_button: Button,
    pub config: Config,
    pub hovered_button: SelectMenuHoveredButtons,
    pub bg_loading: Option<Texture2D>,
}

impl MenuManager {
    pub fn new(config: Config, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let button_width = (config.screen_width as f32) * 0.2;
        let button_height = (config.screen_height as f32) * 0.1;

        let bg_loading = rl.load_texture(thread, "assets/bg_loading.png").ok();

        let my_buttons_select = vec![
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.25,
                    button_width,
                    button_height,
                ),
                label: "Jouer".to_string(),
                id: SelectMenuHoveredButtons::Game,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.40,
                    button_width,
                    button_height,
                ),
                label: "Niveaux".to_string(),
                id: SelectMenuHoveredButtons::LevelSelection,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.55,
                    button_width,
                    button_height,
                ),
                label: "Options".to_string(),
                id: SelectMenuHoveredButtons::Settings,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.70,
                    button_width,
                    button_height,
                ),
                label: "Crédits".to_string(),
                id: SelectMenuHoveredButtons::Credit,
            },
        ];
        let back_button = Button {
            rectangle: Rectangle::new(
                (config.screen_width - (config.screen_width as f32 * 0.1) as i32) as f32,
                0.0,
                config.screen_width as f32 * 0.1,
                config.screen_width as f32 * 0.1,
            ),
            label: "Retour".to_string(),
            id: SelectMenuHoveredButtons::None,
        };

        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            buttons: my_buttons_select,
            back_button: back_button,
            config,
            hovered_button: SelectMenuHoveredButtons::None,
            bg_loading,
        }
    }

    /// Update the current state of the game, and the state variables
    /// Borrow Raylibhandle Pointer
    ///  #Arguments
    ///  * rl - raylib handler, handle the raylib librairie
    pub fn update(&mut self, rl: &RaylibHandle) {
        self.frame_count += 1;
        match self.current_menu {
            Menu::Title => self.update_title(rl),
            Menu::Select => self.update_select(rl),

            Menu::LevelSelection => self.update_game(rl),
            Menu::Game => self.update_game(rl),
            Menu::Settings => self.update_settings(rl),
            Menu::Loading => self.update_loading(rl),
            Menu::Credit => self.update_credit(rl),
        }
    }

    /// Update fonctions
    fn update_title(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.current_menu = Menu::Loading;
        }
    }
    /// update_selects allows to check if mouse is hovering a button,
    ///  and if it's the case, to update the hovered butto state for the menu manager,
    ///  and if the mouse is clicked, to change the menu accordingly to the button
    /// # Arguments
    /// * rl - raylib handler, handle the raylib librairie
    /// #TODO : make it more abstract to be able to use it for the settings menu and other menu with buttons
    fn update_select(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        self.hovered_button = SelectMenuHoveredButtons::None;
        for button in &self.buttons {
            if button.rectangle.check_collision_point_rec(mouse_pos) {
                self.hovered_button = button.id;
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    match button.id {
                        SelectMenuHoveredButtons::Game => self.current_menu = Menu::Game,
                        SelectMenuHoveredButtons::LevelSelection => self.current_menu = Menu::LevelSelection,
                        SelectMenuHoveredButtons::Settings => self.current_menu = Menu::Settings,
                        SelectMenuHoveredButtons::Credit => self.current_menu = Menu::Credit,
                        SelectMenuHoveredButtons::None => {}
                    }
                }
            }
        }
    }
    fn update_level_selection(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }
    fn update_game(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    fn update_settings(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        if self
            .back_button
            .rectangle
            .check_collision_point_rec(mouse_pos)
        {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.current_menu = Menu::Select;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    fn update_loading(&mut self, rl: &RaylibHandle) {
        if self.frame_count % 100 == 0 {
            self.current_menu = Menu::Select;
            self.frame_count = 0;
        }
    }

    fn update_credit(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        if self
            .back_button
            .rectangle
            .check_collision_point_rec(mouse_pos)
        {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.current_menu = Menu::Select;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    ///Draw the graphical elements
    ///
    /// # Arguments :
    /// * d : rayLIbDrawHandle, borrows it to draw graphical elemetns
    /// * c : crab : Crab (alexei's crabito)
    /// * camera : Camera3D (not used for now)
    pub fn draw(&self, mut d: &mut RaylibDrawHandle, crab: &Crab, camera: &Camera3D) {
        d.clear_background(Color::BLACK);

        match self.current_menu {
            Menu::Title => self.draw_title(d),
            Menu::Select => self.draw_select(d),
            Menu::LevelSelection => self.draw_level_selection(d),
            Menu::Settings => self.draw_settings(d),
            Menu::Game => self.draw_game(d, crab, camera),
            Menu::Loading => self.draw_loading(d),
            Menu::Credit => self.draw_credit(d),
        }
    }

    fn draw_title(&self, d: &mut RaylibDrawHandle) {
        let font_size_h1 = (self.config.screen_height / 14) as i32;

        draw_text_center(
            d,
            "Ruzzle",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2 - (self.config.screen_height / 10) as i32,
            font_size_h1,
            Color::WHITE,
        );
        draw_text_center(
            d,
            "Appuyez sur Entrée",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2,
            font_size_h1,
            Color::WHITE,
        );
    }

    fn draw_select(&self, d: &mut RaylibDrawHandle) {
        let font_size_h1 = (self.config.screen_height / 14) as i32;
        let font_size_h2 = (self.config.screen_height / 23) as i32;
        let color_hovered = Color::DARKORANGE;
        let color_button = Color::DARKGRAY;

        draw_text_center(
            d,
            "RUZZLE",
            self.config.screen_width,
            (self.config.screen_height / 10) as i32,
            font_size_h1,
            Color::WHITE,
        );

        for button in &self.buttons {
            let is_hovered = self.hovered_button == button.id;
            draw_button(
                d,
                button.rectangle,
                &button.label,
                color_hovered,
                color_button,
                is_hovered,
                font_size_h2,
            );
        }
    }

    fn draw_level_selection(&self, d: &mut RaylibDrawHandle) {
        let font_size_h2 = (self.config.screen_height / 23) as i32;

        draw_text_center(
            d,
            "Niveaux",
            self.config.screen_width,
            (self.config.screen_height / 7) as i32,
            font_size_h2,
            Color::WHITE,
        );

        draw_button(
            d,
            self.back_button.rectangle,
            &self.back_button.label,
            Color::DARKORANGE,
            Color::DARKGRAY,
            false,
            font_size_h2 / 3,
        );
    }

    fn draw_settings(&self, d: &mut RaylibDrawHandle) {
        let font_size_h2 = (self.config.screen_height / 23) as i32;

        draw_text_center(
            d,
            "Settings Menu",
            self.config.screen_width,
            (self.config.screen_height / 7) as i32,
            font_size_h2,
            Color::WHITE,
        );

        draw_button(
            d,
            self.back_button.rectangle,
            &self.back_button.label,
            Color::DARKORANGE,
            Color::DARKGRAY,
            false,
            font_size_h2 / 3,
        );
    }

    fn draw_game(&self, d: &mut RaylibDrawHandle, crab: &Crab, camera: &Camera3D) {
        {
            let mut d3d = d.begin_mode3D(camera);
            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
        }

        let coordonnees = format!(
            "({:.2}, {:.2}, {:.2})",
            crab.position.x, crab.position.y, crab.position.z
        );

        d.draw_text(
            &coordonnees,
            10,
            40,
            (self.config.screen_height / 36) as i32,
            Color::DARKGRAY,
        );
        d.draw_fps(10, 10);
    }

    fn draw_loading(&self, d: &mut RaylibDrawHandle) {
        let font_size_h1 = (self.config.screen_height / 14) as i32;

        if let Some(texture) = &self.bg_loading {
            let x = self.config.screen_width / 2 - texture.width / 2;
            let y = self.config.screen_height / 2 - texture.height / 2;
            d.draw_texture(texture, x, y, Color::WHITE);
        }
    }

    fn draw_credit(&self, d: &mut RaylibDrawHandle) {
        let font_size_h1 = (self.config.screen_height / 14) as i32;

        draw_text_center(
            d,
            "Jeu réalisé par :",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2 - (self.config.screen_height / 12) as i32,
            font_size_h1,
            Color::WHITE,
        );
        draw_text_center(
            d,
            "Alexey Serrané, Allessandraaaaaa, Carolayne, Max La Menax, André saitpascodé",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2,
            (self.config.screen_height / 36) as i32,
            Color::WHITE,
        );
        draw_text_center(
            d,
            "Max La Menax, André saitpascodé",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2 + (self.config.screen_height / 18) as i32,
            (self.config.screen_height / 12) as i32,
            Color::WHITE,
        );

        draw_button(
            d,
            self.back_button.rectangle,
            &self.back_button.label,
            Color::DARKORANGE,
            Color::DARKGRAY,
            false,
            self.config.screen_height / 30,
        );
    }
}

fn draw_button(
    d: &mut RaylibDrawHandle,
    button: Rectangle,
    text: &str,
    color_hovered: Color,
    color: Color,
    hovered: bool,
    font_size: i32,
) {
    //let text_play = "Jouer";
    let text_width_play = d.measure_text(text, font_size);
    if hovered {
        d.draw_rectangle_rec(button, color_hovered);
        d.draw_text(
            text,
            (button.x + (button.width - text_width_play as f32) / 1.8) as i32,
            (button.y + (button.height - (button.height / 2.0)) / 2.0) as i32,
            font_size,
            Color::WHITE,
        );
    } else {
        d.draw_rectangle_rec(button, color);
        d.draw_text(
            text,
            (button.x + (button.width - text_width_play as f32) / 2.0) as i32,
            (button.y + (button.height - (button.height / 2.0)) / 2.0) as i32,
            font_size,
            Color::BLACK,
        );
    }
}
///given a text, draws it centered based on the coordinate
/// * x :i32 x coordinates, if you want at the middle of the screen screen resolution /2 i   (can be anything but it will not be centered if not the =current screen resolution)
/// * y : i32 y coordinate
/// fontsize : i32 necessary to compute the center of the displayed text
///  color : Color of the text
fn draw_text_center(
    d: &mut RaylibDrawHandle,
    text: &str,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(text, (x / 2) - (text_length / 2), y, font_size, color);
}
