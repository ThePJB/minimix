
use cpal::traits::*;
use ringbuf::*;
use crate::rng::*;
use crate::mixer::*;
use crate::sound_buffer::*;
use crate::sound_library::*;

pub type SoundHandle = u64;
pub type PlayingSoundHandle = u64;

pub struct SoundDesc {
    pub repeat: bool,
    pub h: SoundHandle,
}

pub struct SoundAPI {
    rng: Rng,
    command_prod: Producer<Command>,
    sound_prod: Producer<Sound>,
    stream: cpal::Stream,
}

impl SoundAPI {
    pub fn new() -> Self {
        let (command_prod, command_cons) = RingBuffer::<Command>::new(200).split();
        let (sound_prod, sound_cons) = RingBuffer::<Sound>::new(20).split();
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to retrieve default output device");
        println!("Output device : {}", device.name().expect("couldnt get device name (??? idk)"));
        let config = device.default_output_config().expect("failed to get default output config");
        println!("Default output config : {:?}", config);
        let sample_rate = config.sample_rate().0;
        let sample_format = config.sample_format();
        let channels = config.channels();

        let mut mixer = Mixer::new(sample_rate as usize, channels as usize, command_cons, sound_cons);

        let output_callback = move |output: &mut [f32], info: &cpal::OutputCallbackInfo| {
            mixer.load_sounds();
            mixer.handle_commands();
            mixer.write_samples(output);
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
            command_prod,
            sound_prod,
        }
    }

    pub fn load_buffer(&mut self, buf: SoundBuffer) -> SoundHandle {
        let id = self.rng.next_u64();
        let s = Sound {
            id,
            buf,
        };
        self.sound_prod.push(s);
        id
    }

    pub fn play(&mut self, sound: SoundHandle, repeat: bool) -> PlayingSoundHandle {
        let params = SoundDesc {
            repeat,
            h: sound,
        };
        self.play_raw(params)
    }

    pub fn play_raw(&mut self, params: SoundDesc) -> PlayingSoundHandle {
        let id = self.rng.next_u64();
        let command = Command::Play { params, id };
        self.command_prod.push(command);
        id
    }

    pub fn stop(&mut self, id: PlayingSoundHandle) {
        self.command_prod.push(Command::Stop { id });
    }
}
