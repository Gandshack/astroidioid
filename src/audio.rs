use raylib::audio::{RaylibAudio, Sound};

pub struct Audio<'a> {
    pub audio_device: &'a RaylibAudio,
    pub shoot: Sound<'a>,
    pub explode: Sound<'a>,
    pub next_level: Sound<'a>,
}

impl<'a> Audio<'a> {
    pub fn new(audio_device: &'a RaylibAudio) -> Self {
        Self {
            audio_device,
            shoot: audio_device
                .new_sound("assets/audio/8bit_gunloop_explosion.wav")
                .unwrap(),
            explode: audio_device
                .new_sound("assets/audio/explosion.wav")
                .unwrap(),
            next_level: audio_device
                .new_sound("assets/audio/Jingle_Achievement_01.wav")
                .unwrap(),
        }
    }
}
