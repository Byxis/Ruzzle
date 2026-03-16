use raylib::prelude::*;

pub struct SoundManager<'a> {
    // Drop the explicit lifetimes in the struct definition if possible, 
    // or use the lifetime of the Audio handle.
    pub background_music: Music<'a>,
    pub walking_sound: Sound<'a>,
    pub jump_sound: Sound<'a>,
    pub music_playing: bool,
}

impl<'a> SoundManager<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut audio = RaylibAudio::init_audio_device()
        .expect("Could not open audio device");
        // Use the RaylibAudio handle (rl) to load sounds
        let background_music: Music<'a> = audio.load_music_stream(thread, "rsc/background_music.ogg")
            .expect("Failed to load music");
        let walking_sound: Sound<'a> = audio.load_sound(thread, "rsc/walking_sound.wav")
            .expect("Failed to load walking sound");
        let jump_sound: Sound<'a> = audio.load_sound(thread, "rsc/jump_sound.wav")
            .expect("Failed to load jump sound");

        SoundManager {
            background_music,
            walking_sound,
            jump_sound,
            music_playing: false,
        }

        
    }
    pub fn play_background_music(&mut self) {
        //launches background music if not already playing
        if !self.music_playing {
            self.background_music.play();
            self.music_playing = true;
        }
    }

    pub fn pause_background_music(&mut self) {
        //pauses background music (only if currently playing)
        if self.music_playing {
            self.background_music.pause();
            self.music_playing = false;
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
}