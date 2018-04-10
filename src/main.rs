extern crate beep;
extern crate midir;
extern crate pitch_calc;
extern crate dimensioned;

use beep::beep;
use dimensioned::si;
use pitch_calc::Step;
use std::error::Error;
use midir::{MidiInput, Ignore};
use midir::os::unix::VirtualInput;

fn run() -> Result<(), Box<Error>> {
  // create a midi input port
  let mut input = MidiInput::new(env!("CARGO_PKG_NAME"))?;

  // ignore all fancy events
  input.ignore(Ignore::All);

  // create a virtual midi port
  let _port = input.create_virtual("pcspkr",
    |_, message, _| {
      // we will only handle two messages, NOTE_ON and NOTE_OFF:
      const NOTE_ON : u8 = 0x90;
      const NOTE_OFF: u8 = 0x80;
      // do the stuff:
      match message {
        // todo: polyphonics, maybe.
        &[NOTE_ON , note, _] => beep(Step(note as f32).hz() as f64 * si::HZ),
        &[NOTE_OFF,    _, _] => beep(0. * si::HZ),
                           _ => {}}}, ())?;

  // keep the port open
  loop {std::thread::park()}
}

fn main() {
  // run and, if necessary, print error message to stderr
  if let Err(error) = run() {
    eprintln!("Error: {}", error);
    std::process::exit(1);
  }
}
