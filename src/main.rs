///
/// apc_mk2_mapper
///
/// simple mapper to make it easier to provide feedback to an AKAI APC mini mk2 midi controller
/// used together with qlcplus
///
/// this program receives the feedback messages from qlcplus and adapts them to control the
/// button leds on the  AKAI APC mini mk2
/// 
/// this uses the os::unix::VirtualInput which is only supported under linux
///
/// 

use config::Config;
use midir::{os::unix::VirtualInput, MidiInput, MidiOutput, MidiOutputConnection};
use wmidi::{Channel, MidiMessage, U7};

/// this function gets called every time a midi message arrives
fn midi_callback(_stamp: u64, message_bytes: &[u8], con: &mut MidiOutputConnection) {
    println!("{:?}", message_bytes);

    let message = MidiMessage::try_from(message_bytes).unwrap();
    println!("{:?}", message);

    // only ON messages are used for feedback
    if let MidiMessage::NoteOn(_ch, note, vel) = message {
        let u: u8 = note.into();
        let color: u8 = vel.into();
        let mut bytes = [0u8; 3];

        // this is a note belonging to the outer buttons, these need to stay on CH1
        if u > 88 {
            let new_message = MidiMessage::NoteOn(Channel::Ch1, note, vel);
            println!("Sending {:?}", new_message);

            new_message.copy_to_slice(&mut bytes).unwrap();
            con.send(&bytes).unwrap();
        } else if color > 64 {
            // this button feedback gets mapped to CH9 which makes the button blink
            let new_color = color - 64;
            let new_message = MidiMessage::NoteOn(Channel::Ch9, note, U7::from_u8_lossy(new_color));
            println!("Sending {:?}", new_message);

            new_message.copy_to_slice(&mut bytes).unwrap();
            con.send(&bytes).unwrap();
        } else {
            // this is the normal case, all buttons get mapped to CH7 to make them as bright as possible
            let new_message = MidiMessage::NoteOn(Channel::Ch7, note, vel);
            println!("Sending {:?}", new_message);

            new_message.copy_to_slice(&mut bytes).unwrap();
            con.send(&bytes).unwrap();
        }
    } else if let MidiMessage::NoteOff(ch, note, vel) = message {
        // convert Off messages to on messages
        let new_message = MidiMessage::NoteOn(ch, note, vel);
        let mut bytes = [0u8; 3];
        new_message.copy_to_slice(&mut bytes).unwrap();
        con.send(&bytes).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("mk2apc_4light_port mapping feedback from qlcplus");

    // Load configuration from a file
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    // index of the AKAI APC mini mk2, see consol log for choosing the right one
    let midi_idx = config.get_int("midi_idx")? as usize;

    // create input and output driver
    let midi_in = MidiInput::new("mk2apc_4light").unwrap();
    let midi_out = MidiOutput::new("mk2apc_4light").unwrap();

    // choose output port (the AKAI APC midi device)
    let outports: Vec<midir::MidiOutputPort> = midi_out.ports();
    let out_port = match outports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_out.port_name(&outports[0]).unwrap()
            );
            &outports[0]
        }
        //TODO: serach for the correct name "AKAI APC ..."
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in outports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }

            outports
                .get(midi_idx)
                .ok_or("invalid input port selected")?
        }
    };
    println!("\nOpening connection {}", midi_out.port_name(out_port)?);

    // open output connection
    let outcon = midi_out.connect(out_port, "mk2apc_4light_con").unwrap();

    // create virtual input and pass on the output-connection to the callback function
    let _inport = midi_in
        .create_virtual("mk2apc_4light_port", midi_callback, outcon)
        .unwrap();

    // put the main thread to sleep so midi_in and midi_out can work
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
