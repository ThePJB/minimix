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

    /// Commutates a signal (use for splitting a wav file by number of channels for example)
    ///  n > 0
    pub fn commutate(&self, n: usize) -> Vec<Signal> {
        let mut signals = vec![Signal { samples: vec![] }];
        for i in 0..self.samples.len() {
            signals[i % n].samples.push(self.samples[i]);
        }
        signals
    }

    /// Interleaves a signal (inverse of commutation)
    pub fn interleave(signals: &Vec<Signal>) -> Signal {
        let n = signals[0].len() * signals.len();
        let mut s = Signal { samples: vec![] };
        for i in 0..n {
            s.samples.push(signals[i%signals.len()].samples[i / n]);
        }
        s
    }
 }