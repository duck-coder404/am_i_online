use std::io::{Result, prelude::*};
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    for _ in 0..3 {
        let online: bool = ping("google.com").is_ok();
        println!("{:?}", ping("google.com"));
        if online {
            println!("Yes, you are online");
        } else {
            println!("No, you are offline");
        }
    }
}

fn ping(addr: &str) -> Result<()> {
    let addr = addr.to_owned() + ":80";
    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(&[1])?;
    stream.read_exact(&mut [0; 128])?;
    Ok(())
}
