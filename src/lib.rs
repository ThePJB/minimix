mod rng;
mod load_wav;
mod mixer;
mod sound_api;
mod sound_library;
mod track;
mod signal;
mod signal_synth;

pub use sound_api::*;

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
        let mut buf = Signal::sine(5000, 2.0 * PI * 100.0, 44100.0);
        let h1 = sound_endpoint.load_buffer(buf);
        std::thread::sleep(Duration::from_millis(100));
        sound_endpoint.play(h1, false); // sum ting wong it no play
        std::thread::sleep(Duration::from_secs(1));
    }
}