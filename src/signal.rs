use crate::image::*;
use crate::vector::*;
use crate::load_wav::*;
use std::ops::AddAssign;
use std::ops::MulAssign;

// We are at the point where a lot of this shit is not strictly necessary for minimix but it is how you can generate sounds for minimix

// generate a really nice readme demonstrating eg each thing in signal_synth with plots of at least time domain shit
// its kinda jupyter notebook-esque
// gen_readme test

// A note, i think this still makes sense even as a layer in custom synth, needs to buffer, is maybe more latency, shrug
// like you would only swap the sound at discrete intervals of them modifying the params
// modulation interpolates between buffers?

// speculative note: idk resample, decimate, heterodyne?

// A 1d signal: also a ndvec. does it make sense to consider proj, dot product, etc?
// more shit it could have:
// * to_complex
// * hilbert
// * fft
// * ifft
// * trunc (take the transient off a snare or hat or something)
// * envelope follower (or get off hilbert)
// * plot time, plot freq
#[derive(Clone)]
pub struct Signal {
    pub samples: Vec<f32>,
}

impl Signal {
    pub fn plot(&self, path: &str) {
        let xres = 640;
        let yres = 480;
        let mut buf = ImageBuffer::new(xres, yres);
        buf.plot(&self.samples, vec4(0.0, 0.0, 1.0, 1.0), 3);
        buf.dump_to_file(path);
    }
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn load(path: &str) -> Self {
        Signal {samples: load_wav(path).expect("failed to load path") }
    }
    pub fn save(&self, path: &str) {
        write_wav(path, &self.samples, 44100)
    }
    pub fn set_len(&mut self, n: usize) {
        self.samples.resize(n, 0.0);
    }
    pub fn conv_fast(&mut self, other: &Signal) -> Self {
        let len = self.len() + other.len() - 1;
        self.set_len(len);
        let mut b = other.clone();
        b.set_len(len);

        let mut a = self.fft();
        let b = b.fft();
        a *= &b;
        a.ifft()
    }
    pub fn conv_full(&mut self, other: &Signal) {
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
    pub fn conv_short(&mut self, other: &Signal) {
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
    pub fn cat(&mut self, other: &Signal) {
        self.samples.extend_from_slice(&other.samples);
    }
    pub fn tanh(&mut self) {
        for sample in &mut self.samples {
            *sample = sample.tanh();
        }
    }
    // crossfades other into self. use polynomial interpolation. i did it in 
    pub fn crossfade(&mut self, other: &Signal, n_start: usize, n_end: usize) {

    }
    pub fn fft(&self) -> Self {
        // bruh im not even using dat shit
        // can it do real ffts or whatever  
        use rustfft::{FftPlanner, num_complex::Complex};
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.len());
        let mut buffer: Vec<Complex<f32>> = self.samples.iter().map(|x| Complex{ re: *x, im: 0.0f32}).collect();
        fft.process(&mut buffer);
        Signal { samples: buffer.iter().map(|x| x.re).collect() }
    }
    pub fn ifft(&self) -> Self {
        use rustfft::{FftPlanner, num_complex::Complex};
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(self.len());
        let mut buffer: Vec<Complex<f32>> = self.samples.iter().map(|x| Complex{ re: *x, im: 0.0f32}).collect();
        fft.process(&mut buffer);
        Signal { samples: buffer.iter().map(|x| x.re).collect() }
    }
}

// Implement the *= operator overload for element-wise multiplication with another Signal
impl MulAssign<&Signal> for Signal {
    fn mul_assign(&mut self, other: &Signal) {
        for (s, o) in self.samples.iter_mut().zip(&other.samples) {
            *s *= *o;
        }
    }
}

// Implement the += operator overload for element-wise addition with another Signal
impl AddAssign<&Signal> for Signal {
    fn add_assign(&mut self, other: &Signal) {
        for (s, o) in self.samples.iter_mut().zip(&other.samples) {
            *s += *o;
        }
    }
}

// Implement the *= operator overload for scalar multiplication
impl MulAssign<f32> for Signal {
    fn mul_assign(&mut self, scalar: f32) {
        for s in &mut self.samples {
            *s *= scalar;
        }
    }
}

// Implement the *= operator overload for DC shift
impl AddAssign<f32> for Signal {
    fn add_assign(&mut self, scalar: f32) {
        for s in &mut self.samples {
            *s += scalar;
        }
    }
}
