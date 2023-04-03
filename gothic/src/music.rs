use core::convert::TryInto;
use crate::music::DutyCycle::{TONE_MODE1, TONE_MODE2, TONE_MODE3, TONE_MODE4};
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
    EIGHT = 8000,
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
    volume: u32
}

#[repr(u32)]
enum Channel {
    PULSE1 = 0,
    PULSE2 = 1,
    TRIANGLE = 2,
    NOISE = 3
}

#[repr(u32)]
enum DutyCycle {
    TONE_MODE1 = 0,
    TONE_MODE2 = 1,
    TONE_MODE3 = 2,
    TONE_MODE4 = 3
}

struct Instrument {
    channel: Channel,
    duty_cycle: DutyCycle,
    attack: u8,
    release: u8
}
impl Instrument {
    pub const TRUMPET: Self = Self { channel: Channel::PULSE1, duty_cycle: TONE_MODE4, attack: 2, release: 150 };
    pub const DRUM: Self = Self { channel: Channel::NOISE, duty_cycle: TONE_MODE3, attack: 0, release: 15 };
}

struct ClipInstrumentData {
    instrument: Instrument,

    sound_list: [Sound; 80]
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
                sound_list: [
                    Sound { beat: 2,   octave: Octave::OCT_3, note: Note::c  (NoteLength::DOUBLE),  volume: 100 },
                    Sound { beat: 10,  octave: Octave::OCT_3, note: Note::g  (NoteLength::DOUBLE),  volume: 100 },
                    Sound { beat: 16,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 17,  octave: Octave::OCT_3, note: Note::g  (NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 18,  octave: Octave::OCT_3, note: Note::g_s(NoteLength::QUAD),    volume: 100 },
                    Sound { beat: 30,  octave: Octave::OCT_3, note: Note::g  (NoteLength::HALF),    volume: 100 },
                    Sound { beat: 32,  octave: Octave::OCT_3, note: Note::f  (NoteLength::HALF),    volume: 100 },
                    Sound { beat: 34,  octave: Octave::OCT_3, note: Note::g  (NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 38,  octave: Octave::OCT_3, note: Note::c  (NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 42,  octave: Octave::OCT_3, note: Note::g  (NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 46,  octave: Octave::OCT_3, note: Note::a  (NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 50,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::QUAD),    volume: 100 },
                    Sound { beat: 66,  octave: Octave::OCT_4, note: Note::c  (NoteLength::DOUBLE),  volume: 100 },
                    Sound { beat: 74,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 78,  octave: Octave::OCT_3, note: Note::g_s(NoteLength::WHOLE),   volume: 100 },
                    Sound { beat: 82,  octave: Octave::OCT_3, note: Note::a_s(NoteLength::DOUBLE),  volume: 100 },
                    Sound { beat: 90,  octave: Octave::OCT_3, note: Note::d_s(NoteLength::DOUBLE),  volume: 100 },
                    Sound { beat: 96,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 97,  octave: Octave::OCT_3, note: Note::d_s(NoteLength::QUARTER), volume: 100 },
                    Sound { beat: 98,  octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 100 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                    Sound { beat: 999, octave: Octave::OCT_3, note: Note::f  (NoteLength::QUAD),    volume: 1 },
                ],
            },
            ClipInstrumentData {
                instrument: Instrument::DRUM,
                sound_list: [
                    Sound { beat:  0 + 0,   octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 0,   octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 0,   octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 0,   octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 16,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 16,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 16,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 16,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 16,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 32,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 32,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 32,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 32,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 48,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 48,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 48,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 48,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 48,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 64,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 64,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 64,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 64,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 80,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 80,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 80,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 80,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 80,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 96,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 96,  octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 96,  octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 96,  octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  0 + 112, octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  2 + 112, octave: Octave::OCT_3, note: Note::c  (NoteLength::EIGHT), volume: 100 },
                    Sound { beat:  4 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  5 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  6 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat:  8 + 112, octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 10 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 11 + 112, octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 12 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
                    Sound { beat: 14 + 112, octave: Octave::OCT_2, note: Note::c  (NoteLength::EIGHT), volume: 50 },
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
                                ((instrument_data.instrument.attack as u32) << 24) | sound_unwrapped.note.length.for_bpm(clipData.bpm) | ((instrument_data.instrument.release as u32) << 8),
                                sound_unwrapped.volume,
                                instrument_data.instrument.channel as u32 | (instrument_data.instrument.duty_cycle as u32) << 2);
                }
            }

            self.beat_counter = beat;
        }
    }
}
