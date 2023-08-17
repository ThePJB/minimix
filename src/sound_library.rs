use crate::sound_buffer::*;
use crate::sound_handle::*;

pub struct SoundBuffer {
    pub id: SoundHandle,
    pub samples: Vec<f32>,
}

pub struct SoundLibrary {
    sounds: Vec<SoundBuffer>,
}

impl SoundLibrary {
    pub fn new() -> Self {
        SoundLibrary { sounds: vec![],  }
    }
    pub fn get(&self, id: SoundHandle) -> &SoundBuffer {
        for i in 0..self.sounds.len() {
            if self.sounds[i].id == id { return &self.sounds[i] }
        }
    }
}