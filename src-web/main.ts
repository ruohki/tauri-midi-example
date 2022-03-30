import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'


document.addEventListener("DOMContentLoaded", function(event) {

  // change idx to whatever device
  invoke('open_midi_connection', { inputIdx: 1 });


  listen('midi_message', event => {
    console.log(event)
  })

console.log("Ready...")
});
