use crate::sound_api::*;
use crate::sound_buffer::*;

pub struct Sound {
    pub id: SoundHandle,
    pub buf: SoundBuffer,
}

pub struct SoundLibrary {
    sounds: Vec<Sound>,
}

impl SoundLibrary {
    pub fn new() -> Self {
        SoundLibrary { sounds: vec![],  }
    }
    pub fn push(&mut self, buf: Sound) {
        self.sounds.push(buf);
    }
    pub fn get(&self, id: SoundHandle) -> &Sound {
        for i in 0..self.sounds.len() {
            if self.sounds[i].id == id { return &self.sounds[i] }
        }
        panic!("invalid sound handle");
    }
}