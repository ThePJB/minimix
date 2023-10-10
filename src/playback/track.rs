use super::sound_library::*;
use super::api::*;

pub struct Track {
    pub sound: BufferHandle,
    pub id: TrackHandle,
    pub n: usize,
    pub len: usize,
    pub repeat: bool,
}

impl Track {
    pub fn new(bh: BufferHandle, th: TrackHandle, repeat: bool, loaded_sounds: &SoundLibrary) -> Self {
        let b = loaded_sounds.get(bh);
        let len = b.buf.samples.len();
        let n = 0;
        Track {
            sound: bh,
            id: th,
            n,
            len,
            repeat,
        }
    }
    pub fn accumulate_buffer(&mut self, data: &mut [f32], num_channels: usize, loaded_sounds: &SoundLibrary) {
        let b = loaded_sounds.get(self.sound);
        for sample in data {
            // let t = 500;
            // let n = self.n % t;
            // *sample = (n as f32 / - (t as f32 / 2.0)) / (t as f32 / 2.0);
            // self.n += 1;
            *sample = b.buf.samples[self.n];
            self.n = self.n + 1;
            if self.repeat {
                self.n = self.n % self.len
            } else {
                self.n = self.n.min(self.len - 1)
            }
        }
    }
    // no work.. did it work previously? did it work with accumulate_buffer?

    // pub fn accumulate_buffer(&mut self, data: &mut [f32], num_channels: usize, loaded_sounds: &SoundLibrary) {
    //     let b = loaded_sounds.get(self.sound);
    //     for frame in data.chunks_mut(num_channels) {
    //         for sample in frame {
    //             *sample = b.buf.samples[self.n];
    //         }
    //         self.n = self.n + 1;
    //         if self.repeat {
    //             self.n = self.n % self.len
    //         } else {
    //             self.n = self.n.min(self.len - 1)
    //         }
    //     }
    // }
}