use std::f32::consts::PI;
use minimix::playback::*;
use minimix::*;

#[test]
pub fn basic() {
    let mut mm = Minimixer::new(None);
    let wn = 2.0 * PI * (1.0 / 44100.0);
    let b = (0..44100).map(|i| i as f32 * wn * 220.0).map(|phase| phase.sin()).collect();
    let s = Signal { samples: b };
    let s = Signal::interleave(&vec![s.clone(), s.clone()]); // data for stereo
    let h = mm.load_buffer(s);
    std::thread::sleep(std::time::Duration::from_millis(10));
    mm.play(h, false);
    std::thread::sleep(std::time::Duration::from_millis(1000));
}

#[test]
pub fn resample() {
    let mut mm = Minimixer::new(None);
    let wn = 2.0 * PI * (1.0 / 44100.0);
    let b = (0..40100).map(|i| i as f32 * wn * 880.0).map(|phase| phase.sin()).collect();
    let s = Signal { samples: b };

    let signals = vec![s.clone(), s.resample(1.69), s.resample(2.0)];
    let handles: Vec<BufferHandle> = signals.iter()
        .map(|s| s.interleave2(&s.clone()))
        .map(|s| mm.load_buffer(s))
        .collect();

    std::thread::sleep(std::time::Duration::from_millis(10));
    for i in 0..handles.len() {
        mm.play(handles[i], false);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[test]
pub fn resample2() {
    let mut mm = Minimixer::new(None);
    let wn = 2.0 * PI * (1.0 / 44100.0);
    let b = (0..44100).map(|i| i as f32 * wn * 1200.0).map(|phase| phase.sin()).collect();
    let s = Signal { samples: b };

    let mut signals = vec![];
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0;
    dbg!(phi);
    for i in 0..10 {
        let r = phi.powf(i as f32);
        let from = 44100;
        let to = (44100 as f32 * r) as u32;
        signals.push(s.resample(r));
    }
    let handles: Vec<BufferHandle> = signals.iter()
        .map(|s| s.interleave2(&s.clone()))
        .map(|s| mm.load_buffer(s))
        .collect();

    std::thread::sleep(std::time::Duration::from_millis(10));
    for i in 0..10 {
        mm.play(handles[i], false);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[test]
pub fn chords() {
    let mut mm = Minimixer::new(None);

    let wn = 2.0 * PI * (1.0 / 44100.0);
    let make_sin = |v: &mut Vec<f32>, f| {
        for i in 0..v.len() {
            let phase: f32 = wn * f * i as f32;
            v[i] = phase.sin();
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
    let major_chord = |root_freq: f32| {
        let root = note(root_freq, 32000);
        let mid = note(root_freq * 1.2, 32000);
        let hi = note(root_freq * 1.5, 32000);
        // let seventh = note(root_freq 
        root.add_signal(&mid).add_signal(&hi)
    };
    let progression = |f_start: f32| {
        let mut chords = vec![];
        let mut current_freq = f_start;
        for i in 0..130 {
            dbg!(current_freq);
            chords.push(to_stereo(major_chord(current_freq)));
            current_freq *= (2.0 / 3.0);
            if current_freq > f_start {
                current_freq /= 2.0;
            }
            if current_freq < f_start {
                current_freq *= 2.0;
            }
        }
        chords
    };

    let c4 = 261.63 / 2.0;
    let prog = progression(c4);
    let handles: Vec<BufferHandle> = prog.iter().map(|x| mm.load_buffer(x.clone())).collect();

    for i in 0..handles.len() {
        mm.play(handles[i], false);
        std::thread::sleep(std::time::Duration::from_millis(400));
    }



//     mm.play(snd_handles[trig[i]], false);
//     // let f_start = 


//     let a = note(220.0, 22050); 
//     let b = note(246.94, 22050); 
//     let c = note(261.63, 22050); 
//     let d = note(293.66, 22050); 
//     let e = note(329.63, 22050);
//     let f = note(349.23, 22050);
//     let g = note(392.00, 22050);

//     let a_minor_chord = a.clone().add_signal(&c).add_signal(&e);
//     let c_major_chord = c.clone().add_signal(&e).add_signal(&g);

//     let snd_collection = [a, b, c, d, e, f, g, a_minor_chord, c_major_chord];
//     let snd_collection: Vec<Signal> = snd_collection.iter()
//         .map(|s| to_stereo(s.clone()))
//         .map(|s| s.set_vol(-10.0))
//     .collect();
// let snd_handles: Vec<BufferHandle> = snd_collection.iter().map(|s| mm.load_buffer(s.clone())).collect();
// let trig = [0, 1, 2, 3, 4, 5, 6, 7, 8];

//     std::thread::sleep(std::time::Duration::from_millis(10));

}
// pub fn blackman(n: usize, w: usize) -> f32 {
//     if n == 0 {
//         return 0.0;
//     }

//     let a0 = 0.42;
//     let a1 = 0.5;
//     let a2 = 0.08;

//     let i = w as f32 / n as f32;

//     a0 - a1 * (2.0 * PI * i).cos() + a2 * (4.0 * PI * i).cos()
// }

pub fn note(n: usize, f: f32) -> f32 {
    let wn = 2.0 * PI * (1.0 / 44100.0);
    let phase: f32 = wn * f * n as f32;
    return phase.sin()
}
pub fn chord(n: usize, f: f32) -> f32 {
    note(n, f) + note(n, f*1.2) + note(n, f*1.5)
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Note{f: f32, duration: f32},
    Chord{f: f32, duration: f32},
    Silence{duration: f32},
}

// 2 4 6 5 1
// chord of key == note of scale

pub fn circle_of_fifth_instruction(f: f32) -> Vec<Instruction> {
    const semitone_ratio: f32 = 1.059463094359;

    vec![
        Instruction::Chord { f, duration: 0.3 },
        Instruction::Silence { duration: 0.1 },
        Instruction::Chord { f, duration: 0.15 },
        Instruction::Silence { duration: 0.1 },
        Instruction::Note { f: f*2.0/3.0*semitone_ratio*semitone_ratio*semitone_ratio, duration: 0.2},
        Instruction::Note { f: f*2.0/3.0*semitone_ratio*semitone_ratio, duration: 0.2},
        Instruction::Note { f: f*2.0/3.0*semitone_ratio, duration: 0.2},
    ]
}

pub fn instruction_to_samples(inst: Instruction, dur: usize) -> Vec<f32> {
    match inst {
        Instruction::Note { f, duration } => (0..(duration * dur as f32) as usize).map(|i| note(i, f)).collect(),
        Instruction::Chord { f, duration } => (0..(duration * dur as f32) as usize).map(|i| chord(i, f)).collect(),
        Instruction::Silence { duration } => vec![0.0f32; (duration * dur as f32) as usize],
    }
}


// kinda doing flatten operation

fn sleep(ms: usize) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64))
}

#[test]
pub fn testthing() {
    let c4 = 261.63;
    let mut f = c4;
    let mut instruction = circle_of_fifth_instruction(f);
    for _ in 0..20 {
        f *= 2.0;
        f /= 3.0;
        if f < c4 {
            f *= 2.0;
        }
        if f > (2.0*c4) {
            f /= 2.0;
        }
        instruction.extend(circle_of_fifth_instruction(f));
    }
    dbg!(&instruction);
    let mut samples = vec![];
    for i in 0..instruction.len() {
        samples.extend(instruction_to_samples(instruction[i], 50000));
    }
    let sig = Signal{samples};
    let sig = sig.interleave2(&sig);
    let mut mm = Minimixer::new(None);
    let h = mm.load_buffer(sig);
    mm.play(h, false);
    sleep(10000);
}