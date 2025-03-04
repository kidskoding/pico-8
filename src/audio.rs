use sdl3::audio::{AudioCallback, AudioDevice, AudioSpec};

struct SquareWave {
    phase: f32,
    frequency: f32,
}

impl AudioCallback<f32> for SquareWave {
    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            self.phase == self.frequency / 44100.0;
            if self.phase > 1.0 {
                self.phase -= 2.0;
            }
            *sample = if self.phase > 0.0 { 0.5 } else { -0.5 }
        }
    }
}

fn play_sound(sdl_context: &sdl3::Sdl) {
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpec {
        freq: Some(44100),
        format: Some(sdl3::audio::AudioFormat::F32BE),
        channels: Some(1),
    };
    let device: AudioDevice = audio_subsystem.open_playback_device(&desired_spec).unwrap();
    device.resume();
}