extern crate beep;
extern crate midir;
extern crate wmidi;
extern crate pitch_calc;
extern crate dimensioned;

use beep::beep;
use dimensioned::si;
use pitch_calc::Step;
use std::error::Error;
use midir::{MidiInput, Ignore};
use midir::os::unix::VirtualInput;
use wmidi::MidiMessage::{self, *};

fn run() -> Result<(), Box<Error>> {
  // silence the PC speaker
  beep(0. * si::HZ);

  // create a midi input port
  let mut input = MidiInput::new(env!("CARGO_PKG_NAME"))?;

  // ignore all fancy events
  input.ignore(Ignore::All);

  // create a virtual midi port
  let _port = input.create_virtual("pcspkr",
    |_, message, _| {
      match MidiMessage::from_bytes(message) {
        Ok(NoteOn(_, note, _))
          => beep(Step(note as f32).hz() as f64 * si::HZ),
        Ok(NoteOff(_, _, _)) | Ok(ControlChange(_, 123, 0))
          => beep(0. * si::HZ),
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
