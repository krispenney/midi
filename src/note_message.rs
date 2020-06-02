const ONE_MINUTE_MS: u64 = 60_000;

pub enum NoteDuration {
    Sixteenth,
    Eigth,
    Quarter,
    Half,
    Whole,
}

pub struct NoteMessage {
    pub note: u8,
    pub velocity: u8,

    duration: NoteDuration,
}

impl NoteMessage {
    pub fn new(note: u8, duration: NoteDuration) -> NoteMessage {
        NoteMessage {
            note,
            duration,
            velocity: 0x64,
        }
    }

    pub fn duration_ms(&self, bpm: u64) -> u64 {
        let quarter_note_duration = ONE_MINUTE_MS / bpm;

        match self.duration {
            NoteDuration::Sixteenth => quarter_note_duration >> 2,
            NoteDuration::Eigth => quarter_note_duration >> 1,
            NoteDuration::Quarter => quarter_note_duration,
            NoteDuration::Half => quarter_note_duration << 1,
            NoteDuration::Whole => quarter_note_duration << 2,
        }
    }
}
