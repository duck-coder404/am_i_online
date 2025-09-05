use ping_rs::{self, send_ping};
use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");

    // Parse the IP address
    let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(142, 250, 180, 174));
    // Prepare ping data
    let data = [1; 4];
    // Set timeout
    let timeout = Duration::from_secs(1);
    // Configure ping options (optional)
    let options = ping_rs::PingOptions {
        ttl: 128,
        dont_fragment: true,
    };

    let mut previous_state: Option<bool> = None;

    loop {
        let current_state = send_ping(&addr, timeout, &data, Some(&options)).is_ok();

        // Check if the state has changed
        if previous_state != Some(current_state) {
            if current_state {
                // Print in green
                println!("\x1b[32m{:?} Yes, you are online\x1b[0m", SystemTime::now());
            } else {
                // Print in red
                println!("\x1b[31m{:?} No, you are offline\x1b[0m", SystemTime::now());
            }
            // Update the previous state
            previous_state = Some(current_state);
        }

        thread::sleep(Duration::new(1, 0)); // Sleep
    }
}
