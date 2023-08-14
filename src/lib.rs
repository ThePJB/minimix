use cpal::traits::*;
use ringbuf::*;
use riff_wave::*;
use std::io::Read;
mod rng;
use rng::*;

pub type SoundHandle = usize;

pub struct PlayingSoundHandle {
    // generational index
}
pub struct SoundBuffer {
    samples: Vec<f32>,
}
pub struct Channel {
    t: u64,
    id: PlayingSoundHandle,
    desc: SoundDesc,
}

impl Channel {
    pub fn tick(&mut self) -> f32 {
        if let Some(other_id) = self.desc.b {
            let id = self.desc.a;
            let samp_a = self.
        }
        self.t += 1;
    }
}

pub struct Mixer {
    rng: Rng,
    sounds: Vec<SoundBuffer>,
    channels: Vec<Channel>,
    command_consumer: Consumer<Command>,

}
impl Mixer {
    pub fn tick(&mut self) {

    }
}

// mixer goes in SampleRequestOptions. now its coming back to me.
// holy shit. maybe when im not decrepit af

// needs stops as well

// random u64 IDs and linear search through channels cunt. its the only way.

// honestly do you just make a string for each one lol. like impact.wav/vol:0.4/fadeout:20/

// almost
// sound buffer needs random u64 as well

pub enum Command {
    Load {
        path: String,
        id: SoundHandle,
    },
    Play {
        params: SoundDesc,
        id: PlayingSoundHandle,
    },
    Stop {
        id: PlayingSoundHandle,
    }

}

// how am i gonna do handling references to the playing sounds
// 1 channel going back from audio thread
// 2 tell audio thread what reference im gonna use - named channels
// 3 could do it all string based or even name based - minimal ui, kinda icky, mm but kinda based
// 4 nah but multiple instances tho yeah.


pub struct SoundDesc {
    vol: f32,
    t_begin_transition: u64,
    t_end_transition: u64,
    t_fade_in: u64,
    t_fade_out: u64,
    repeat: bool,
    a: SoundHandle,
    b: Option<SoundHandle>,
}

pub fn load_wav(path: &str) -> Option<Vec<f32>> {
    let file = std::fs::File::open(path).unwrap();
    let wave_reader = WaveReader::new(file).unwrap();
    let format = wave_reader.pcm_format;
    dbg!(format);
    if format.num_channels == 1 {
        if format.bits_per_sample == 8 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / 255.0 * 2.0 - 1.0).collect());
        } 
        if format.bits_per_sample == 16 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i16::MAX as f32).collect());
        } 
        if format.bits_per_sample == 24 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / MAX_I24_VALUE as f32).collect());
        } 
        if format.bits_per_sample == 32 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i32::MAX as f32).collect());
        }
    }
    dbg!("unsupported");

    None
}

pub struct SoundController {
    rng: Rng,
    prod: Producer<Command>,
    stream: cpal::Stream,
}

impl SoundController {
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

    pub fn load_sound(&mut self, path: &str) -> Option<SoundHandle> {
        let token = self.sounds.len();
        if let Some(samples) = load_wav(path) {
            self.sounds.push(SoundBuffer { samples });
            return Some(token);
        }
        None
    }

    pub fn play(&mut self, sound: SoundHandle, vol: f32, repeat: bool) -> PlayingSoundHandle {

    }

    pub fn play_fade(&mut self, start: SoundHandle, end: SoundHandle, fade_start: f32, fade_end: f32, vol: f32, repeat: bool) -> PlayingSoundHandle {

    }

    pub fn stop(&mut self, handle: PlayingSoundHandle) {

    }
}

// ====================
// Audio stuff
// ====================
// 0 : kick drum
// 1 : sad ding

fn sample_next(o: &mut SampleRequestOptions) -> f32 {
    let mut acc = 0.0;
    let mut idx = o.sounds.len();
    loop {
        if idx == 0 {
            break;
        }
        idx -= 1;

        if o.sounds[idx].wait > 0.0 {
            o.sounds[idx].wait -= 1.0/44100.0;
            continue;
        }

        o.sounds[idx].elapsed += 1.0/44100.0;
        o.sounds[idx].remaining -= 1.0/44100.0;

        let t = o.sounds[idx].elapsed;

        if o.sounds[idx].remaining < 0.0 {
            o.sounds.swap_remove(idx);
            continue;
        }
        if o.sounds[idx].id == 0 {
            o.sounds[idx].magnitude *= 0.999;

            let f = o.sounds[idx].frequency;
            let f_trans = f*3.0;

            let t_trans = 1.0/(2.0*PI*f_trans);

            if o.sounds[idx].elapsed < t_trans {
                o.sounds[idx].phase += f_trans*2.0*PI*1.0/o.sample_rate;
            } else {
                o.sounds[idx].phase += f*2.0*PI*1.0/o.sample_rate;
            }
            // o.sounds[idx].phase += f*2.0*PI*1.0/o.sample_rate;

            //o.sounds[idx].phase = o.sounds[idx].phase % 2.0*PI; // this sounds really good lol

            acc += (o.sounds[idx].phase).sin() * o.sounds[idx].magnitude
        } else if o.sounds[idx].id == 1 {
            o.sounds[idx].magnitude *= o.sounds[idx].mag_exp;
            o.sounds[idx].frequency *= o.sounds[idx].freq_exp;
            o.sounds[idx].phase += o.sounds[idx].frequency*2.0*PI*1.0/o.sample_rate;
            acc += (o.sounds[idx].phase).sin() * o.sounds[idx].magnitude
        } else if o.sounds[idx].id == 2 {
            o.sounds[idx].magnitude *= o.sounds[idx].mag_exp;
            acc += krand(o.sounds[idx].samp as usize) * o.sounds[idx].magnitude;
        }
        o.sounds[idx].samp += 1;
    }
    acc
}



pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub nchannels: usize,
    pub channel: Consumer<Sound>,
    pub sounds: Vec<Sound>,
}

pub fn stream_setup_for<F>(on_sample: F, channel: Consumer<Sound>) -> Result<cpal::Stream, anyhow::Error>
where
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => stream_make::<f32, _>(&device, &config.into(), on_sample, channel),
        cpal::SampleFormat::I16 => stream_make::<i16, _>(&device, &config.into(), on_sample, channel),
        cpal::SampleFormat::U16 => stream_make::<u16, _>(&device, &config.into(), on_sample, channel),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

pub fn stream_make<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    on_sample: F,
    channel: Consumer<Sound>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: cpal::Sample,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let sample_rate = config.sample_rate.0 as f32;
    let nchannels = config.channels as usize;
    let mut request = SampleRequestOptions {
        sample_rate,
        nchannels,
        sounds: vec![],
        channel,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            on_window(output, &mut request, on_sample)
        },
        err_fn,
    )?;

    Ok(stream)
}

fn on_window<T, F>(output: &mut [T], request: &mut SampleRequestOptions, mut on_sample: F)
where
    T: cpal::Sample,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static,
{
    if let Some(sc) = request.channel.pop() {
        request.sounds.push(sc);
    }
    for frame in output.chunks_mut(request.nchannels) {
        let value: T = cpal::Sample::from::<f32>(&on_sample(request));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}