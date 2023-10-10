// use crate::wav::*;
// use crate::interp::*;
// use minimg::*;
// use minvect::*;

use std::f32::consts::PI;
use minimix::interp::*;
use minimix::wav::*;
use minvect::*;
use minimg::*;

#[test]
fn test_lan() {
    let wn = 2.0 * PI * 440.0 / 44100.0;
    let s: Vec<f32> = (0..800)
        .map(|n| wn * n as f32)
        .map(|phase| phase.sin())
        .collect();

    let s2 = lanczos_interp(&s, 1.31, 3);
    let s3 = lanczos_interp(&s, 2.0, 3);

    write_wav("s1.wav", &s, 44100);
    write_wav("s2.wav", &s2, 44100);
    write_wav("s3.wav", &s3, 44100);

    let mut im = ImageBuffer::new(512, 512);
    im.plot(&s2, vec4(0.0, 0.0, 1.0, 1.0), 2);
    im.plot(&s3, vec4(1.0, 0.0, 0.0, 1.0), 2);
    im.plot(&s, vec4(0.0, 1.0, 0.0, 1.0), 2);

    dbg!(s2.len(), s3.len(), s2.into_iter().reduce(f32::max), s3.into_iter().reduce(f32::max));

    im.dump_to_file("test.png");
}