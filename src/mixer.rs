use crate::rng::*;
use crate::sound_api::*;
use crate::track::*;
use crate::sound_library::*;

use ringbuf::*;

pub enum Command {
    Play {
        params: SoundDesc,
        id: PlayingSoundHandle,
    },
    Stop {
        id: PlayingSoundHandle,
    }
}

pub struct Mixer {
    pub sample_rate: usize,
    pub nchannels: usize,
    rng: Rng,
    command_consumer: Consumer<Command>,
    sound_consumer: Consumer<Sound>,
    sound_library: SoundLibrary,
    tracks: Vec<Track>,

}
impl Mixer {
    pub fn new(sample_rate: usize, nchannels: usize, command_consumer: Consumer<Command>, sound_consumer: Consumer<Sound>) -> Self {
        let rng = Rng::new_random();
        let sound_library = SoundLibrary::new();
        let tracks = vec![];
        Mixer {
            sample_rate,
            nchannels,
            rng,
            command_consumer,
            sound_consumer,
            sound_library,
            tracks,
        }
    }
    pub fn write_samples(&mut self, output: &mut [f32]) {
        for track in self.tracks.iter_mut() {
            track.accumulate_buffer(output, self.nchannels, &self.sound_library);
        }
    }
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Play{params, id} => {},
            Command::Stop{id} => {},
        }
    }
    pub fn handle_commands(&mut self) {
        while let Some(command) = self.command_consumer.next() {
            self.handle_command(command)
        }
    }
    pub fn load_sounds(&mut self) {
        while let Some(sound) = self.sound_consumer.next() {
            self.sound_library.push(sound);
        }
    }
    pub fn stop(&mut self, id: SoundHandle) {
        for i in 0..self.tracks.len() {
            if self.tracks[i].id == id {
                self.tracks.swap_remove(i);
                return;
            }
        }
    }
    pub fn play(&mut self, desc: SoundDesc, id: SoundHandle) {
        self.tracks.push(Track::new(id, desc.repeat, &self.sound_library));
    }
}