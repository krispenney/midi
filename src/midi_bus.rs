use midir::MidiOutputConnection;
use std::thread::sleep;
use std::time::Duration;

use crate::note_message::NoteMessage;

const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;

pub struct MidiBus<'a> {
    pub conn: &'a mut MidiOutputConnection,
}

impl<'a> MidiBus<'_> {
    pub fn send(&mut self, message: &NoteMessage, bpm: u64) {
        self.conn.send(&[NOTE_ON, message.note, message.velocity]);

        sleep(Duration::from_millis(message.duration_ms(bpm)));

        self.conn.send(&[NOTE_OFF, message.note, message.velocity]);
    }
}
