use std::f32::consts::PI;

pub fn lanczos_interp(s: &[f32], r: f32, a: usize) -> Vec<f32> {
    let sx = |x: f32| {
        let mut acc = 0.0;
        let lo = (x.floor() - a as f32 + 1.0).max(0.0) as usize;
        let hi = ((x.floor() + a as f32) as usize).min(s.len() - 1);
        for i in lo..hi {
            acc += s[i]*lanczos_kernel(x-i as f32, a);
        }
        acc
    };

    let len_out = (s.len() as f32 * r).ceil() as usize;
    let mut b = vec![0.0f32; len_out];
    for j in 0..len_out {
        let t = (j as f32 / len_out as f32) * s.len() as f32;
        b[j] = sx(t);
    }
    b
}

// a typ 2 or 3
fn lanczos_kernel(x: f32, a: usize) -> f32 {
    if x == 0.0 {
        1.0
    } else if x.abs() < a as f32 {
        let af = a as f32;
        let pi_x = PI*x;
        af*(pi_x.sin())*((pi_x/af).sin())/(pi_x*pi_x)
    } else {
        0.0
    }
}