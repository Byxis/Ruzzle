pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,
    pub font_size_h1: i32,
    pub font_size_h2: i32,
}
/// Configuration files for the game
/// #TODO : add the sound level and music level
impl Config {
    pub fn new(screen_width : i32, screen_height : i32) -> Self {
        Config {
            screen_width,
            screen_height,
            font_size_h1: (screen_height / 14) as i32,
            font_size_h2: (screen_height / 23) as i32,
        }
    }
    pub fn update(&mut self, new_screen_width: i32, new_screen_height: i32) {
        self.screen_width = new_screen_width;
        self.screen_height = new_screen_height;
        self.font_size_h1 = (new_screen_height / 14) as i32;
        self.font_size_h2 = (new_screen_height / 23) as i32;
    }
}
