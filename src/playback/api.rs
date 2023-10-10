
use cpal::traits::*;
use ringbuf::*;
use crate::rng::*;
use super::mixer::*;
use crate::signal::*;
use super::sound_library::*;

pub use cpal::Device;

// pub enum DeviceOption {
//     Default,
//     Specific(usize),
// }

// pub fn list_devices() -> Vec<(u16, u32, String)> {
//     let mut res = Vec::new();
//     let host = cpal::default_host();
//     let devices = host.devices().expect("Couldn't get devices");
//     for device in devices.into_iter() {
//         let default_config = device.default_output_config();
//         if default_config.is_err() { continue };
//         let default_config = default_config.unwrap();
//         let name = device.name();
//         if name.is_err() { continue };
//         let name = name.unwrap();
//         let sample_rate = default_config.sample_rate().0;
//         let channels = default_config.channels();
//         res.push((channels, sample_rate, name));
//     }
//     res
// }

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BufferHandle {
    h: u64,
}
impl From<u64> for BufferHandle {
    fn from(h: u64) -> Self {
        BufferHandle {
            h
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TrackHandle {
    h: u64,
}
impl From<u64> for TrackHandle {
    fn from(h: u64) -> Self {
        TrackHandle {
            h
        }
    }
}

pub struct SoundDesc {
    pub repeat: bool,
    pub h: BufferHandle,
}

pub struct Minimixer {
    rng: Rng,
    command_prod: Producer<Command>,
    sound_prod: Producer<Sound>,
    stream: cpal::Stream,
}

impl Minimixer {
    pub fn new(device: Option<cpal::Device>) -> Self {
        let (command_prod, command_cons) = RingBuffer::<Command>::new(200).split();
        let (sound_prod, sound_cons) = RingBuffer::<Sound>::new(20).split();
        let host = cpal::default_host();
        let device = device.unwrap_or(host.default_output_device().expect("Failed to retrieve default output device"));
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
        Minimixer {
            rng: Rng::new_random(),
            stream,
            command_prod,
            sound_prod,
        }
    }

    pub fn load_buffer(&mut self, buf: Signal) -> BufferHandle {
        let id = self.rng.next_u64();
        let h = BufferHandle::from(id);
        let s = Sound {
            id: h,
            buf,
        };
        self.sound_prod.push(s);
        h
    }

    pub fn play(&mut self, sound: BufferHandle, repeat: bool) -> TrackHandle {
        let params = SoundDesc {
            repeat,
            h: sound,
        };
        self.play_raw(params)
    }
    
    pub fn play_raw(&mut self, params: SoundDesc) -> TrackHandle {
        let id = self.rng.next_u64();
        let h = TrackHandle::from(id);
        let command = Command::Play { params, id: h };
        self.command_prod.push(command);
        h
    }

    pub fn stop(&mut self, id: TrackHandle) {
        self.command_prod.push(Command::Stop { id });
    }
}
