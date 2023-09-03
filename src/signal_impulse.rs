use crate::signal::*;

impl Signal {
    // fc: normalized cutoff
    pub fn impulse_lp(fc: f32, n: usize) -> Self {
        let coeffs = lowpass_filter_coeffs(1.0, fc, n);
        Signal { samples: coeffs }
    }

    pub fn impulse_echo(d: usize, num_echos: usize, atten: f32) -> Self {
        let mut samples = vec![0.0; num_echos*d];
        let mut mag = 1.0;
        for i in 0..num_echos {
            samples[i*d] = mag;
            mag *= atten;
        }
        Signal { samples }
    }
}

pub fn lowpass_filter_coeffs(fs: f32, fc: f32, num_taps: usize) -> Vec<f32> {
    let nyquist = 0.5 * fs;
    let cutoff = fc / nyquist;

    // Calculate the filter impulse response (sinc function)
    let mut impulse_response = Vec::with_capacity(num_taps);
    let half_tap = (num_taps - 1) as f32 * 0.5;
    for n in 0..num_taps {
        let sinc_val = if n as f32 == half_tap {
            2.0 * cutoff
        } else {
            let t = (n as f32 - half_tap) * std::f32::consts::PI * cutoff;
            (t).sin() / t
        };
        impulse_response.push(sinc_val);
    }

    // Apply a window function (Hanning) to the impulse response
    let windowed_response: Vec<f32> = impulse_response
        .iter()
        .enumerate()
        .map(|(i, x)| x * 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (num_taps - 1) as f32).cos()))
        .collect();

    // Normalize the filter coefficients
    let sum: f32 = windowed_response.iter().sum();
    let normalized_response: Vec<f32> = windowed_response.iter().map(|x| x / sum).collect();

    normalized_response
}