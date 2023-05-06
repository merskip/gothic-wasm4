use std::mem::MaybeUninit;
use std::time::Duration;

use rodio::{OutputStream, OutputStreamHandle, Sink, source::Source};
use rodio::source::SineWave;

use gothic::audio::audio_system::AudioSystem;
use gothic::audio::music::Instrument;
use gothic::audio::music::Instrument::{Drum, Trumpet, Trumpet2};
use crate::noise_wave::NoiseWave;
use crate::square_wave::SquareWave;

pub struct WindowsAudioSystem;

static mut STREAM: MaybeUninit<OutputStream> = MaybeUninit::uninit();
static mut STREAM_HANDLER: MaybeUninit<OutputStreamHandle> = MaybeUninit::uninit();
static mut CHANNEL_TRUMPET_1: MaybeUninit<InstrumentChannel> = MaybeUninit::uninit();
static mut CHANNEL_TRUMPET_2: MaybeUninit<InstrumentChannel> = MaybeUninit::uninit();
static mut CHANNEL_DRUM: MaybeUninit<InstrumentChannel> = MaybeUninit::uninit();

impl WindowsAudioSystem {
    pub fn initialize() {
        let (stream, steam_handle) = OutputStream::try_default().unwrap();

        unsafe {
            CHANNEL_TRUMPET_1.write(InstrumentChannel {
                sink: Sink::try_new(&steam_handle).unwrap(),
                attack_duration: 2 * 16,
                release_duration: 150 * 16,
            });
            CHANNEL_TRUMPET_2.write(InstrumentChannel {
                sink: Sink::try_new(&steam_handle).unwrap(),
                attack_duration: 2 * 16,
                release_duration: 15 * 16,
            });
            CHANNEL_DRUM.write(InstrumentChannel {
                sink: Sink::try_new(&steam_handle).unwrap(),
                attack_duration: 0 * 16,
                release_duration: 15 * 16,
            });

            STREAM.write(stream);
            STREAM_HANDLER.write(steam_handle);
        }
    }
}

impl AudioSystem for WindowsAudioSystem {
    fn stop_all(&self) {
        // TODO
    }

    fn tone(&self, instrument: Instrument, frequency: u32, duration: u32, _volume: f32) {
        let channel = unsafe {
            match instrument {
                Instrument::Trumpet => &CHANNEL_TRUMPET_1,
                Instrument::Trumpet2 => &CHANNEL_TRUMPET_2,
                Instrument::Drum => &CHANNEL_DRUM,
            }.assume_init_ref()
        };

        if instrument == Trumpet || instrument == Trumpet2 {
            let sound_duration = channel.sound_duration(duration);
            let source = SquareWave::new(frequency as f32)
                .take_duration(sound_duration)
                .fade_in(Duration::from_millis(duration as u64 * 16));

            channel.sink.stop();
            channel.sink.append(source);
        } else if instrument == Drum {
            let sound_duration = channel.sound_duration(duration);
            let source = NoiseWave::new(frequency as f32)
                .take_duration(sound_duration)
                .fade_in(Duration::from_millis(duration as u64 * 16));

            channel.sink.stop();
            channel.sink.append(source);
        }
    }
}

struct InstrumentChannel {
    pub sink: Sink,
    pub attack_duration: u64,
    pub release_duration: u64,
}

impl InstrumentChannel {
    fn sound_duration(&self, sustain: u32) -> Duration {
        Duration::from_millis(
            (self.attack_duration * 16)
                + (sustain as u64 * 16)
                + (self.release_duration * 16)
        )
    }
}
