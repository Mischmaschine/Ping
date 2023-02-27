use ping::{PingResult, ping};

fn main() -> PingResult<()> {
    let host = "example.com";
    let timeout = std::time::Duration::from_secs(5);
    let response = ping(host, timeout)?;

    if response.dropped > 0 {
        println!("Failed to receive {} packets", response.dropped);
    } else {
        println!("Received {} packets in {}ms", response.received, response.time_ms);
    }

    Ok(())
}
