use crate::components::map::Map;
use crate::config::Config;
use crate::crab::crab::Crab;
use raylib::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::menu::utils::{
    draw_back_button, draw_button, draw_text_center, draw_texture_button, draw_texture_contain,
};

use crate::menu::screens::{draw_title, draw_select, draw_level_selection, draw_multiplayer, draw_credit};

/// Enum for the differents states displayed currently by the application
pub enum Menu {
    Title,
    Select,
    LevelSelection,
    Settings,
    Game,
    Multiplayer,
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
    Multiplayer,
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
            rectangle,
            label,
            id,
        }
    }
}

/// The Menu Manager is designed to display the current state of the game.
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
    pub level_buttons: Vec<Button>,
    pub back_button: Button,
    pub config: Config,
    pub hovered_button: SelectMenuHoveredButtons,
    pub bg_logo: Option<Texture2D>,
    pub tex_back: Option<Texture2D>,
    pub background_menu: Option<Texture2D>,
    //pub egg_menu: Option<Texture2D>,
}

impl MenuManager {
    pub fn new(config: Config, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let button_width = (config.screen_width as f32) * 0.2;
        let button_height = (config.screen_height as f32) * 0.1;

        let bg_logo = rl.load_texture(thread, "assets/bg_logo.png").ok();
        let tex_back = rl.load_texture(thread, "assets/back_button.png").ok();
        let background_menu = rl.load_texture(thread, "assets/background.png").ok();
        //let egg_menu = rl.load_texture(thread, "assets/egg.png").ok();
        let level_buttons = vec![
            Self::create_level_button(&config, "Niveau 1", 0, button_width, button_height),
            Self::create_level_button(&config, "Niveau 2", 1, button_width, button_height),
            Self::create_level_button(&config, "Niveau 3", 2, button_width, button_height),
            Self::create_level_button(&config, "Niveau 4", 3, button_width, button_height),
            Self::create_level_button(&config, "Niveau 5", 4, button_width, button_height),
        ];

        let my_buttons_select = vec![
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.20,
                    button_width,
                    button_height,
                ),
                label: "Jouer".to_string(),
                id: SelectMenuHoveredButtons::Game,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.35,
                    button_width,
                    button_height,
                ),
                label: "Niveaux".to_string(),
                id: SelectMenuHoveredButtons::LevelSelection,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.50,
                    button_width,
                    button_height,
                ),
                label: "Multijoueur".to_string(),
                id: SelectMenuHoveredButtons::Multiplayer,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.65,
                    button_width,
                    button_height,
                ),
                label: "Options".to_string(),
                id: SelectMenuHoveredButtons::Settings,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.80,
                    button_width,
                    button_height,
                ),
                label: "Crédits".to_string(),
                id: SelectMenuHoveredButtons::Credit,
            },
        ];
        let back_button = Button {
            rectangle: Rectangle::new(
                0.0,
                0.0,
                config.screen_width as f32 * 0.1,
                config.screen_height as f32 * 0.1,
            ),
            label: "Retour".to_string(),
            id: SelectMenuHoveredButtons::None,
        };

        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            buttons: my_buttons_select,
            level_buttons,
            back_button,
            config,
            hovered_button: SelectMenuHoveredButtons::None,
            bg_logo,
            tex_back,
            background_menu,
            //egg_menu,
        }
    }
    /// Helper to build level selection buttons with a simple vertical layout.
    ///
    /// # Arguments
    /// * config - used for positioning according to screen size
    /// * label - displayed text
    /// * index - vertical order of the button
    /// * width - button width in pixels
    /// * height - button height in pixels
    fn create_level_button(
        config: &Config,
        label: &str,
        index: usize,
        width: f32,
        height: f32,
    ) -> Button {
        let y_offset = (config.screen_height as f32) * 0.25 + (index as f32) * (height + 20.0);
        Button {
            rectangle: Rectangle::new(
                (config.screen_width / 2 - width as i32 / 2) as f32,
                y_offset,
                width,
                height,
            ),
            label: label.to_string(),
            id: SelectMenuHoveredButtons::None,
        }
    }
    /// Update the current state of the application, and the state variables.
    ///
    /// # Arguments
    /// * rl - raylib handler (inputs)
    /// * thread - raylib thread (passed to game update / crab)
    /// * map - used for collisions / grounded checks (game state)
    /// * crab - player entity updated in game state
    /// * camera - camera used for movement computations in game state
    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        map: &Map,
        crab: &mut Crab,
        camera: &Camera3D,
    ) {
        self.frame_count += 1;
        match self.current_menu {
            Menu::Title => self.update_title(rl),
            Menu::Select => self.update_select(rl),
            Menu::LevelSelection => self.update_level_selection(rl),
            Menu::Game => self.update_game(rl, thread, map, crab, camera),
            Menu::Settings => self.update_settings(rl),
            Menu::Multiplayer => self.update_multiplayer(rl),
            Menu::Loading => self.update_loading(rl),
            Menu::Credit => self.update_credit(rl),
        }
    }

    /// Update fonctions
    fn update_title(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.current_menu = Menu::Loading;
            self.frame_count = 0;
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
                        SelectMenuHoveredButtons::LevelSelection => {
                            self.current_menu = Menu::LevelSelection
                        }
                        SelectMenuHoveredButtons::Multiplayer => {
                            self.current_menu = Menu::Multiplayer
                        }
                        SelectMenuHoveredButtons::Settings => self.current_menu = Menu::Settings,
                        SelectMenuHoveredButtons::Credit => self.current_menu = Menu::Credit,
                        SelectMenuHoveredButtons::None => {}
                    }
                }
            }
        }
    }

    fn update_game(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        map: &Map,
        crab: &mut Crab,
        camera: &Camera3D,
    ) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }

        let is_grounded = map.is_grounded(&crab.collider, crab.effective_position());
        let will_grounded = map.is_grounded(
            &crab.collider,
            crab.effective_position() - Vector3::new(0.0, 0.4, 0.0),
        );

        let mut t = crab.calculate_next_transform(rl, &camera, &thread, is_grounded, will_grounded);

        t.position = map.resolve_collisions(&crab.collider, t.position);
        t.position = map.handle_out_of_map(t.position);
        crab.teleport(t);
    }

    fn update_level_selection(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    fn update_multiplayer(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    fn update_settings(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    fn update_loading(&mut self, rl: &RaylibHandle) {
        if self.frame_count >= 100 {
            self.current_menu = Menu::Select;
            self.frame_count = 0;
        }
    }

    fn update_credit(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.current_menu = Menu::Title;
        }
    }

    /// handle_back_button: common helper for menus that have a "Retour" button.
    ///
    /// # Arguments
    /// * rl - raylib handler (mouse position + click)

    fn handle_back_button(&mut self, rl: &RaylibHandle) {
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
    }

    ///Draw the graphical elements
    ///
    /// # Arguments :
    /// * d : rayLIbDrawHandle, borrows it to draw graphical elemetns
    /// * c : crab : Crab (alexei's crabito)
    /// * camera : Camera3D (not used for now)
    pub fn draw(&self, d: &mut RaylibDrawHandle, map: &Map, crab: &mut Crab, camera: &Camera3D) {
        if let Some(text) = &self.background_menu {
            draw_texture_contain(d, text, self.config.screen_width, self.config.screen_height);
        } else {
            d.clear_background(Color::BLACK);
        };

        match self.current_menu {
            Menu::Title => draw_title(d, &self.config),
            Menu::Select => draw_select(d, &self.config, &self.buttons),
            Menu::LevelSelection => draw_level_selection(d, &self.config, &self.level_buttons, &self.tex_back, &self.back_button),
            Menu::Multiplayer => draw_multiplayer(d, &self.config, &self.back_button, &self.tex_back),
            Menu::Settings => self.draw_settings(d),
            Menu::Game => self.draw_game(d, crab, map, camera),
            Menu::Loading => self.draw_loading(d),
            Menu::Credit => draw_credit(d, &self.config, &self.back_button, &self.tex_back),
        }
    }
    fn draw_settings(&self, d: &mut RaylibDrawHandle) {
        draw_text_center(
            d,
            "Settings Menu",
            self.config.screen_width,
            (self.config.screen_height / 7) as i32,
            self.config.font_size_h2,
            Color::WHITE,
        );
        let mouse = d.get_mouse_position(); // RaylibDrawHandle a accès à get_mouse_position
        let hovered = self.back_button.rectangle.check_collision_point_rec(mouse);
        if let Some(tex) = &self.tex_back {
            draw_texture_button(d, tex, self.back_button.rectangle, hovered);
        } else {
            // fallback si la texture n'a pas chargé
            draw_button(
                d,
                self.back_button.rectangle,
                &self.back_button.label,
                Color::DARKORANGE,
                Color::DARKGRAY,
                hovered,
                self.config.font_size_h2 / 3,
            );
        }
    }

    fn draw_game(&self, d: &mut RaylibDrawHandle, crab: &mut Crab, map: &Map, camera: &Camera3D) {
        {
            d.clear_background(Color::BLACK);
            let mut d3d = d.begin_mode3D(camera);
            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
            map.draw(&mut d3d);
        }

        let coordonnees = format!(
            "({:.2}, {:.2}, {:.2})",
            crab.transform.position.x, crab.transform.position.y, crab.transform.position.z
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
        if let Some(texture) = &self.background_menu {
            let x = 0;
            let y = 0;
            d.draw_texture(texture, x, y, Color::WHITE);
        }
        if let Some(texture) = &self.bg_logo {
            let x = self.config.screen_width / 2 - texture.width / 2;
            let y = self.config.screen_height / 2 - texture.height / 2;
            d.draw_texture(texture, x, y, Color::WHITE);
        }
    }

    fn draw_credit(&self, d: &mut RaylibDrawHandle) {
        draw_text_center(
            d,
            "Jeu réalisé par :",
            self.config.screen_width,
            (self.config.screen_height as i32) / 2 - (self.config.screen_height / 12) as i32,
            self.config.font_size_h1,
            Color::WHITE,
        );
        draw_text_center(
            d,
            "Alexey Serrané, Allessandraaaaaa, Carolayne",
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

        draw_back_button(
            d,
            self.back_button.rectangle,
            &self.tex_back,
            &self.back_button.label,
            self.config.font_size_h2 / 3,
        );
    }
}
