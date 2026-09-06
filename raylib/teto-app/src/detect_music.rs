use pulsectl::controllers::AppControl;
use pulsectl::controllers::SinkController;

/// Returns `true` if any application is actively playing audio.
pub fn is_audio_playing() -> Result<bool, Box<dyn std::error::Error>> {
    // Connect to local PulseAudio / PipeWire server
    let mut handler = SinkController::create()?;

    // List active playback applications
    let inputs = handler.list_applications()?;

    // Return true if any stream is not paused (corked) and not muted
    let is_playing = inputs.iter().any(|app| !app.corked && !app.mute);

    Ok(is_playing)
}

// fn main() {
//     match is_audio_playing() {
//         Ok(playing) => println!("Audio playing: {}", playing),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }