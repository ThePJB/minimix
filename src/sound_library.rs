pub struct SoundBuffer {
    id: SoundHandle,
    samples: Vec<f32>,
}

pub struct SoundLibrary {
    sounds: Vec<SoundBuffer>,
}

impl SoundLibrary {
    pub fn new() -> Self {

    }
}