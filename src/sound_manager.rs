use std::collections::HashMap;

use raylib::prelude::*;

//enum for sound files
#[derive(PartialEq, Eq, Hash, Clone, Copy)] //makes it usable in cache (as HashMap key)
pub enum SoundEffect {
    Boing,
    Jump,
    Walking,
    Click,
    Rotate,
}

impl SoundEffect {
    fn path(&self) -> &'static str {
        match self {
            Self::Boing => "rsc/sounds/boing_effect.mp3", //(to be decided)
            Self::Jump => "rsc/sounds/jump_effect.mp3", //(for crab jump)
            Self::Walking => "rsc/sounds/walking_effect.mp3", //(for crab movement)
            Self::Click => "rsc/sounds/click_effect.mp3", //(for menu interactions)
            Self::Rotate => "rsc/sounds/rotate_effect.mp3", //(for block rotation)
        }
    }
}

pub enum BackgroundMusic {
    CrabRave, //(default music)
    //TODO : other music choices ?
}

impl BackgroundMusic {
    fn path(&self) -> &'static str {
        match self {
            Self::CrabRave => "rsc/sounds/8bit_crab_rave.mp3",
        }
    }
}

//volume
const DEFAULT_MUSIC_VOLUME: f32 = 0.5; //default volume for music
const DEFAULT_EFFECT_VOLUME: f32 = 0.5; //default volume for sound effects

pub struct SoundManager<'a> {
    background_music: Option<Music<'a>>,
    music_volume: f32,
    pub music_playing: bool,
    effects_cache: HashMap<SoundEffect, Sound<'a>>, //cache for lazy loading and memory of sound effects
    effect_volume: f32,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {

        SoundManager {
            background_music: None, //init later (allows for future track choice)
            music_volume: DEFAULT_MUSIC_VOLUME,
            effects_cache: HashMap::new(), //initialize the effects cache
            effect_volume: DEFAULT_EFFECT_VOLUME, //volumes set to default
            music_playing: false,
        }
    }
    //volume control
    pub fn set_music_volume(&mut self, volume: f32) {
        //TODO : make a slider for this thing
        //sets the volume of the background music
        //f32 in [0.0 ; 1.0]
        if let Some(ref mut music) = self.background_music {
            music.set_volume(volume);
        }
        self.music_volume = volume;
    }

    pub fn set_effect_volume(&mut self, volume: f32) {
        //TODO : another slider for this one
        //sets the volume of all sound effects
        //f32 in [0.0 ; 1.0]
        self.effect_volume = volume;
    }

    pub fn set_default_volumes(&mut self) {
        //SET DEFAULT 
        //sets default volumes for music and sound effects (acts as reset)
        self.set_music_volume(DEFAULT_MUSIC_VOLUME);
        self.set_effect_volume(DEFAULT_EFFECT_VOLUME);
    }

    //background music controls
    pub fn set_background_music(&mut self, audio: &'a RaylibAudio, music: BackgroundMusic) {
        //SET BACKGROUND MUSIC
        //sets the background music to the specified track
        self.background_music = Some(audio.new_music(music.path()).expect("Failed to load background music"));
        self.background_music.as_mut().unwrap().set_volume(self.music_volume); //maintain current volume when changing music
        //background music as Option needs unwrapping 
    }
    pub fn start_background_music(&mut self) {
        //START
        //launches background music if not already playing (only called once)
        if !self.music_playing {
            self.background_music.as_mut().unwrap().play_stream();
            self.music_playing = true;
        }
    }

    pub fn pause_background_music(&mut self) {
        //TOGGLE OFF
        //pauses background music (only if currently playing)
        if self.music_playing {
            self.background_music.as_mut().unwrap().pause_stream();
            self.music_playing = false;
        }
    }

    pub fn resume_background_music(&mut self) {
        //TOGGLE ON
        //resumes background music (only if currently paused)
        if !self.music_playing {
            self.background_music.as_mut().unwrap().resume_stream();
            self.music_playing = true;
        }
    }

    pub fn update_music_stream(&mut self) {
        //CONTINUOUS STREAM UPDATE
        //updates the music stream (called every frame)
        if let Some(ref mut music) = self.background_music {
            music.update_stream();
        }
    }

    //sound effects
    //(to be added to actions that need a sound effect, directly add to action functions)
    fn load_sound(&mut self, audio: &'a RaylibAudio, effect: SoundEffect) {
        //LOAD SOUND EFFECT
        //loads the specified sound effect and returns it (to be played immediately after)
        let effect_path = effect.path();
        let new_sound: Sound<'a> = audio.new_sound(effect.path()).expect(&format!("Failed to load {effect_path}"));
        self.effects_cache.insert(effect, new_sound);
    }

    pub fn play_sound_effect(&mut self, audio: &'a RaylibAudio, effect: SoundEffect) {
        //PLAY SOUND EFFECT
        //plays the specified sound effect from the cache at the current effect volume
        //loads the sound effect if not already in the cache (lazy loading)
        if !self.effects_cache.contains_key(&effect) {
            SoundManager::load_sound(self, audio, effect);
        }
        //play sound effect from the cache
        let cached_effect = self.effects_cache.get(&effect).unwrap();
        cached_effect.set_volume(self.effect_volume);
        cached_effect.play();
    }

    

}