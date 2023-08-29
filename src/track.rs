use crate::sound_library::*;
use crate::sound_api::*;

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
        // let b = SoundBuffer::sine(5000, 2.0*std::f32::consts::PI*100.0);
        for frame in data.chunks_mut(num_channels) {
            for sample in frame {
                *sample = b.buf.samples[self.n];
                // *sample = (100.0 * 2.0 * std::f32::consts::PI / 44100.0 * self.n as f32).sin()
                // as this is expected, the buf must be fucked
            }
            self.n = self.n + 1;
            if self.repeat {
                self.n = self.n % self.len
            } else {
                self.n = self.n.min(self.len - 1)
            }
        }
    }
}