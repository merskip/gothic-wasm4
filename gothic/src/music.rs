use core::convert::TryInto;
use crate::music::NoteLength::{HALF, QUARTER};

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
    QUARTER = 4000,
    HALF = 2000,
    WHOLE = 1000,
    DOUBLE = 500,
    QUAD = 250
}
impl NoteLength {

    pub fn for_bpm(&self, bpm: Bpm) -> u32 {
        return QUARTER as u32 * bpm as u32 / *self as u32;
    }
}

struct Note {
    base_frequency_factor: f32,
    length: NoteLength
}
impl Note {
    pub const fn c  (length: NoteLength) -> Self { Self { base_frequency_factor: 0.749162, length } }
    pub const fn c_s(length: NoteLength) -> Self { Self { base_frequency_factor: 0.793689, length } }
    pub const fn d  (length: NoteLength) -> Self { Self { base_frequency_factor: 0.840878, length } }
    pub const fn d_s(length: NoteLength) -> Self { Self { base_frequency_factor: 0.890903, length } }
    pub const fn e  (length: NoteLength) -> Self { Self { base_frequency_factor: 0.943876, length } }
    pub const fn f  (length: NoteLength) -> Self { Self { base_frequency_factor: 1.000000, length } }
    pub const fn f_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.059445, length } }
    pub const fn g  (length: NoteLength) -> Self { Self { base_frequency_factor: 1.122469, length } }
    pub const fn g_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.189188, length } }
    pub const fn a  (length: NoteLength) -> Self { Self { base_frequency_factor: 1.259915, length } }
    pub const fn a_s(length: NoteLength) -> Self { Self { base_frequency_factor: 1.334822, length } }
    pub const fn b  (length: NoteLength) -> Self { Self { base_frequency_factor: 1.414197, length } }
}

struct Sound {
    beat: isize,
    octave: Octave,
    note: Note,
}

#[repr(u32)]
enum Channel {
    PULSE1 = 0,
    PULSE2 = 1,
    TRIANGLE = 2,
    NOISE = 3
}

struct Instrument {
    channel: Channel,
}
impl Instrument {
    pub const UNKNOWN: Self = Self { channel: Channel::PULSE1 };
    pub const DRUM: Self = Self { channel: Channel::NOISE };
}

struct ClipInstrumentData {
    instrument: Instrument,

    sound_list: [Sound; 22]
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
        bpm: Bpm::Bpm150,
        instrument_data: [
            ClipInstrumentData {
                instrument: Instrument::UNKNOWN,
                sound_list: [
                    Sound { beat: 0,   octave: Octave::OCT_3, note: Note::c  (NoteLength::DOUBLE) },
                    Sound { beat: 8,   octave: Octave::OCT_3, note: Note::g  (NoteLength::DOUBLE) },
                    Sound { beat: 14,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUARTER) },
                    Sound { beat: 15,  octave: Octave::OCT_3, note: Note::g  (NoteLength::QUARTER) },
                    Sound { beat: 16,  octave: Octave::OCT_3, note: Note::g_s(NoteLength::QUAD) },
                    Sound { beat: 28,  octave: Octave::OCT_3, note: Note::g  (NoteLength::HALF) },
                    Sound { beat: 30,  octave: Octave::OCT_3, note: Note::f  (NoteLength::HALF) },
                    Sound { beat: 32,  octave: Octave::OCT_3, note: Note::g  (NoteLength::WHOLE) },
                    Sound { beat: 36,  octave: Octave::OCT_3, note: Note::c  (NoteLength::WHOLE) },
                    Sound { beat: 40,  octave: Octave::OCT_3, note: Note::g  (NoteLength::WHOLE) },
                    Sound { beat: 44,  octave: Octave::OCT_3, note: Note::a  (NoteLength::WHOLE) },
                    Sound { beat: 48,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::QUAD) },
                    Sound { beat: 64,  octave: Octave::OCT_4, note: Note::c  (NoteLength::DOUBLE) },
                    Sound { beat: 72,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::WHOLE) },
                    Sound { beat: 76,  octave: Octave::OCT_3, note: Note::g_s(NoteLength::WHOLE) },
                    Sound { beat: 80,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::DOUBLE) },
                    Sound { beat: 88,  octave: Octave::OCT_3, note: Note::d_s(NoteLength::DOUBLE) },
                    Sound { beat: 94,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUARTER) },
                    Sound { beat: 95,  octave: Octave::OCT_3, note: Note::d_s(NoteLength::QUARTER) },
                    Sound { beat: 96,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD) },
                    Sound { beat: 104, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD) },
                    Sound { beat: 112, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD) },
                ],
            },
            ClipInstrumentData {
                instrument: Instrument::DRUM,
                sound_list: [
                    Sound { beat: 0,   octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 3,   octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 6,   octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 16,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 19,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 22,  octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 32,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 35,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 38,  octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 48,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 51,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 54,  octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 64,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 67,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 70,  octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 80,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 83,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 86,  octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 96,  octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                    Sound { beat: 99,  octave: Octave::OCT_2, note: Note::b  (NoteLength::QUARTER) },
                    Sound { beat: 102, octave: Octave::OCT_2, note: Note::a  (NoteLength::QUARTER) },
                    Sound { beat: 112, octave: Octave::OCT_3, note: Note::c  (NoteLength::QUARTER) },
                ],
            },
        ],
    };
}

#[derive(PartialEq)]
pub enum Clip {
    MainTheme,
    TestTheme
}

pub struct Music {
    current_clip: Clip,
    beat_counter: isize,
    frame_counter: isize,
}
static mut MUSIC: Music = Music {
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

        let clipData: ClipData = ClipData::data_for_clip(clip);
        let beat: isize = self.frame_counter / clipData.bpm as isize;

        if beat != self.beat_counter {

            for instrument_data in clipData.instrument_data {
                let sound = instrument_data.sound_list.iter().find(|&x| x.beat == beat);
                if sound.is_some() {
                    let sound_unwrapped = sound.unwrap();
                    wasm4::tone((sound_unwrapped.octave.as_f32() * sound_unwrapped.note.base_frequency_factor) as u32,
                                sound_unwrapped.note.length.for_bpm(clipData.bpm),
                                100,
                                instrument_data.instrument.channel as u32);
                }
            }

            self.beat_counter = beat;
        }
    }
}
