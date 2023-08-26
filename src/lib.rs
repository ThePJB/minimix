mod rng;
mod load_wav;
mod mixer;
mod sound_api;
mod sound_library;
mod sound_buffer;
mod track;

pub use sound_api::*;

mod test {
    use std::f32::consts::PI;
    use crate::sound_api::*;
    use crate::sound_buffer::*;
    use std::time::Duration;

    #[test]
    pub fn test_beats() {
        // hmmmm ring buf as two frequencies vary? that would also be a good sound intuitionizer demo
        // like a 2d: matrix, identity line will probs be sin^2
        let mut sound_endpoint = SoundAPI::new();
        let mut buf1 = SoundBuffer::sine(5000, 2.0*PI*440.0);
        let mut buf2 = SoundBuffer::sine(5000, 2.0*PI*457.0);
        buf2.elementwise_mul(&buf1);
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
}