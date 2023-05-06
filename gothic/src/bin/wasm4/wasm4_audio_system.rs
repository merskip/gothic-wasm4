use gothic::audio::audio_system::AudioSystem;
use gothic::audio::music::Instrument;
use wasm4::audio::*;

pub struct Wasm4AudioSystem;

const SHARED_AUDIO: Audio = Audio::shared();

impl AudioSystem for Wasm4AudioSystem {
    fn tone(&self, instrument: Instrument, frequency: u32, duration: u32, volume: f32) {
        let instrument_details = InstrumentDetails::from(instrument);
        SHARED_AUDIO.tone(
            Frequency::constant(frequency as u16),
            ADSRDuration::new(
                instrument_details.attack_duration,
                0u8,
                duration as u8,
                instrument_details.release_duration,
            ),
            Volume::constant((volume * 100.0) as u8),
            Flags::new(
                instrument_details.channel,
                instrument_details.duty_cycle,
                Pan::default(),
            ),
        );
    }
}


struct InstrumentDetails {
    channel: Channel,
    duty_cycle: DutyCycle,
    attack_duration: ToneDuration,
    release_duration: ToneDuration
}

impl InstrumentDetails {
    const TRUMPET_1: Self = Self {
        channel: Channel::Pulse1,
        duty_cycle: DutyCycle::ThreeQuarters,
        attack_duration: 2,
        release_duration: 150
    };

    const TRUMPET_2: Self = Self {
        channel: Channel::Pulse2,
        duty_cycle: DutyCycle::OneHalf,
        attack_duration: 2,
        release_duration: 15
    };

    const DRUM: Self = Self {
        channel: Channel::Noise,
        duty_cycle: DutyCycle::OneHalf,
        attack_duration: 0,
        release_duration: 15
    };
}

impl From<Instrument> for InstrumentDetails {
    fn from(value: Instrument) -> Self {
        match value {
            Instrument::Trumpet => Self::TRUMPET_1,
            Instrument::Trumpet2 => Self::TRUMPET_2,
            Instrument::Drum => Self::DRUM,
        }
    }
}
