use raylib::prelude::*;

//files
const BOING: &str = "rsc/<sound_path>";
const JUMP: &str = "rsc/<sound_path>";
const WALKING: &str = "rsc/<sound_path>";
const MUSIC: &str = "rsc/<music_path>";

pub struct SoundManager<'a> {
    pub background_music: Music<'a>,
    pub walking_sound: Sound<'a>,
    pub jump_sound: Sound<'a>,
    pub boing_sound: Sound<'a>,
    pub music_playing: bool,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let background_music: Music<'a> = audio.new_music(MUSIC)
            .expect("Failed to load background music");
        let walking_sound: Sound<'a> = audio.new_sound(WALKING)
            .expect("Failed to load walking sound");
        let jump_sound: Sound<'a> = audio.new_sound(JUMP)
            .expect("Failed to load jump sound");
        let boing_sound: Sound<'a> = audio.new_sound(BOING)
            .expect("Failed to load boing sound");

        SoundManager {
            background_music,
            walking_sound,
            jump_sound,
            boing_sound,
            music_playing: false,
        }

        
    }
    pub fn play_background_music(&mut self) {
        //launches background music if not already playing
        if !self.music_playing {
            self.background_music.play_stream();
            self.music_playing = true;
        }
    }

    pub fn pause_background_music(&mut self) {
        //pauses background music (only if currently playing)
        if self.music_playing {
            self.background_music.pause_stream();
            self.music_playing = false;
        }
    }

    pub fn resume_background_music(&mut self) {
        //resumes background music (only if currently paused)
        if !self.music_playing {
            self.background_music.resume_stream();
            self.music_playing = true;
        }
    }

    pub fn play_walking_sound(&mut self) {
        //plays walking sound effect
        self.walking_sound.play();
    }

    pub fn play_jump_sound(&mut self) {
        //plays jump sound effect
        self.jump_sound.play();
    }

    pub fn play_boing_sound(&mut self) {
        //plays boing sound effect
        self.boing_sound.play();
    }
}