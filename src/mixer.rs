use crate::rng::*;
use crate::sound_api::*;
use crate::load_wav::*;

use ringbuf::*;

// a few issues
// need to bake in the index of the buffer


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
    t_max: u64,
    id: PlayingSoundHandle,
    desc: SoundDesc,
}

// what about a_index, b_index
// a_index a_index fade? why not bra
// think dj decks
// yea each channel needs 2 'tracks' or whatever
// and give tracks whatever relative time u want
// manage times in seconds or what?




// !!!! playing track needs to be able to bake the index of its underlying track !!!!
// !!!! 2 Tracks to a channel!
// !!!! its kind of 2 tracks with volume lerp


// track.seek - could make music

// oh yeah and if you could record and replay tracks that would be sweet
// yea runtime manageable please

// 2 tracks to a stream, tracks with own offset (not t, t => offset)
// we could honestly move the sound sources in and have them be completely static ..... but dynamic
// just at creation time put the buffer 
// but i like the idea of addressing the buffers, supporting operations, crop etc.

// oh yea and race condition of loading wav file vs. playing wav file -- Just dont crash and its OK

impl Channel {
    // pub fn new(id: PlayingSoundHandle, desc: SoundDesc) {
    //     Channel {

    //     }
    // }
    pub fn tick(&mut self, mixer: &Mixer) -> f32 {
        let sd = self.desc;
        let b1 = self.get_sound_buffer(sd.a);
        let s1 = b1[self.t];
        if let Some(b) = sd.b {
            let t_transition = if self.t < sd.t_begin_transition {
                0.0
            } else if self.t < sd.t_end_transition {
                 (self.t - sd.t_begin_transition) as f32 /
                 (sd.t_end_transition - sd.t_begin_transition) as f32
            } else {
                1.0
            };
            let b2 = self.get_sound_buffer(b);
            let s2 = b2[self.t];
            

        } else {
            acc += sd.vol * b1[self.channels[i].t];

        }
        self.t += 1;
        if sd.repeat {
            self.t = self.t % self.t_max;
        }
    }
}

pub struct Mixer {
    pub sample_rate: usize,
    pub nchannels: usize,
    rng: Rng,
    sound_library: SoundLibrary,
    sounds: Vec<SoundBuffer>,
    channels: Vec<Channel>,
    command_consumer: Consumer<Command>,

}
impl Mixer {
    pub fn tick_channel(&mut self, idx: usize) -> f32 {
        
    } 
    pub fn tick(&mut self) -> f32 {
        let mut acc = 0.0;
        for i in 0..self.channels.len() {
            self.channels[i].tick(self);

            self.tick_channel(i);
            
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
    pub fn handle_commands(&mut self) {
        while let Some(command) = self.command_consumer.next() {
            self.handle_command(command)
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
