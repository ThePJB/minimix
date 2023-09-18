use crate::load_wav::*;
use ordered_float::*;

#[derive(Clone)]
pub struct Signal {
    pub samples: Vec<f32>,
}

impl Signal {
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn load(path: &str) -> Self {
        Signal {samples: load_wav(path).expect("failed to load path") }
    }
    pub fn save(&self, path: &str, sample_rate: u32) {
        write_wav(path, &self.samples, sample_rate)
    }
    // set_vol 0 to normalize
    // values are in range -1 to 1
    pub fn set_vol(mut self, db: f32) -> Signal {
        let scaling_factor = 10.0_f32.powf(db / 20.0);
        let max = self.samples.iter().map(|x| OrderedFloat(x.abs())).max().unwrap().0;
        for sample in self.samples.iter_mut() {
            *sample /= max;
            *sample *= scaling_factor;
        }
        self
    }
    // add 2 signals
    pub fn add_signal(mut self, other: &Signal) -> Self {
        self.samples.resize(other.len().max(self.len()), 0.0);
        for i in 0..other.len() {
            self.samples[i] += other.samples[i];
        }
        self
    }
}