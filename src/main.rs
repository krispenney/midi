use midir::MidiOutput;
use std::error::Error;
use std::io::{stdin, stdout, Write};

mod note_message;
use note_message::{NoteDuration, NoteMessage};

mod midi_bus;
use midi_bus::MidiBus;

mod midi_player;
use midi_player::MidiPlayer;

fn main() {
    match run() {
        Err(e) => println!("ERROR: {}", e),
        _ => (),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Test")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

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

    MidiPlayer {
        bus: &mut message_bus,
        notes: &messages,
    }
    .play(180);

    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");

    Ok(())
}
