use midir::{
    MidiInput, MidiInputPort, MidiInputPorts, MidiOutput, MidiOutputPort, MidiOutputPorts,
};
use std::error::Error;
use std::io::{stdin, stdout, Write};

pub fn midi_output<'a>(
    midi_out: &MidiOutput,
    out_ports: &'a MidiOutputPorts,
) -> Result<&'a MidiOutputPort, Box<dyn Error>> {
    match out_ports.len() {
        0 => Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            Ok(&out_ports[0])
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
            let port = out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?;
            Ok(port)
        }
    }
}

pub fn midi_input<'a>(
    midi_input: &MidiInput,
    in_ports: &'a MidiInputPorts,
) -> Result<&'a MidiInputPort, Box<dyn Error>> {
    match in_ports.len() {
        0 => Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_input.port_name(&in_ports[0]).unwrap()
            );
            Ok(&in_ports[0])
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_input.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let port = in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?;
            Ok(port)
        }
    }
}
