use std::process::Command;

pub fn get_volume() -> Result<i32, std::io::Error> {
    let output = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()?;
    let s = String::from_utf8_lossy(&output.stdout);
    // parse "Volume: front-left: 65536 / 100% / 0.00 dB"
    let vol = s.split('/').nth(1)
        .and_then(|v| v.trim().trim_end_matches('%').parse().ok())
        .unwrap_or(0);
    Ok(vol)
}

pub fn set_volume(vol: i32) -> Result<(), std::io::Error> {
    Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", vol)])
        .output()?;
    Ok(())
}
