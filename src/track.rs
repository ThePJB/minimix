use crate::sound_library::*;
use crate::sound_handle::*;

pub struct Track {
    vol: f32,
    id: SoundHandle,
    n: usize,
    n0: usize,
    n1: usize,
    nmax: usize,
    repeat: bool,
}

impl Track {
    pub fn new() -> Self {

    }
    pub fn accumulate_buffer(data: &mut [f32], num_channels: usize, loaded_sounds: &SoundLibrary) {
        let b = loaded_sounds.get(self.id);
        for &mut frame in data.chunks(num_channels) {
            for &mut sample in frame {
                let v_fade = if n0 == n1 {
                    1.0
                } else if n0 < n1 {
                    if self.n < n0 {
                        0.0
                    } else {
                        if self.n > n1 {
                            1.0
                        } else {
                            (self.n - n0) as f32 / (n1 - n0) as f32
                        }
                    }
                } else if n1 < n0 {
                    if self.n < n1 {
                        1.0
                    } else {
                        if self.n > n0 {
                            0.0
                        } else {
                            (self.n - n1) as f32 / (n0 - n1) as f32
                        }
                    }
                };
    
                let mut s = self.vol;
                s *= v_fade;
                s *= b.samples[self.n];
                *sample = s;
            }
            self.n = self.n + 1;
            if repeat {
                self.n = self.n % self.nmax
            } else {
                self.n = self.n.min(self.nmax)
            }
        }
    }
}