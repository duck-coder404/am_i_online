use chrono::prelude::*;
use clap::Parser;
use ping_rs::{self, PingOptions, send_ping};
use std::net::IpAddr;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "ping-monitor")]
#[command(about = "Monitor network connectivity with visual status changes", long_about = None)]
struct Args {
    /// IP address to ping
    #[arg(short, long, default_value = "8.8.8.8")]
    address: IpAddr,

    /// Timeout in seconds
    #[arg(short, long, default_value_t = 2)]
    timeout: u64,

    /// Interval between pings in seconds
    #[arg(short, long, default_value_t = 1)]
    interval: u64,

    /// TTL (time to live)
    #[arg(long, default_value_t = 128)]
    ttl: u8,
}

fn main() {
    let args = Args::parse();

    // Print the current settings to user
    println!(
        "Monitoring connectivity to {} (timeout: {}s, interval: {}s, TTL: {})",
        args.address, args.timeout, args.interval, args.ttl
    );
    println!("Press Ctrl+C to stop\n");

    let data = [1; 4];
    let timeout = Duration::from_secs(args.timeout);
    let interval = Duration::from_secs(args.interval);

    let options = PingOptions {
        ttl: args.ttl,
        dont_fragment: false,
    };

    let mut previous_state: Option<bool> = None;

    loop {
        let current_state = send_ping(&args.address, timeout, &data, Some(&options));

        if previous_state != Some(current_state.is_ok()) {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            if current_state.is_ok() {
                println!("\x1b[32m{} Yes, you are online\x1b[0m", time);
            } else {
                println!("\x1b[31m{} No, you are offline\x1b[0m", time);
                println!("{:?}", current_state);
            }
            previous_state = Some(current_state.is_ok());
        }

        thread::sleep(interval);
    }
}
