use std::f32::consts::PI;
use crate::signal::Signal;
use crate::rng::Rng;

// synthesis ideas:
// tanh tube amp clipping
// doom shit: filter, subtract one to get both, then fuck up each one separately, then re add
// possibly delay separately as well
// hardstyle kick
// FM synth
// additive synth
// that IIR shit / simulation shit
// ringmod example
// yea it would be pretty god damn easy to do arbitrary synthesis techniques here. like fm. 
// frequency sweep
// crossfade (incl. crossfade with self mmm)
// FIR impulses

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

impl Signal {
    pub fn zero(n: usize) -> Self {
        Signal { samples: vec![0.0; n] }
    }
    pub fn sine(n: usize, f: f32, fs: f32) -> Self {
        let samples = (0..n).map(|x| x as f32 * 2.0 * PI * f / fs).map(|phase| phase.sin()).collect();
        Signal { samples }
    }
    pub fn sweep(n: usize, f1: f32, f2: f32, fs: f32) -> Self {
        let mut samples = vec![0.0; n];
        let mut phase = 0.0;
        for i in 0..n {
            let f = lerp(f1, f2, i as f32 / n as f32);
            phase += 2.0 * PI * f / fs;
            samples[i] = phase.sin();
        }
        Signal { samples }
    }
    pub fn white(n: usize) -> Self {
        let mut rng = Rng::new_random();
        let samples = (0..n).map(|x| rng.next_float()*2.0 - 1.0).collect();
        Signal { samples }
    }
    pub fn wnd_adsr(n: usize, a: f32, d: f32, s: f32, r: f32, fs: f32) -> Self {
        let samples = (0..n).map(|i| envelope_adsr(n, i, a, d, s, r, 44100.0)).collect();
        Signal { samples }
    }
    pub fn wnd_exp(n: usize, a: f32) -> Self {
        let mut buf = vec![0.0; n];
        let mut val = 1.0;
        for i in 0..buf.len() {
            buf[i] = val;
            val *= a;
        }
        Signal {samples: buf}
    }
    pub fn wnd_blackman(n: usize) -> Self {
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let coeff = 0.42 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos()
                + 0.08 * (4.0 * std::f32::consts::PI * i as f32 / n as f32).cos();
            samples.push(coeff);
        }
        Signal { samples }
    }
    pub fn kick(n: usize) -> Self {
        let mut buf = Self::sine(n, 2.0*PI*50.0, 44100.0);
        
        let mut buf2 = buf.clone();
        // wnd should be exponential
        buf *= &Self::wnd_exp(n, 0.999);
        buf *= 2.0;
        buf.tanh();
        buf *= 1.1;
        buf.tanh();
        buf *= 1.1;
        buf.tanh();
        buf *= 1.1;
        buf.tanh();
        buf *= 16.0;
        buf.tanh();
        buf.tanh();

        // maybe u would zinv a few times n do echo / rumble

        buf2 *= &Self::wnd_exp(n, 0.99);
        buf2 *= 4.0;
        buf2.tanh();
        buf2 *= 8.0;
        buf2.tanh();
        buf2 *= 8.0;
        buf2.tanh();
        buf2 *= 4.0;
        buf2.tanh();

        buf += &buf2;
        buf.tanh();

        buf
    }
}

// outputs window between 0 and 1.0
// a, d and r are times in seconds
pub fn envelope_adsr(len: usize, current_sample: usize, a: f32, d: f32, s: f32, r: f32, sample_rate: f32) -> f32 {
    // Calculate the total time for the envelope (attack + decay + sustain + release)
    let total_time = a + d + r;

    // Calculate the number of samples for each phase
    let a_samples = (a * sample_rate) as usize;
    let d_samples = (d * sample_rate) as usize;
    let r_samples = (r * sample_rate) as usize;

    // Calculate the sustain level
    let sustain_level = s;

    // Check which phase we are in
    if current_sample < a_samples {
        // We are in the attack phase
        // Calculate the linear interpolation between 0 and 1 for the attack phase
        return (current_sample as f32) / (a_samples as f32);
    } else if current_sample < (a_samples + d_samples) {
        // We are in the decay phase
        // Calculate the linear interpolation between 1 and the sustain level for the decay phase
        let decay_start = a_samples as f32;
        let decay_samples = d_samples as f32;
        return 1.0 - ((current_sample as f32 - decay_start) / decay_samples) * (1.0 - sustain_level);
    } else if current_sample < (len - r_samples) {
        // We are in the sustain phase
        return sustain_level;
    } else {
        // We are in the release phase
        // Calculate the linear interpolation between the sustain level and 0 for the release phase
        let release_start = (len - r_samples) as f32;
        let release_samples = r_samples as f32;
        return sustain_level * (1.0 - (current_sample as f32 - release_start) / release_samples);
    }
}
