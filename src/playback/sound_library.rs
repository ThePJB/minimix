use super::api::*;
use crate::signal::*;

pub struct Sound {
    pub id: BufferHandle,
    pub buf: Signal,
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
    pub fn get(&self, id: BufferHandle) -> &Sound {
        for i in 0..self.sounds.len() {
            if self.sounds[i].id == id { return &self.sounds[i] }
        }
        panic!("handle {:?} refers to nonexistant buffer", id);
    }
}