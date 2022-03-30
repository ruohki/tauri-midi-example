#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use midir::{Ignore, MidiInput, MidiInputConnection};
use tauri::{Manager, Window, Wry};
use serde::{Serialize};

#[derive(Default)]
pub struct MidiState {
  pub input: Mutex<Option<MidiInputConnection<()>>>
}

#[derive(Clone, Serialize)]
struct MidiMessage {
  message: Vec<u8>
}

#[tauri::command]
fn open_midi_connection(
  midi_state: tauri::State<'_, MidiState>,
  window: Window<Wry>,
  input_idx: usize
) {
  let handle = Arc::new(window).clone();
  let mut midi_in = MidiInput::new("My Test Input").unwrap();
  midi_in.ignore(Ignore::None);
  let midi_in_ports = midi_in.ports();
  if let Some(in_port) = midi_in_ports.get(input_idx) {
    let conn_in = midi_in.connect(in_port, "midir-test", move |stamp, message, log| {
      // The last of the three callback parameters is the object that we pass in as last parameter of `connect`.

      handle.emit_all("midi_message",  MidiMessage { message: message.to_vec() });

      println!("{}: {:?} (len = {})", stamp, message, message.len());
    }, ()).unwrap();
    *midi_state.input.lock().unwrap() = Some(conn_in);
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_midi_connection])
    .manage(MidiState { ..Default::default() })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
