use crate::*;
use std::f32::consts::PI;

#[test]
pub fn chords() {
    let mut mm = Minimixer::new(None);

    let wn = 2.0 * PI * (1.0 / 44100.0);
    let make_sin = |v: &mut Vec<f32>, f| {
        for i in 0..v.len() {
            v[i] = wn * f;
        }
    };
    let wnd_blackman = |v: &mut Vec<f32>| {
        for i in 0..v.len() {
            let coeff = 0.42 - 0.5 * (2.0 * PI * i as f32 / v.len() as f32).cos() + 0.08 * (4.0 * PI * i as f32 / v.len() as f32).cos();
            v[i] *= coeff;
        }
    };
    let note = |f, n| {
        let mut v = vec![0.0f32; n];
        make_sin(&mut v, f);
        wnd_blackman(&mut v);
        Signal {samples: v}
    };
    let to_stereo = |s: Signal| {
        let s2 = s.clone();
        Signal::interleave(&vec![s, s2])
    };

    let a = note(220.0, 22050); 
    let b = note(246.94, 22050); 
    let c = note(261.63, 22050); 
    let d = note(293.66, 22050); 
    let e = note(329.63, 22050);
    let f = note(349.23, 22050);
    let g = note(392.00, 22050);

    let a_minor_chord = a.clone().add_signal(&c).add_signal(&e);
    let c_major_chord = c.clone().add_signal(&e).add_signal(&g);

    let snd_collection = [a, b, c, d, e, f, g, a_minor_chord, c_major_chord];
    let snd_collection: Vec<Signal> = snd_collection.iter()
        .map(|s| to_stereo(s.clone()))
        .map(|s| s.set_vol(-10.0))
    .collect();
    let trig = [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1];
    let snd_handles: Vec<BufferHandle> = snd_collection.iter().map(|s| mm.load_buffer(s.clone())).collect();

    std::thread::sleep(std::time::Duration::from_millis(10));
    for i in 0..trig.len() {
        mm.play(snd_handles[trig[i]], false);
        std::thread::sleep(std::time::Duration::from_millis(600));
    }
}