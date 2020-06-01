use midir::MidiOutput;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

mod note_message;
use note_message::NoteMessage;

mod midi_bus;
use midi_bus::MidiBus;

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
        NoteMessage::new(57, 3),
        NoteMessage::new(59, 3),
        NoteMessage::new(60, 3),
        NoteMessage::new(62, 3),
        NoteMessage::new(57, 1),
        NoteMessage::new(59, 2),
        NoteMessage::new(55, 2),
        NoteMessage::new(57, 5),
    ];

    let mut message_bus = MidiBus {
        conn: &mut conn_out,
    };
    for m in messages {
        message_bus.send(&m);
    }

    sleep(Duration::from_millis(150));
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");

    Ok(())
}
