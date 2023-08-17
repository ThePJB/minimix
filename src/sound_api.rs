
use cpal::traits::*;
use ringbuf::*;
use crate::rng::*;
use crate::mixer::*;

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
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to retrieve default output device");
        println!("Output device : {}", device.name().expect("couldnt get device name (??? idk)"));
        let config = device.default_output_config().expect("failed to get default output config");
        println!("Default output config : {:?}", config);
        let sample_rate = config.sample_rate().0;
        let sample_format = config.sample_format();
        let channels = config.channels();

        let mixer = Mixer {
            sample_rate: sample_rate as usize,
            nchannels: channels as usize,
            rng: Rng::new_random(),
            sounds: vec![],
            channels: vec![],
            command_consumer: cons,
        };

        let output_callback = move |output: &mut [f32], info: &cpal::OutputCallbackInfo| {
            mixer.handle_commands();
            mixer.write_samples(output);
            for frame in output.chunks_mut(channels as usize) {
                for sample in frame.iter_mut() {
                    *sample = mixer.tick();
                }
            }
        };

        let config = cpal::StreamConfig {
            channels: channels,
            sample_rate: config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(&config, output_callback, |_| panic!("error"), None),
            _ => panic!("unsupported"),
        }.expect("failed to make stream");
        stream.play().expect("failed to play stream");
        SoundAPI {
            rng: Rng::new_random(),
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
        let params = SoundDesc {
            vol: 0.5,
            t_begin_transition: 0,
            t_end_transition: 0,
            t_fade_in: 0,
            t_fade_out: 0,
            repeat: false,
            a: sound,
            b: None,
        };
        self.play_raw(params)
    }

    // maybe fade in or fade out in percentage of the sound?
    // and similarly for crossover point
    // interesting sampling schemes plz like ping pong or repeat or heterodyne or 
    // with procedural shit u could 
    // todo add convolution with an impulse lol.
    pub fn play_raw(&mut self, params: SoundDesc) -> PlayingSoundHandle {
        let id = self.rng.next_u64();
        let command = Command::Play { params, id };
        self.prod.push(command);
        id
    }

    pub fn stop(&mut self, id: PlayingSoundHandle) {
        self.prod.push(Command::Stop { id });
    }
}
