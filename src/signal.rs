use crate::interp::*;

#[derive(Clone)]
pub struct Signal {
    pub samples: Vec<f32>,
}

impl Signal {
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn max(&self) -> f32 {
        let mut max = std::f32::NEG_INFINITY;
        for sample in self.samples.iter() {
            if *sample > max {
                max = *sample;
            }
        }
        max
    }

    // set_vol 0 to normalize
    // values are in range -1 to 1
    pub fn set_vol(mut self, db: f32) -> Signal {
        let scaling_factor = 10.0_f32.powf(db / 20.0);
        let max = self.max();
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

    pub fn interleave2(&self, other: &Signal) -> Signal {
        Self::interleave(&vec![self.clone(), other.clone()])
    }

    /// Interleaves a signal (inverse of commutation)
    pub fn interleave(signals: &Vec<Signal>) -> Signal {
        let n = signals[0].len() * signals.len();
        let mut s = Signal { samples: vec![] };
        for i in 0..n {
            s.samples.push(signals[i%signals.len()].samples[i / signals.len()]);
        }
        s
    }

    pub fn resample(&self, r: f32) -> Signal {
        let samples = lanczos_interp(&self.samples, r, 4);
        Signal { samples }
    }
}