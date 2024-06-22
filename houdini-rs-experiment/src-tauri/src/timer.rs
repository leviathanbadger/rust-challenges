use std::{thread, time::{self, Duration}};
use tauri::{AppHandle, Manager};

pub fn handle_timer(app: AppHandle) -> Result<(), String> {
    thread::spawn(move || {
        let start = time::Instant::now();
        let mut last_sent = u64::MAX;
        loop {
            thread::sleep(Duration::from_millis(10));
            let now = time::Instant::now();
            let elapsed = now - start;
            let elapsed_whole_seconds: u64 = elapsed.as_secs_f64().floor() as u64;

            if elapsed_whole_seconds != last_sent {
                last_sent = elapsed_whole_seconds;
                let result = app.emit_all("updateTimer", elapsed_whole_seconds);
                if let Err(err) = result {
                    println!("Failed to emit updateTimer: {}", err);
                }
            }
        }
    });

    Ok(())
}
