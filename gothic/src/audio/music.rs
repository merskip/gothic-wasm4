use wasm4::audio::{ADSRDuration, Audio, Channel, Duration, DutyCycle, Flags, Frequency, Pan, Volume};
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::updatable::Updatable;

#[derive(Copy, Clone)]
pub enum Bpm {
    Bpm1800 = 1,
    Bpm900 = 2,
    Bpm600 = 3,
    Bpm450 = 4,
    Bpm360 = 5,
    Bpm300 = 6,
    Bpm257 = 7,
    Bpm225 = 8,
    Bpm200 = 9,
    Bpm180 = 10,
    Bpm164 = 11,
    Bpm150 = 12,
    Bpm138 = 13,
    Bpm129 = 14,
    Bpm120 = 15,
}

pub struct Octave(f32);

impl Octave {
    pub const OCT_0: Self = Self(21.83);
    pub const OCT_1: Self = Self(43.65);
    pub const OCT_2: Self = Self(87.31);
    pub const OCT_3: Self = Self(174.61);
    pub const OCT_4: Self = Self(349.23);
    pub const OCT_5: Self = Self(698.46);
    pub const OCT_6: Self = Self(1396.91);
    pub const OCT_7: Self = Self(2793.83);
    pub const OCT_8: Self = Self(5587.65);

    fn as_f32(&self) -> f32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NoteLength {
    EIGHT = 8000,
    QUARTER = 4000,
    HALF = 2000,
    WHOLE = 1000,
    DOUBLE = 500,
    QUAD = 250,
}

impl NoteLength {
    pub fn for_bpm(&self, bpm: Bpm) -> u32 {
        return NoteLength::QUARTER as u32 * bpm as u32 / *self as u32;
    }
}

pub struct Note {
    base_frequency_factor: f32,
    length: NoteLength,
}

impl Note {
    pub const fn c(length: NoteLength) -> Self { Self { base_frequency_factor: 0.749162, length } }
    pub const fn c_s(length: NoteLength) -> Self { Self { base_frequency_factor: 0.793689, length } }
    pub const fn d(length: NoteLength) -> Self { Self { base_frequency_factor: 0.840878, length } }
    pub const fn d_s(length: NoteLength) -> Self { Self { base_frequency_factor: 0.890903, length } }
    pub const fn e(length: NoteLength) -> Self { Self { base_frequency_factor: 0.943876, length } }
    pub const fn f(length: NoteLength) -> Self { Self { base_frequency_factor: 1.000000, length } }
    pub const fn f_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.059445, length } }
    pub const fn g(length: NoteLength) -> Self { Self { base_frequency_factor: 1.122469, length } }
    pub const fn g_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.189188, length } }
    pub const fn a(length: NoteLength) -> Self { Self { base_frequency_factor: 1.259915, length } }
    pub const fn a_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.334822, length } }
    pub const fn b(length: NoteLength) -> Self { Self { base_frequency_factor: 1.414197, length } }
}

pub struct Sound {
    pub beat: isize,
    pub octave: Octave,
    pub note: Note,
    pub volume: u32,
}

pub struct Instrument {
    channel: Channel,
    duty_cycle: DutyCycle,
    attack: u8,
    release: u8,
}

impl Instrument {
    pub const TRUMPET: Self = Self { channel: Channel::Pulse1, duty_cycle: DutyCycle::ThreeQuarters, attack: 2, release: 150 };
    pub const DRUM: Self = Self { channel: Channel::Noise, duty_cycle: DutyCycle::OneHalf, attack: 0, release: 15 };
}

pub struct ClipInstrumentData {
    pub instrument: Instrument,
    pub sound_list: &'static [Sound],
}

pub struct ClipData {
    pub bpm: Bpm,
    pub length: isize,
    pub instrument_data: [ClipInstrumentData; 2],
}

pub struct Music {
    audio: Audio,
    current_clip: Option<&'static ClipData>,
    beat_counter: isize,
    frame_counter: isize,
}

static mut MUSIC: Music = Music {
    audio: Audio::shared(),
    current_clip: None,
    beat_counter: -1,
    frame_counter: -1,
};

impl Updatable for Music {
    fn update(&mut self, _inputs: &Inputs, _dispatcher: &mut Dispatcher) {
        if let Some(clip_data) = self.current_clip {
            self.update_play_clip(clip_data)
        }
    }
}

impl Music {
    pub fn shared() -> &'static mut Self {
        unsafe { &mut MUSIC }
    }

    pub fn play_clip(&mut self, clip: &'static ClipData) {
        self.current_clip = Some(clip);
    }

    fn update_play_clip(&mut self, clip_data: &ClipData) {
        self.frame_counter += 1;

        let beat = self.frame_counter / clip_data.bpm as isize;

        if beat != self.beat_counter {
            for instrument_data in &clip_data.instrument_data {
                let sound_list_len = instrument_data.sound_list.len() as isize;
                let sound = instrument_data.sound_list
                    .iter()
                    .find(|&sound| beat % sound_list_len == sound.beat);

                if let Some(sound) = sound {
                    self.play_sound(clip_data.bpm, &instrument_data.instrument, sound);
                }
            }

            self.beat_counter = beat;
        }
    }

    fn play_sound(&self, bpm: Bpm, instrument: &Instrument, sound: &Sound) {
        self.audio.tone(
            Frequency::constant((sound.octave.as_f32() * sound.note.base_frequency_factor) as u16),
            ADSRDuration::new(
                Duration::from_frames(instrument.attack),
                Duration::from_frames(0),
                Duration::from_frames(sound.note.length.for_bpm(bpm) as u8),
                Duration::from_frames(instrument.release),
            ),
            Volume::constant(sound.volume as u8),
            Flags::new(
                instrument.channel.into(),
                instrument.duty_cycle.into(),
                Pan::default(),
            ),
        )
    }
}
