use midir::{MidiInput, MidiOutput};
use std::error::Error;

mod note_message;
use note_message::{NoteDuration, NoteMessage};

use std::thread::sleep;
use std::time::Duration;

mod midi_bus;
use midi_bus::MidiBus;

mod midi_player;
use midi_player::MidiPlayer;

use std::io::stdin;

mod resolve_port;

fn main() {
    match run() {
        Err(e) => println!("ERROR: {}", e),
        _ => (),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Test")?;
    let out_ports = midi_out.ports();
    let out_port = resolve_port::midi_output(&midi_out, &out_ports)?;

    let midi_in = MidiInput::new("Reader")?;
    let in_ports = midi_in.ports();
    let in_port = resolve_port::midi_input(&midi_in, &in_ports)?;

    println!("Opening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");

    // :perfection:
    let messages = vec![
        NoteMessage::new(57, NoteDuration::Quarter),
        NoteMessage::new(59, NoteDuration::Quarter),
        NoteMessage::new(60, NoteDuration::Quarter),
        NoteMessage::new(62, NoteDuration::Quarter),
        NoteMessage::new(57, NoteDuration::Sixteenth),
        NoteMessage::new(59, NoteDuration::Quarter),
        NoteMessage::new(55, NoteDuration::Eigth),
        NoteMessage::new(57, NoteDuration::Whole),
    ];

    let mut message_bus = MidiBus {
        conn: &mut conn_out,
    };

    let mut player = MidiPlayer {
        bus: &mut message_bus,
        notes: &messages,
    };
    player.play(160);

    let mut input = String::new();
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
            if let [msg_code, note, velocity] = message {
                conn_out.send(&[*msg_code, note + 12, *velocity]);
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    // conn_out.close();
    println!("Connection closed");

    Ok(())
}
