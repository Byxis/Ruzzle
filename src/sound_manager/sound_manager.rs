use std::collections::HashMap;

use raylib::prelude::*;

//------------------------------------------------------------------------------
// SOUND MANAGER
// This module manages all sound-related functionalities, including background music and sound effects.
// It provides functions to set and control background music, as well as to load and play cached sound effects on demand.
//------------------------------------------------------------------------------

// SOUND EFFECTS AND BACKGROUND MUSIC ENUMS
// These enums define the available sound effects and background music tracks in the game, along with their file paths for loading.
// TODO : make it into a config file ?

// Sound effects enum with file paths
#[derive(PartialEq, Eq, Hash, Clone, Copy)] // Makes it usable in cache (as HashMap key)
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
            Self::Boing => "rsc/sounds/boing_effect.mp3", // To be decided
            Self::Jump => "rsc/sounds/jump_effect.mp3", // For crab jump
            Self::Walking => "rsc/sounds/walking_effect.mp3", // For crab movement
            Self::Click => "rsc/sounds/click_effect.mp3", // For menu interactions
            Self::Rotate => "rsc/sounds/rotate_effect.mp3", // For block rotation
            _ => "rsc/sounds/boing_effect.mp3", // Boing is default sound effect
        }
    }
}

// Background music enum with file paths
pub enum BackgroundMusic {
    CrabRave, // Default music
    // TODO : other music choices ?
}

impl BackgroundMusic {
    fn path(&self) -> &'static str {
        match self {
            Self::CrabRave => "rsc/sounds/8bit_crab_rave.mp3",
            _ => "rsc/sounds/8bit_crab_rave.mp3", // Crab rave is default music
            
        }
    }
}

// DEFAULT VOLUME SETTINGS
// TODO : also add it to config file ?
const DEFAULT_MUSIC_VOLUME: f32 = 0.5; // Default volume for music
const DEFAULT_EFFECT_VOLUME: f32 = 0.5; // Default volume for sound effects

// SOUND MANAGER STRUCT
// This struct encapsulates all sound-related data and functionalities, 
// Including the current background music, volume levels, and a cache for sound effects to optimize loading and playback.
pub struct SoundManager<'a> {
    background_music: Option<Music<'a>>,
    music_volume: f32,
    pub music_playing: bool,
    effects_cache: HashMap<SoundEffect, Sound<'a>>, // Cache for lazy loading and memory of sound effects
    effect_volume: f32,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        SoundManager {
            background_music: None, // Initialize later (allows for future track choice)
            music_volume: DEFAULT_MUSIC_VOLUME,
            effects_cache: HashMap::new(), // Initialize the effects cache
            effect_volume: DEFAULT_EFFECT_VOLUME, // Volumes set to default
            music_playing: false,
        }
    }

    //Volume controls

    // TODO : make a slider for this thing
    // Sets the volume of the background music
    // f32 in [0.0 ; 1.0]
    pub fn set_music_volume(&mut self, volume: f32) {
        if let Some(ref mut music) = self.background_music {
            music.set_volume(volume);
        }
        self.music_volume = volume;
    }

    // TODO : another slider for this one
    // Sets the volume of all sound effects
    // f32 in [0.0 ; 1.0]
    pub fn set_effect_volume(&mut self, volume: f32) {
        self.effect_volume = volume;
    }

    // SET DEFAULT 
    // Sets default volumes for music and sound effects (acts as reset)
    pub fn set_default_volumes(&mut self) {
        self.set_music_volume(DEFAULT_MUSIC_VOLUME);
        self.set_effect_volume(DEFAULT_EFFECT_VOLUME);
    }

    // Background music controls

    // SET BACKGROUND MUSIC
    // Sets the background music to the specified track
    pub fn set_background_music(&mut self, audio: &'a RaylibAudio, music: BackgroundMusic) {
        self.background_music = Some(audio.new_music(music.path()).expect("Failed to load background music"));
        self.background_music.as_mut().unwrap().set_volume(self.music_volume); // Maintain current volume when changing music
        // Background music as Option needs unwrapping 
    }

    // START
    // Launches background music if not already playing (only called once)
    pub fn start_background_music(&mut self) {
        if !self.music_playing {
            self.background_music.as_mut().unwrap().play_stream();
            self.music_playing = true;
        }
    }

    // TOGGLE OFF
    // Pauses background music (only if currently playing)
    pub fn pause_background_music(&mut self) {
        if self.music_playing {
            self.background_music.as_mut().unwrap().pause_stream();
            self.music_playing = false;
        }
    }

    // TOGGLE ON
    // Resumes background music (only if currently paused)
    pub fn resume_background_music(&mut self) {
        if !self.music_playing {
            self.background_music.as_mut().unwrap().resume_stream();
            self.music_playing = true;
        }
    }

    // CONTINUOUS STREAM UPDATE
    // Updates the music stream (called every frame)
    pub fn update_music_stream(&mut self) {
        if let Some(ref mut music) = self.background_music {
            music.update_stream();
        }
    }

    // Sound effects controls
    // To be added to actions that need a sound effect, directly add to action functions

    // LOAD SOUND EFFECT
    // Loads the specified sound effect and returns it (to be played immediately after)
    fn load_sound(&mut self, audio: &'a RaylibAudio, effect: SoundEffect) {
        let effect_path = effect.path();
        let new_sound: Sound<'a> = audio.new_sound(effect.path()).expect(&format!("Failed to load {effect_path}"));
        self.effects_cache.insert(effect, new_sound);
    }

    // PLAY SOUND EFFECT
    // Plays the specified sound effect from the cache at the current effect volume
    // Loads the sound effect if not already in the cache (lazy loading)
    pub fn play_sound_effect(&mut self, audio: &'a RaylibAudio, effect: SoundEffect) {
        if !self.effects_cache.contains_key(&effect) {
            SoundManager::load_sound(self, audio, effect); // Load sound effect if not in cache
        }
        // Play sound effect from the cache
        let cached_effect = self.effects_cache.get(&effect).unwrap();
        cached_effect.set_volume(self.effect_volume); // Set volume to current setting for effects
        cached_effect.play();
    }

    

}