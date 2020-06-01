pub struct NoteMessage {
    pub note: u8,
    pub velocity: u8,

    pub duration: u64,
}

impl NoteMessage {
    pub fn new(note: u8, duration: u64) -> NoteMessage {
        NoteMessage {
            note,
            duration,
            velocity: 0x64,
        }
    }
}
