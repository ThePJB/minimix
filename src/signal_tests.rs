use crate::signal::*;
use crate::signal_impulse::*;
use crate::sound_api::*;
use std::time::Duration;

pub fn play(s: &Signal) {
    let slen = s.samples.len();
    let mut e = SoundAPI::new();
    let h = e.load_buffer(s.clone());
    std::thread::sleep(Duration::from_millis(100));
    e.play(h, false);
    let ms_per_sample = 1000.0 / 44100.0;
    let ms = slen as f32 * ms_per_sample;
    dbg!(slen, ms_per_sample, ms);
    std::thread::sleep(Duration::from_millis(ms as u64));
}

#[test]
pub fn test_clap_echo() {
    let mut a = Signal::white(4000);    // .wnd_exp() or .mul(Signal::wnd_exp)
    let wnd = Signal::wnd_exp(4000, 0.999);
    a *= &wnd;
    let i = Signal::impulse_echo(14000, 6, 0.5);
    i.plot("echo_impulse.png");
    // a.conv_full(&i);    // yea dis shiz be slow, faster to do directly a .echo
    a.conv_fast(&i);    // yea dis shiz be slow, faster to do directly a .echo
    a.plot("result.png");
    // yea conv fast be not working
    // or it fft time ay
    // wow convolution is cringe af
    play(&a);
}

#[test]
pub fn test_sweep() {
    let b = Signal::sweep(80000, 220.0, 880.0, 44100.0);
    let mut e = SoundAPI::new();
    let h = e.load_buffer(b);
    std::thread::sleep(Duration::from_millis(100));
    e.play(h, false);
    std::thread::sleep(Duration::from_millis(2000));

}

#[test]
pub fn test_sweep_ringmod() {
    let mut b = Signal::sweep(80000, 220.0, 440.0, 44100.0);
    let b2 = Signal::sweep(80000, 440.0, 220.0, 44100.0);
    // let b2 = Signal::sine(8000, 10.0, 44100.0);
    b *= &b2;
    let mut e = SoundAPI::new();
    let h = e.load_buffer(b);
    std::thread::sleep(Duration::from_millis(100));
    e.play(h, false);
    std::thread::sleep(Duration::from_millis(2000));
}

// ring mod with noise?

#[test]
fn test_play_sin() {
    let s = Signal::sine(40000, 220.0, 44100.0);
    play(&s);
}

// comb of sin will just be sin * A
#[test]
pub fn test_comb() {
    let mut s = Signal::sine(40000, 220.0, 44100.0);
    let mut s_copy = s.clone();
    let mut z = Signal::zero(10);
    z.cat(&s_copy);
    s_copy += &z;
    s_copy *= 0.5;
    s.cat(&s_copy);

    play(&s);
}

// comb of noise
#[test]
pub fn test_noise() {
    let mut s = Signal::white(40000);
    let mut z = Signal::zero(30);
    z.cat(&s);
    let mut s2 = s.clone();
    s2 += &z;
    s.cat(&s2);


    play(&s);
}

#[test]
pub fn kick() {
    let mut transient = Signal::white(50);
    let mut env = Signal::wnd_exp(80000, 0.9999);
    let mut s = Signal::sine(80000, 400.0, 44100.0);

    s *= &env;

    transient.cat(&s);
    play(&transient);

    let t1 = transient.clone();
    let t2 = transient.clone();

    // do munted shit
    // robust primitives: tape delay
    // me needing a cool reverb impulse

    

}

// echo impulse
// reverb impulse


#[test]
pub fn asd() {
    let mut a = Signal::white(50);
    let mut b = Signal::sine(80000, 100.0, 44100.0);
    let mut c = a.clone();
    c.cat(&b);
    a.save("a.wav");
    b.save("b.wav");
    c.save("c.wav");
    play(&c);
}


mod test {
    use std::f32::consts::PI;
    use crate::sound_api::*;
    use crate::signal::*;
    use crate::signal_synth::*;
    use std::time::Duration;

    #[test]
    pub fn test_beats() {
        // hmmmm ring buf as two frequencies vary? that would also be a good sound intuitionizer demo
        // like a 2d: matrix, identity line will probs be sin^2
        // what if u zinv and mul or zinv and add. zinv and add is comb filter
        // understanding ringmod
        // this ringmod sounds like it makes the sum and difference frequencies
        // a principled approach to sound production
        // so ring mod will apply an envelope, yea makes sense
        // but what if both are complex
        // take voice signal, multiply voice with hf carrier signal and it sounds sick
        // sweep that shit
        // it does sound really crazy
        // klaxon effect too
        // weird fx: mult with frequency sweep
        // what if u just multiplied 2 frequency sweeps lol
        // timesing random shit good
        // what about convolving random shit
        // eg white noise etc or sin. convolution is the sum of the diagonals. 
        // conv is the sum of all shifted scalar multiplications of the components of one signal with the other signal
        // conv does not contain ring mod (plain mul)
        // convolution of 2 sine waves produces a sinc. thats because its the same as ifft( fft(sin) * fft(sin) ) I guess, because they can be a rect function or some shit idk
        // conv in frequency is mul in time, mul in time is conv in frequency
        let mut sound_endpoint = SoundAPI::new();
        let mut buf1 = Signal::sine(5000, 2.0*PI*220.0, 44100.0);
        let mut buf2 = Signal::sine(5000, 2.0*PI*555.0, 44100.0);
        buf2 *= &buf1;
        let h1 = sound_endpoint.load_buffer(buf1);
        let h2 = sound_endpoint.load_buffer(buf2);

        dbg!(h1, h2);

        for i in 0..2 {
            sound_endpoint.play(h1, false);
            std::thread::sleep(Duration::from_secs(1));
            sound_endpoint.play(h2, false);
            std::thread::sleep(Duration::from_secs(1));
        }

    }

