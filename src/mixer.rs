use crate::rng::*;
use crate::sound_api::*;
use crate::load_wav::*;

use ringbuf::*;

pub enum Command {
    Load {
        path: String,
        id: SoundHandle,
    },
    Play {
        params: SoundDesc,
        id: PlayingSoundHandle,
    },
    Stop {
        id: PlayingSoundHandle,
    }
}

pub struct Channel {
    t: u64,
    id: PlayingSoundHandle,
    desc: SoundDesc,
}

pub struct SoundBuffer {
    id: SoundHandle,
    samples: Vec<f32>,
}

pub struct Mixer {
    pub sample_rate: f32,
    pub nchannels: usize,
    rng: Rng,
    sounds: Vec<SoundBuffer>,
    channels: Vec<Channel>,
    command_consumer: Consumer<Command>,

}
impl Mixer {
    pub fn tick(&mut self) -> f32 {
        let mut acc = 0.0;
        for i in 0..self.channels.len() {
            let sd = self.channels[i].desc;
            let b1 = self.get_sound_buffer(sd.a);
            if let Some(b2) = self.get_sound_buffer(sd.b) {

            } else {
                acc += sd.vol * b1[self.channels[i].t];
                // and get fade etc
                // and do other one
                // and deleting if done, maybe make delete a field of channel
                
            }
            self.channels[i].t += 1;
        }
        0.0
    }
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Play{params, id} => {},
            Command::Load{path, id} => self.load(path, id),
            Command::Stop{id} => {},
        }
    }
    pub fn load(&mut self, path: String, id: SoundHandle) {
        if let Some(samples) = load_wav(&path) {
            self.sounds.push(SoundBuffer { samples, id });
        }
    }
    pub fn stop(&mut self, id: SoundHandle) {
        for i in 0..self.channels.len() {
            if self.channels[i].id == id {
                self.channels.swap_remove(i);
                return;
            }
        }
    }
    pub fn play(&mut self, desc: SoundDesc, id: SoundHandle) {
        self.channels.push(Channel {
            t: 0,
            id,
            desc
        });
    }
    pub fn get_sound_buffer(&self, id: SoundHandle) -> &SoundBuffer {
        for i in 0..self.sounds.len() {
            if self.sounds[i].id == id {
                return &self.sounds[i]
            }
        }
        panic!("invalid sound");
    }
}
