use std::fs;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

const PORT: &str = "127.0.0.1:7878";
const DATA_FILE: &str = "jstrain.json";

fn server_run() -> io::Result<()> {
    let listener = TcpListener::bind(PORT)?;
    println!("listening on {}", PORT);

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("connected: {}", addr);
                if let Err(e) = useit(stream) {
                    eprintln!("client error: {}", e);
                }
            }
            Err(e) => eprintln!("accept error: {}", e),
        }
    }
}

fn useit(mut stream: TcpStream) -> io::Result<()> {
    let current = fs::read_to_string(DATA_FILE).unwrap_or_default();
    stream.write_all(current.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    let mut updated = String::new();
    stream.read_to_string(&mut updated)?;


    if !updated.is_empty() {
        fs::write(DATA_FILE, updated)?;
        println!("data.json updated");
    }
    Ok(())
}

fn main() {
    if let Err(e) = server_run() {
        eprintln!("server error: {}", e);
    }
}
