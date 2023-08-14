
use cpal::traits::*;
use ringbuf::*;
use crate::rng::*;
use crate::cpal_setup::*;

pub type SoundHandle = u64;
pub type PlayingSoundHandle = u64;

pub struct SoundDesc {
    pub vol: f32,
    pub t_begin_transition: u64,
    pub t_end_transition: u64,
    pub t_fade_in: u64,
    pub t_fade_out: u64,
    pub repeat: bool,
    pub a: SoundHandle,
    pub b: Option<SoundHandle>,
}

pub struct SoundAPI {
    rng: Rng,
    prod: Producer<Command>,
    stream: cpal::Stream,
}

impl SoundAPI {
    pub fn new() -> Self {
        let rb = RingBuffer::<Command>::new(200);
        let (mut prod, mut cons) = rb.split();
        let stream = stream_setup_for(sample_next, cons).expect("no can make stream");
        stream.play().expect("no can play stream");
        SoundController {
            rng: Rng::new_random(),
            sounds: vec![],
            stream,
            prod,
        }
    }

    pub fn load_sound(&mut self, path: String) -> SoundHandle {
        let id = self.rng.next_u64();
        self.prod.push(Command::Load { path, id });
        id
    }

    pub fn play(&mut self, sound: SoundHandle, vol: f32, repeat: bool) -> PlayingSoundHandle {

    }

    pub fn play_fade(&mut self, start: SoundHandle, end: SoundHandle, fade_start: f32, fade_end: f32, vol: f32, repeat: bool) -> PlayingSoundHandle {

    }

    pub fn stop(&mut self, handle: PlayingSoundHandle) {

    }
}
