use riff_wave::*;
use std::io::Read;

pub fn load_wav(path: &str) -> Option<Vec<f32>> {
    let file = std::fs::File::open(path).unwrap();
    let wave_reader = WaveReader::new(file).unwrap();
    let num_channels = wave_reader.pcm_format.num_channels;
    let bits_per_sample = wave_reader.pcm_format.bits_per_sample;
    dbg!(&wave_reader.pcm_format);
    if num_channels == 1 {
        if bits_per_sample == 8 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / 255.0 * 2.0 - 1.0).collect());
        } 
        if bits_per_sample == 16 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i16::MAX as f32).collect());
        } 
        if bits_per_sample == 24 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / MAX_I24_VALUE as f32).collect());
        } 
        if bits_per_sample == 32 {
            return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i32::MAX as f32).collect());
        }
    }
    dbg!("unsupported");

    None
}
