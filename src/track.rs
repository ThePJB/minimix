use crate::sound_library::*;
use crate::sound_api::*;

pub struct Track {
    pub id: SoundHandle,
    pub n: usize,
    pub len: usize,
    pub repeat: bool,
}

impl Track {
    pub fn new(id: SoundHandle, repeat: bool, loaded_sounds: &SoundLibrary) -> Self {
        let b = loaded_sounds.get(id);
        let len = b.buf.samples.len();
        let n = 0;
        Track {
            id,
            n,
            len,
            repeat,
        }
    }
    pub fn accumulate_buffer(&mut self, data: &mut [f32], num_channels: usize, loaded_sounds: &SoundLibrary) {
        let b = loaded_sounds.get(self.id);
        for frame in data.chunks_mut(num_channels) {
            for sample in frame {
                *sample = b.buf.samples[self.n];
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