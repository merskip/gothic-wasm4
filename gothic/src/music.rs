use wasm4::audio::{ADSRDuration, Audio, Channel, Duration, DutyCycle, Flags, Frequency, Pan, Volume};

#[derive(Copy, Clone)]
enum Bpm {
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

struct Octave(f32);

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
enum NoteLength {
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

struct Note {
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

struct Sound {
    beat: isize,
    octave: Octave,
    note: Note,
    volume: u32,
}

struct Instrument {
    channel: Channel,
    duty_cycle: DutyCycle,
    attack: u8,
    release: u8,
}

impl Instrument {
    pub const TRUMPET: Self = Self { channel: Channel::Pulse1, duty_cycle: DutyCycle::ThreeQuarters, attack: 2, release: 150 };
    pub const DRUM: Self = Self { channel: Channel::Noise, duty_cycle: DutyCycle::OneHalf, attack: 0, release: 15 };
}

struct ClipInstrumentData {
    instrument: Instrument,
    sound_list: &'static [Sound],
}

struct ClipData {
    bpm: Bpm,
    instrument_data: [ClipInstrumentData; 2],
}

impl ClipData {
    pub fn data_for_clip(clip: Clip) -> ClipData {
        return ClipData::MAIN_THEME_DATA;
    }
    const MAIN_THEME_DATA: ClipData = ClipData {
        bpm: Bpm::Bpm164,
        instrument_data: [
            ClipInstrumentData {
                instrument: Instrument::TRUMPET,
                sound_list: &[
                    Sound { beat: 0, octave: Octave::OCT_3, note: Note::c(NoteLength::DOUBLE), volume: 100 },
                    Sound { beat: 8, octave: Octave::OCT_3, note: Note::g(NoteLength::DOUBLE), volume: 100 },
                    Sound { beat: 14, octave: Octave::OCT_3, note: Note::f(NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 15, octave: Octave::OCT_3, note: Note::g(NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 16, octave: Octave::OCT_3, note: Note::g_s(NoteLength::QUAD), volume: 100 },
                    Sound { beat: 28, octave: Octave::OCT_3, note: Note::g(NoteLength::HALF), volume: 100 },
                    Sound { beat: 30, octave: Octave::OCT_3, note: Note::f(NoteLength::HALF), volume: 100 },
                    Sound { beat: 32, octave: Octave::OCT_3, note: Note::g(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 36, octave: Octave::OCT_3, note: Note::c(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 40, octave: Octave::OCT_3, note: Note::g(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 44, octave: Octave::OCT_3, note: Note::a(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 48, octave: Octave::OCT_3, note: Note::a_s(NoteLength::QUAD), volume: 100 },
                    Sound { beat: 64, octave: Octave::OCT_4, note: Note::c(NoteLength::DOUBLE), volume: 100 },
                    Sound { beat: 72, octave: Octave::OCT_3, note: Note::a_s(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 76, octave: Octave::OCT_3, note: Note::g_s(NoteLength::WHOLE), volume: 100 },
                    Sound { beat: 80, octave: Octave::OCT_3, note: Note::a_s(NoteLength::DOUBLE), volume: 100 },
                    Sound { beat: 88, octave: Octave::OCT_3, note: Note::d_s(NoteLength::DOUBLE), volume: 100 },
                    Sound { beat: 94, octave: Octave::OCT_3, note: Note::f(NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 95, octave: Octave::OCT_3, note: Note::d_s(NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 96, octave: Octave::OCT_3, note: Note::f(NoteLength::QUAD), volume: 100 },
                ],
            },
            ClipInstrumentData {
                instrument: Instrument::DRUM,
                sound_list: &[
                    Sound { beat: 0,  octave: Octave::OCT_3, note: Note::c(NoteLength::EIGHT), volume: 100 },
                    Sound { beat: 2,  octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 3,  octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 4,  octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 6,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 8,  octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 9,  octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14, octave: Octave::OCT_3, note: Note::c(NoteLength::EIGHT), volume: 100 },
                ],
            },
        ],
    };
}

#[derive(PartialEq)]
pub enum Clip {
    MainTheme,
    TestTheme,
}

pub struct Music {
    audio: Audio,
    current_clip: Clip,
    beat_counter: isize,
    frame_counter: isize,
}

static mut MUSIC: Music = Music {
    audio: Audio::shared(),
    current_clip: Clip::MainTheme,
    beat_counter: -1,
    frame_counter: -1,
};

impl Music {
    pub fn play_clip(clip: Clip) {
        unsafe {
            MUSIC.play(clip);
        }
    }

    fn play(&mut self, clip: Clip) {
        if self.current_clip != clip {
            self.beat_counter = -1;
            self.frame_counter = -1;
        }

        self.frame_counter += 1;

        let clip_data: ClipData = ClipData::data_for_clip(clip);
        let beat: isize = self.frame_counter / clip_data.bpm as isize;

        if beat != self.beat_counter {
            for instrument_data in clip_data.instrument_data {
                let sound = instrument_data.sound_list.iter().find(|&x| x.beat == beat);
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
