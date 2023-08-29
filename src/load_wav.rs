use riff_wave::*;
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;

// pub fn load_wav(path: &str) -> Option<Vec<f32>> {
//     let file = std::fs::File::open(path).unwrap();
//     let wave_reader = WaveReader::new(file).unwrap();
//     let num_channels = wave_reader.pcm_format.num_channels;
//     let bits_per_sample = wave_reader.pcm_format.bits_per_sample;
//     dbg!(&wave_reader.pcm_format);
//     if num_channels == 1 {
//         if bits_per_sample == 8 {
//             return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / 255.0 * 2.0 - 1.0).collect());
//         } 
//         if bits_per_sample == 16 {
//             return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i16::MAX as f32).collect());
//         } 
//         if bits_per_sample == 24 {
//             return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / MAX_I24_VALUE as f32).collect());
//         } 
//         if bits_per_sample == 32 {
//             return Some(wave_reader.into_inner().bytes().map(|x| x.unwrap() as f32 / i32::MAX as f32).collect());
//         }
//     }
//     dbg!("unsupported");

//     None
// }

pub fn load_wav(path: &str) -> Option<Vec<f32>> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut wav_data = Vec::new();

    let wave_reader = riff_wave::WaveReader::new(&mut file).ok()?;
    let num_channels = wave_reader.pcm_format.num_channels;
    let bits_per_sample = wave_reader.pcm_format.bits_per_sample;

    if num_channels == 1 {
        if bits_per_sample == 16 {
            let mut buffer = [0; 2]; // 16-bit buffer
            while let Ok(()) = file.read_exact(&mut buffer) {
                let sample_i16 = i16::from_le_bytes(buffer); // Convert little-endian bytes to i16
                let sample_f32 = sample_i16 as f32 / i16::MAX as f32;
                wav_data.push(sample_f32);
            }
            return Some(wav_data);
        }
    }

    None
}

pub fn write_wav(outfile: &str, samples: &Vec<f32>, sample_rate: u32) {
    let file = File::create(outfile).unwrap();
	let writer = BufWriter::new(file);
	let mut wave_writer = WaveWriter::new(1, sample_rate, 16, writer).unwrap();
    for s in samples {
        wave_writer.write_sample_i16((s * i16::MAX as f32) as i16).unwrap();
    }
}


#[test]
pub fn test_save_load() {
    let fs = 44100u32;
    let samples = (0..1000).map(|x| x as f32 / fs as f32).collect();
    write_wav("test_save_load.wav", &samples, fs);
    std::thread::sleep(std::time::Duration::from_millis(10));
    let samples2 = load_wav("test_save_load.wav");
    assert!(samples2.is_some());
    let samples2 = samples2.unwrap();
    assert_eq!(samples.len(), samples2.len());
    for i in 0..samples.len() {
        assert_eq!(samples[i], samples2[i]);
    }
}