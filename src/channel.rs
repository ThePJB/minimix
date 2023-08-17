
pub struct Channel {
    t: u64,
    t_max: u64,
    id: PlayingSoundHandle,
    desc: SoundDesc,
}

// what about a_index, b_index
// a_index a_index fade? why not bra
// think dj decks
// yea each channel needs 2 'tracks' or whatever
// and give tracks whatever relative time u want
// manage times in seconds or what?




// !!!! playing track needs to be able to bake the index of its underlying track !!!!
// !!!! 2 Tracks to a channel!
// !!!! its kind of 2 tracks with volume lerp


// lolz a more mvp one where samples get loaded and moved in
// but the runtime loading and manipulating capabilities tho


// track.seek - could make music

// oh yeah and if you could record and replay tracks that would be sweet
// yea runtime manageable please

// 2 tracks to a stream, tracks with own offset (not t, t => offset)
// we could honestly move the sound sources in and have them be completely static ..... but dynamic
// just at creation time put the buffer 
// but i like the idea of addressing the buffers, supporting operations, crop etc.

// oh yea and race condition of loading wav file vs. playing wav file -- Just dont crash and its OK

impl Channel {
    // pub fn new(id: PlayingSoundHandle, desc: SoundDesc) {
    //     Channel {

    //     }
    // }
    pub fn tick(&mut self, mixer: &Mixer) -> f32 {
        let sd = self.desc;
        let b1 = self.get_sound_buffer(sd.a);
        let s1 = b1[self.t];
        if let Some(b) = sd.b {
            let t_transition = if self.t < sd.t_begin_transition {
                0.0
            } else if self.t < sd.t_end_transition {
                 (self.t - sd.t_begin_transition) as f32 /
                 (sd.t_end_transition - sd.t_begin_transition) as f32
            } else {
                1.0
            };
            let b2 = self.get_sound_buffer(b);
            let s2 = b2[self.t];
            

        } else {
            acc += sd.vol * b1[self.channels[i].t];

        }
        self.t += 1;
        if sd.repeat {
            self.t = self.t % self.t_max;
        }
    }
}