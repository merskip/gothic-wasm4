use crate::audio::music::Instrument;

pub trait AudioSystem {
    fn stop_all(&self);

    fn tone(&self, instrument: Instrument, frequency: u32, duration: u32, volume: f32);
}
