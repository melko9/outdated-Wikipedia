

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let mut current = String::new();
    stream.read_to_string(&mut current)?;

    let updated = shared::jschanges(&current);

    stream.write_all(updated.as_bytes())?;
    Ok(())
}
