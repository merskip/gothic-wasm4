use crate::audio::music::{Bpm, ClipData, ClipInstrumentData, Instrument, Note, NoteLength, Octave, Sound};

pub static MAIN_THEME: ClipData = ClipData {
    bpm: Bpm::Bpm164,
    length: 160,
    instrument_data: [
        ClipInstrumentData {
            instrument: Instrument::TRUMPET,
            length: 160,
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
            instrument: Instrument::TRUMPET2,
            length: 160,
            sound_list: &[
                Sound { beat: 0, octave: Octave::OCT_2, note: Note::c(NoteLength::QUAD), volume: 30 },
                Sound { beat: 16, octave: Octave::OCT_2, note: Note::g_s(NoteLength::QUAD), volume: 30 },
                Sound { beat: 32, octave: Octave::OCT_2, note: Note::c(NoteLength::QUAD), volume: 30 },
                Sound { beat: 48, octave: Octave::OCT_2, note: Note::a_s(NoteLength::QUAD), volume: 30 },
                Sound { beat: 64, octave: Octave::OCT_3, note: Note::c(NoteLength::QUAD), volume: 30 },
                Sound { beat: 80, octave: Octave::OCT_2, note: Note::a_s(NoteLength::QUAD), volume: 30 },
                Sound { beat: 96, octave: Octave::OCT_2, note: Note::f(NoteLength::QUAD), volume: 30 },
                Sound { beat: 112, octave: Octave::OCT_2, note: Note::d_s(NoteLength::QUAD), volume: 30 },
                Sound { beat: 128, octave: Octave::OCT_2, note: Note::c(NoteLength::QUAD), volume: 30 },
                Sound { beat: 143, octave: Octave::OCT_1, note: Note::b(NoteLength::QUARTER), volume: 30 },
                Sound { beat: 144, octave: Octave::OCT_2, note: Note::c(NoteLength::QUAD), volume: 30 },
                Sound { beat: 159, octave: Octave::OCT_1, note: Note::b(NoteLength::QUARTER), volume: 30 },
            ],
        },
        ClipInstrumentData {
            instrument: Instrument::DRUM,
            length: 16,
            sound_list: &[
                Sound { beat: 0, octave: Octave::OCT_3, note: Note::c(NoteLength::EIGHT), volume: 100 },
                Sound { beat: 2, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 3, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 4, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 6, octave: Octave::OCT_2, note: Note::d_s(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 8, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 9, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 10, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 12, octave: Octave::OCT_2, note: Note::c(NoteLength::EIGHT), volume: 50 },
                Sound { beat: 14, octave: Octave::OCT_3, note: Note::c(NoteLength::EIGHT), volume: 100 },
            ],
        },
    ],
};