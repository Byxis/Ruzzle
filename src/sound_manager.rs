use raylib::prelude::*;

//sound files
const BOING: &str = "rsc/sounds/boing_effect.mp3";
const JUMP: &str = "rsc/sounds/jump_effect.mp3";
const WALKING: &str = "rsc/sounds/walking_effect.mp3";
const CLICK: &str = "rsc/sounds/click_effect.mp3";
const MUSIC: &str = "rsc/sounds/boing_effect.mp3"; //"rsc/music/ruzzle_music.mp3"
//for now replaced music with boing, music will be added when done

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
        let click_sound: Sound<'a> = audio.new_sound(CLICK)
            .expect("Failed to load click sound");

        SoundManager {
            background_music,
            walking_sound,
            jump_sound,
            boing_sound,
            click_sound,
            music_playing: false,
        }

        
    }

    //background music controls
    pub fn start_background_music(&mut self) {
        //START
        //launches background music if not already playing (only called once)
        if !self.music_playing {
            self.background_music.play_stream();
            self.music_playing = true;
        }
    }

    pub fn pause_background_music(&mut self) {
        //TOGGLE OFF
        //pauses background music (only if currently playing)
        if self.music_playing {
            self.background_music.pause_stream();
            self.music_playing = false;
        }
    }

    pub fn resume_background_music(&mut self) {
        //TOGGLE ON
        //resumes background music (only if currently paused)
        if !self.music_playing {
            self.background_music.resume_stream();
            self.music_playing = true;
        }
    }

    pub fn update_music_stream(&mut self) {
        //CONTINUOUS STREAM UPDATE
        //updates the music stream (called every frame)
        self.background_music.update_stream();
    }

    //sound effects
    //(to be added to actions that need a sound effect, directly add to action functions)
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

    pub fn play_click_sound(&mut self) {
        //plays click sound effect (for menu interactions)
        self.click_sound.play();
    }
}