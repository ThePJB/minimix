use crate::load_wav::*;
use crate::rng::*;

pub struct SoundBuffer {
    pub samples: Vec<f32>,
}

impl SoundBuffer {
    pub fn from_file(path: &str) -> Self {
        SoundBuffer {
            samples: load_wav(path).expect(&format!("failed to load {}", path))
        }
    }

    pub fn blackman(n: usize) -> Self {
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let coeff = 0.42 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos()
                + 0.08 * (4.0 * std::f32::consts::PI * i as f32 / n as f32).cos();
            samples.push(coeff);
        }
        SoundBuffer { samples }
    }

    pub fn sine(n: usize, w: f32) -> Self {
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let sample = (w * i as f32).sin();
            samples.push(sample);
        }
        SoundBuffer { samples }
    }

    pub fn noise(n: usize, rng: &mut Rng) -> Self {
        let mut samples = Vec::with_capacity(n);
        for _ in 0..n {
            samples.push(rng.next_float());
        }
        SoundBuffer { samples }
    }

    pub fn adsr(a: f32, d: f32, s: f32, r: f32, n: usize) -> Self {
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let mut envelope = a * (i as f32) / (a * n as f32);
            if i > (a * n as f32) as usize {
                envelope = (s - 1.0) * ((i as f32) - (a * n as f32)) / (d * n as f32) + 1.0;
            }
            if i > ((a + d) * n as f32) as usize {
                envelope = s;
            }
            if i > ((a + d + s) * n as f32) as usize {
                envelope = (r - s) * ((i as f32) - ((a + d + s) * n as f32)) / (r * n as f32) + s;
            }
            samples.push(envelope);
        }
        SoundBuffer { samples }
    }

    pub fn scale(&mut self, a: f32) {
        for sample in &mut self.samples {
            *sample *= a;
        }
    }

    pub fn elementwise_mul(&mut self, other: &SoundBuffer) {
        self.samples.iter_mut().zip(&other.samples).for_each(|(s1, s2)| {
            *s1 *= *s2;
        });
    }

    pub fn conv_full(&mut self, other: &SoundBuffer) {
        let mut result = vec![0.0; self.samples.len() + other.samples.len() - 1];
        for i in 0..result.len() {
            for j in 0..other.samples.len() {
                if i >= j && i - j < self.samples.len() {
                    result[i] += self.samples[i - j] * other.samples[j];
                }
            }
        }
        self.samples = result;
    }

    pub fn conv_short(&mut self, other: &SoundBuffer) {
        let mut result = vec![0.0; self.samples.len() + other.samples.len() - 1];
        for i in 0..result.len() {
            for j in 0..other.samples.len() {
                if i >= j && i - j < self.samples.len() {
                    result[i] += self.samples[i - j] * other.samples[j];
                }
            }
        }
        self.samples = result.split_off(other.samples.len() - 1);
    }

    // yea it would be pretty god damn easy to do arbitrary synthesis techniques here. like fm. 
    // frequency sweep
    // crossfade
    // I still want to crossfade something with itself
    // FIR impulses
}