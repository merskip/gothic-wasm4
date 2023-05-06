use windows::Win32::System::Diagnostics::Debug::Beep;
use gothic::audio::audio_system::AudioSystem;
use gothic::audio::music::Instrument;

pub struct WindowsAudioSystem;



impl AudioSystem for WindowsAudioSystem {
    fn tone(&self, instrument: Instrument, frequency: u32, duration: u32, _volume: f32) {
        unsafe {
            Beep(frequency, duration + Self::extra_duration(instrument));
        }
    }
}

impl WindowsAudioSystem {

    fn extra_duration(instrument: Instrument) -> u32 {
        // attack + release
        match instrument {
            Instrument::Trumpet => 2 + 150,
            Instrument::Trumpet2 => 2 + 150,
            Instrument::Drum => 0 + 15,
        }
    }
}