pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,
    pub sound_effects_volume: f32,
    pub music_volume: f32,
}
/// Configuration files for the game
/// #TODO : add the sound level and music level
impl Config {
    pub fn new() -> Self {
        Config {
            screen_width: 1280,
            screen_height: 720,
            sound_effects_volume: 0.5,
            music_volume: 0.5,
        }
    }
    pub fn update(&mut self, new_screen_width: i32, new_screen_height: i32) {
        self.screen_width = new_screen_width;
        self.screen_height = new_screen_height;
    }
}
