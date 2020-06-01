use crate::midi_bus::MidiBus;
use crate::note_message::NoteMessage;
use std::thread::sleep;
use std::time::Duration;

pub struct MidiPlayer<'a> {
    pub bus: &'a mut MidiBus<'a>,
    pub notes: &'a Vec<NoteMessage>,
}

impl<'a> MidiPlayer<'_> {
    pub fn play(&mut self, bpm: u64) {
        sleep(Duration::from_millis(150));

        for note in self.notes {
            self.bus.send(&note, bpm);
        }

        sleep(Duration::from_millis(150));
    }
}