    #[test]
    pub fn test_kick() {
        // hmmmm ring buf as two frequencies vary? that would also be a good sound intuitionizer demo
        // like a 2d: matrix, identity line will probs be sin^2
        let mut sound_endpoint = SoundAPI::new();
        let mut buf = Signal::kick(160000);
        let h1 = sound_endpoint.load_buffer(buf);

        for i in 0..1 {
            sound_endpoint.play(h1, false);
            std::thread::sleep(Duration::from_secs(1));
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    #[test]
    pub fn test_load() {
        let mut sound_endpoint = SoundAPI::new();
        let mut buf = Signal::load("waveguide.wav");
        let h1 = sound_endpoint.load_buffer(buf);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h1, false); // sum ting wong it no play
        std::thread::sleep(Duration::from_secs(1));
    }
    #[test]
    pub fn test_sin() {
        let mut sound_endpoint = SoundAPI::new();
        let mut buf = Signal::sine(5000, 200.0, 44100.0);
        let h1 = sound_endpoint.load_buffer(buf);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h1, false); // sum ting wong it no play
        std::thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_tanh() {
        let sb = Signal::sine(30000, 220.0, 44100.0);
        
        let mut tb1 = sb.clone();
        tb1.tanh();

        let mut tb2 = sb.clone();
        tb2 *= 2.0;
        tb2.tanh();

        let mut tb3 = sb.clone();
        tb3 *= 3.0;
        tb3.tanh();

        let mut s = sb.clone();
        s.cat(&tb1);
        s.cat(&tb2);
        s.cat(&tb3);

        let mut sound_endpoint = SoundAPI::new();
        let h = sound_endpoint.load_buffer(s);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h, false);

        std::thread::sleep(Duration::from_millis(3000));
    }

    #[test]
    fn test_tanh2() {
        let sb = Signal::sine(30000, 220.0, 44100.0);
        
        let mut tb1 = sb.clone();
        tb1.tanh();

        let mut tb2 = sb.clone();
        tb2 *= 1.1;
        tb2.tanh();

        let mut tb3 = sb.clone();
        tb3 *= 1.2;
        tb3.tanh();

        let mut s = sb.clone();
        s.cat(&tb1);
        s.cat(&tb2);
        s.cat(&tb3);

        let mut sound_endpoint = SoundAPI::new();
        let h = sound_endpoint.load_buffer(s);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h, false);

        std::thread::sleep(Duration::from_millis(3000));
    }

    #[test]
    fn test_tanh3() {
        let sb = Signal::sine(30000, 220.0, 44100.0);
        
        let mut tb1 = sb.clone();
        tb1.tanh();

        let mut tb2 = sb.clone();
        tb2.tanh();
        tb2.tanh();

        let mut tb3 = sb.clone();
        tb3.tanh();
        tb3.tanh();
        tb3.tanh();

        let mut s = sb.clone();
        s.cat(&tb1);
        s.cat(&tb2);
        s.cat(&tb3);

        let mut sound_endpoint = SoundAPI::new();
        let h = sound_endpoint.load_buffer(s);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h, false);

        std::thread::sleep(Duration::from_millis(3000));
    }

    #[test]
    fn test_tanh4() {
        let sb = Signal::sine(30000, 220.0, 44100.0);
        
        let mut tb1 = sb.clone();
        tb1.tanh();

        let mut tb2 = sb.clone();
        tb2.tanh();
        tb2 *= 2.0;
        tb2.tanh();
        
        let mut tb3 = sb.clone();
        tb3.tanh();
        tb3 *= 2.0;
        tb3.tanh();
        tb3 *= 2.0;
        tb3.tanh();

        let mut s = sb.clone();
        s.cat(&tb1);
        s.cat(&tb2);
        s.cat(&tb3);

        let mut sound_endpoint = SoundAPI::new();
        let h = sound_endpoint.load_buffer(s);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h, false);

        std::thread::sleep(Duration::from_millis(3000));
    }
}

#[test]
fn test_tanh() {
    dbg!(0.5f32.tanh());
    dbg!(1.0f32.tanh());
    dbg!(2.0f32.tanh());
    dbg!(3.0f32.tanh());
    dbg!(4.0f32.tanh());
}

// do a test where its sin 0..1, then apply varying degrees of tanh and concat buffers to see