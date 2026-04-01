use crate::components::collider::CollisionShape;
use crate::components::map::Map;
use crate::config::Config;
use crate::crab::crab::Crab;
use crate::menu::screens::draw_finish;
use raylib::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::menu::utils::draw_texture_contain;

use crate::menu::screens::{
    draw_credit, draw_game, draw_level_selection, draw_loading, draw_multiplayer, draw_select,
    draw_settings, draw_title, draw_win,
};

use crate::levels::level::Level;
// use crate::shader::daylight::DayCycleManager;
// use crate::shader::shader::ShaderManager;

use crate::sound_manager::sound_manager::{SoundEffect, SoundManager};

use crate::Transform3D;

pub struct Assets {
    pub textures: Vec<Texture2D>,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            textures: vec![
                rl.load_texture(thread, "/rsc/images/sand.png")
                    .expect("sand"),
                rl.load_texture(thread, "/rsc/images/bg_loading.png")
                    .expect("bg_loading"),
                rl.load_texture(thread, "/rsc/images/bg_logo.png")
                    .expect("bg_logo"),
                rl.load_texture(thread, "/rsc/images/back_button.png")
                    .expect("bg_logo"),
                rl.load_texture(thread, "/rsc/images/background.png")
                    .expect("background"),
            ],
        }
    }
}

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
    Win,
    Finish,
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
pub struct MenuManager<'a> {
    pub current_menu: Menu,
    pub frame_count: i32,
    pub buttons: Vec<Button>,
    pub level_buttons: Vec<Button>,
    pub back_button: Button,
    pub config: &'a Config,
    pub hovered_button: SelectMenuHoveredButtons,
    pub current_level: Option<Level>,
    pub assets: Assets,
    pub win_buttons: Vec<Button>,
    pub current_level_index: i8,
    pub render_target: RenderTexture2D,
    pub sound_manager: SoundManager<'a>,
}

impl<'a> MenuManager<'a> {
    pub fn new(
        config: &'a Config,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        sound_manager: SoundManager<'a>,
    ) -> Self {
        let button_width = (config.screen_width as f32) * 0.2;
        let button_height = (config.screen_height as f32) * 0.1;
        let assets = Assets::new(rl, thread);

        //let egg_menu = rl.load_texture(thread, "assets/egg.png").ok();
        let level_buttons = vec![
            Self::create_level_button(&config, "Niveau 1", 0, button_width, button_height),
            Self::create_level_button(&config, "Niveau 2", 1, button_width, button_height),
            Self::create_level_button(&config, "Niveau 3", 2, button_width, button_height),
            Self::create_level_button(&config, "Niveau 4", 3, button_width, button_height),
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
        let win_buttons = vec![
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.52,
                    button_width,
                    button_height,
                ),
                label: "Suivant".to_string(),
                id: SelectMenuHoveredButtons::LevelSelection,
            },
            Button {
                rectangle: Rectangle::new(
                    (config.screen_width / 2 - button_width as i32 / 2) as f32,
                    (config.screen_height as f32) * 0.67,
                    button_width,
                    button_height,
                ),
                label: "Menu".to_string(),
                id: SelectMenuHoveredButtons::Game,
            },
        ];

        // let shader_manager = ShaderManager::new(rl, thread);
        // let day_cycle = DayCycleManager::new();
        let render_target = rl
            .load_render_texture(
                thread,
                config.screen_width as u32,
                config.screen_height as u32,
            )
            .expect("Failed to load render texture");

        MenuManager {
            current_menu: Menu::Title,
            frame_count: 0,
            buttons: my_buttons_select,
            level_buttons,
            back_button,
            config,
            hovered_button: SelectMenuHoveredButtons::None,
            current_level: None,
            assets,
            win_buttons,
            current_level_index: 1,
            render_target,
            sound_manager,
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

        // Update shader uniforms and day cycle
        // self.shader_manager
        //     .set_sunlight_color(self.day_cycle.get_light_color());
        // self.shader_manager
        //     .set_ambient_color(self.day_cycle.get_ambient_color());
        // self.shader_manager.update_background_colors(
        //     self.day_cycle.get_background_top(),
        //     self.day_cycle.get_background_bottom(),
        // );
        // self.shader_manager.update_postprocess_resolution(
        //     self.config.screen_width as f32,
        //     self.config.screen_height as f32,
        // );

        match self.current_menu {
            Menu::Title => self.update_title(rl),
            Menu::Select => self.update_select(rl, thread, crab),
            Menu::LevelSelection => self.update_level_selection(rl, thread, crab),
            Menu::Game => self.update_game(rl, thread, map, crab, camera),
            Menu::Settings => self.update_settings(rl),
            Menu::Multiplayer => self.update_multiplayer(rl),
            Menu::Loading => self.update_loading(rl),
            Menu::Credit => self.update_credit(rl),
            Menu::Win => self.update_win(rl, thread, crab),
            Menu::Finish => self.update_finish(rl),
        }
    }

