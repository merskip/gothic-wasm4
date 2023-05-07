use std::mem::MaybeUninit;
use std::time::Duration;

use rodio::{OutputStream, OutputStreamHandle, Sink};

use gothic::audio::audio_system::AudioSystem;
use gothic::audio::music::Instrument;
use crate::adsr_envelope::SourceADSREnvelope;

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

        let trumpet1_sink = Sink::try_new(&steam_handle).unwrap();
        let trumpet2_sink = Sink::try_new(&steam_handle).unwrap();
        let drum_sink = Sink::try_new(&steam_handle).unwrap();

        for sink in [&trumpet1_sink, &trumpet2_sink, &drum_sink] {
            sink.set_volume(0.2);
        }

        unsafe {
            CHANNEL_TRUMPET_1.write(InstrumentChannel {
                sink: trumpet1_sink,
                attack: Duration::from_millis(2 * 16),
                release: Duration::from_millis(150 * 16),
            });
            CHANNEL_TRUMPET_2.write(InstrumentChannel {
                sink: trumpet2_sink,
                attack: Duration::from_millis(2 * 16),
                release: Duration::from_millis(15 * 16),
            });
            CHANNEL_DRUM.write(InstrumentChannel {
                sink: drum_sink,
                attack: Duration::from_millis(0 * 16),
                release: Duration::from_millis(15 * 16),
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

    fn tone(&self, instrument: Instrument, frequency: u32, duration: u32, volume: f32) {
        let instrument_channel = unsafe {
            match instrument {
                Instrument::Trumpet1 => CHANNEL_TRUMPET_1.assume_init_ref(),
                Instrument::Trumpet2 => CHANNEL_TRUMPET_2.assume_init_ref(),
                Instrument::Drum => CHANNEL_DRUM.assume_init_ref(),
            }
        };

        let sink = &instrument_channel.sink;

        match instrument {
            Instrument::Trumpet1 => {
                let source = SquareWave::new(frequency as f32, 0.75)
                    .adsr_envelope(
                        instrument_channel.attack,
                        Duration::from_millis(0),
                        Duration::from_millis(duration as u64 * 16),
                        instrument_channel.release,
                        volume,
                        volume
                    );

                sink.stop();
                sink.append(source);
            }

            Instrument::Trumpet2 => {
                let source = SquareWave::new(frequency as f32, 0.5)
                    .adsr_envelope(
                        instrument_channel.attack,
                        Duration::from_millis(0),
                        Duration::from_millis(duration as u64 * 16),
                        instrument_channel.release,
                        volume,
                        volume
                    );

                sink.stop();
                sink.append(source);
            }

            Instrument::Drum => {
                let source = NoiseWave::new(frequency as f32)
                    .adsr_envelope(
                        instrument_channel.attack,
                        Duration::from_millis(0),
                        Duration::from_millis(duration as u64 * 16),
                        instrument_channel.release,
                        volume,
                        volume
                    );

                sink.stop();
                sink.append(source);
            },
        }
    }
}

struct InstrumentChannel {
    pub sink: Sink,
    pub attack: Duration,
    pub release: Duration,
}
