use beep::beep;
use midir::os::unix::VirtualInput;
use midir::{Ignore, MidiInput};
use std::error::Error;
use std::convert::TryInto;
use wmidi::{
    ControlFunction,
    MidiMessage::*,
};

fn main() -> Result<(), Box<dyn Error>> {
    // silence the PC speaker
    beep(0)?;

    // create a midi input port
    let mut input = MidiInput::new(env!("CARGO_PKG_NAME"))?;

    // ignore all fancy events
    input.ignore(Ignore::All);

    // create a virtual midi port
    input.create_virtual(
        "pcspkr",
        |_, message, _|
          match message.try_into() {
              // note on, beep
              Ok(NoteOn(_, note, _)) =>
                  beep(note.to_freq_f32() as u16).unwrap(),
              // (all) note(s) off, beep 0hz
              Ok(NoteOff(_, _, _)) |
              Ok(ControlChange(_, ControlFunction::ALL_NOTES_OFF, _)) =>
                  beep(0).unwrap(),
              // ignore other events
              _ => {},
          },
        (),
    )?;

    // keep the port open
    loop {
        std::thread::yield_now()
    }
}