    /// Update fonctions
    fn update_title(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            // self.sound_manager.play_sound_effect(SoundEffect::Click);
            self.current_menu = Menu::Loading;
            self.frame_count = 0;
        }
    }

    fn update_win(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, crab: &mut Crab) {
        let mouse_pos = rl.get_mouse_position();
        for button in &self.win_buttons {
            if button.rectangle.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    match button.id {
                        SelectMenuHoveredButtons::LevelSelection => {
                            let next_index = self.current_level_index + 1;
                            let next_level = Level::new(next_index, rl, thread);

                            if !next_level.groups.is_empty() {
                                self.current_level_index = next_index;
                                let spawn = next_level.spawnpoint;
                                let mut next_level = next_level;
                                // self.apply_shader_to_level(&mut next_level);
                                self.current_level = Some(next_level);
                                crab.teleport(Transform3D {
                                    position: spawn,
                                    rotation: 0.0,
                                });
                                self.current_menu = Menu::Game;
                            } else {
                                self.current_menu = Menu::Finish;
                            }
                        }
                        SelectMenuHoveredButtons::Game => {
                            self.current_menu = Menu::Select;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn update_finish(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.current_menu = Menu::Select;
        }
    }

    /// update_selects allows to check if mouse is hovering a button,
    ///  and if it's the case, to update the hovered butto state for the menu manager,
    ///  and if the mouse is clicked, to change the menu accordingly to the button
    /// # Arguments
    /// * rl - raylib handler, handle the raylib librairieupdate_sele
    /// #TODO : make it more abstract to be able to use it for the settings menu and other menu with buttons
    fn update_select(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, crab: &mut Crab) {
        let mouse_pos = rl.get_mouse_position();
        self.hovered_button = SelectMenuHoveredButtons::None;
        for button in &self.buttons {
            if button.rectangle.check_collision_point_rec(mouse_pos) {
                self.hovered_button = button.id;
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    // self.sound_manager.play_sound_effect(SoundEffect::Click);
                    match button.id {
                        SelectMenuHoveredButtons::Game => {
                            let mut level = Level::new((1) as i8, rl, thread);
                            // self.apply_shader_to_level(&mut level);
                            self.current_level = Some(level);
                            self.current_level_index = (1) as i8;
                            self.current_menu = Menu::Game;
                            if let Some(level) = self.current_level.as_ref() {
                                let new_transform = Transform3D {
                                    position: level.spawnpoint,
                                    rotation: 0.0,
                                };
                                crab.teleport(new_transform);
                            }
                        }
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
            // self.sound_manager.play_sound_effect(SoundEffect::Click);
            self.current_menu = Menu::Title;
        }

        if let Some(level) = &mut self.current_level {
            level.update(rl);

            let is_grounded = level.is_grounded(&crab.collider, crab.effective_position());
            let will_grounded = level.is_grounded(
                &crab.collider,
                crab.effective_position() - Vector3::new(0.0, 0.4, 0.0),
            );

            let mut t =
                crab.calculate_next_transform(rl, &camera, &thread, is_grounded, will_grounded);

            t.position = level.resolve_collisions(&crab.collider, t.position);

            t.position = level.handle_out_of_map(t.position);
            crab.teleport(t);

            let radius = match crab.collider.shape {
                CollisionShape::Sphere { radius } => radius,
                _ => 1.0,
            };
            if level.is_at_endpoint(crab.transform.position, radius) {
                self.current_menu = Menu::Win;
                self.current_level = None;
            }
        }
    }

    fn update_level_selection(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        crab: &mut Crab,
    ) {
        self.handle_back_button(rl);

        let mouse_pos = rl.get_mouse_position();
        for (i, button) in self.level_buttons.iter().enumerate() {
            if button.rectangle.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let mut level = Level::new((i + 1) as i8, rl, thread);
                    self.current_level = Some(level);
                    self.current_level_index = (i + 1) as i8;
                    self.current_menu = Menu::Game;
                    if let Some(level) = self.current_level.as_ref() {
                        let new_transform = Transform3D {
                            position: level.spawnpoint,
                            rotation: 0.0,
                        };
                        crab.teleport(new_transform);
                    }
                }
            }
        }
    }

    fn update_multiplayer(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            // self.sound_manager.play_sound_effect(SoundEffect::Click);
            self.current_menu = Menu::Title;
        }
    }

    fn update_settings(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            // self.sound_manager.play_sound_effect(SoundEffect::Click);
            self.current_menu = Menu::Title;
        }
    }

    fn update_loading(&mut self, _rl: &RaylibHandle) {
        if self.frame_count >= 100 {
            self.current_menu = Menu::Select;
            self.frame_count = 0;
        }
    }

    fn update_credit(&mut self, rl: &RaylibHandle) {
        self.handle_back_button(rl);
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            // self.sound_manager.play_sound_effect(SoundEffect::Click);
            self.current_menu = Menu::Title;
        }
    }

    /// Returns true if the game is currently in the Game state
    pub fn is_in_game(&self) -> bool {
        matches!(self.current_menu, Menu::Game)
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
                // self.sound_manager.play_sound_effect(SoundEffect::Click);
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
    pub fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        map: &Map,
        crab: &mut Crab,
        camera: &Camera3D,
    ) {
        if let Some(text) = &self.assets.textures.get(4) {
            draw_texture_contain(d, text, self.config.screen_width, self.config.screen_height);
        } else {
            d.clear_background(Color::BLACK);
        }
        match self.current_menu {
            Menu::Game => {
                if let Some(level) = &mut self.current_level {
                    // Draw 3D scene into RenderTexture
                    {
                        let mut td = d.begin_texture_mode(thread, &mut self.render_target);
                        td.clear_background(Color::new(0, 0, 0, 0));

                        draw_game(
                            &mut td,
                            &self.config,
                            crab,
                            map,
                            camera,
                            level,
                            &self.assets,
                            self.current_level_index,
                        );
                    }

                    // Draw the RenderTexture to screen with post-process shader
                    {
                        let menu_flag = 0i32;
                        d.draw_texture_rec(
                            self.render_target.texture(),
                            Rectangle::new(
                                0.0,
                                0.0,
                                self.render_target.texture().width as f32,
                                -(self.render_target.texture().height as f32),
                            ),
                            Vector2::new(0.0, 0.0),
                            Color::WHITE,
                        );
                    }
                } else {
                    d.draw_text("Erreur : Aucun niveau chargé", 10, 10, 20, Color::RED);
                }
            }
            _ => {
                if let Some(text) = &self.assets.textures.get(4) {
                    {
                        let mut td = d.begin_texture_mode(thread, &mut self.render_target);
                        td.clear_background(Color::BLACK);
                        draw_texture_contain(
                            &mut td,
                            text,
                            self.config.screen_width,
                            self.config.screen_height,
                        );
                    }

                    {
                        let menu_flag = 1i32;
                        // self.shader_manager.postprocess_shader.set_shader_value(
                        //     self.shader_manager
                        //         .postprocess_shader
                        //         .get_shader_location("isMenu"),
                        //     menu_flag,
                        // );

                        // let mut sd =
                        //     d.begin_shader_mode(&mut self.shader_manager.postprocess_shader);
                        // sd.draw_texture_rec(
                        //     self.render_target.texture(),
                        //     Rectangle::new(
                        //         0.0,
                        //         0.0,
                        //         self.render_target.texture().width as f32,
                        //         -(self.render_target.texture().height as f32),
                        //     ),
                        //     Vector2::new(0.0, 0.0),
                        //     Color::WHITE,
                        // );
                    }
                } else {
                    d.clear_background(Color::BLACK);
                }

                match self.current_menu {
                    Menu::Title => draw_title(d, &self.config),
                    Menu::Select => draw_select(d, &self.config, &self.buttons),
                    Menu::LevelSelection => draw_level_selection(
                        d,
                        &self.config,
                        &self.level_buttons,
                        &self.assets,
                        &self.back_button,
                    ),
                    Menu::Multiplayer => draw_multiplayer(
                        d,
                        &self.config,
                        &self.back_button,
                        self.assets.textures.get(3),
                    ),
                    Menu::Settings => draw_settings(
                        d,
                        &self.config,
                        &self.back_button,
                        self.assets.textures.get(3),
                    ),
                    Menu::Loading => draw_loading(d, &self.config, self.assets.textures.get(2)),
                    Menu::Credit => draw_credit(
                        d,
                        &self.config,
                        &self.back_button,
                        self.assets.textures.get(3),
                    ),
                    Menu::Win => draw_win(d, &self.config, &self.win_buttons),
                    Menu::Finish => draw_finish(d, &self.config),
                    _ => {}
                }
            }
        }
    }

    // /// Applies the cel-shade shader to all models contained within a level.
    // fn apply_shader_to_level(&self, level: &mut Level) {
    //     for group in level.groups.iter_mut() {
    //         if let Some(model) = &mut group.model {
    //             self.shader_manager.apply_cel_shade_to_model(model);
    //         }
    //     }
    // }
}
