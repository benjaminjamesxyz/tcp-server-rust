use std::io::Result;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const HOST: &str = "127.0.0.1";
const PORT: &str = "4554";
const BUFFER_SIZE: usize = 512;

fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;
    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_connection(stream).unwrap();
        });
    }
    Ok(())
}

fn handle_connection(stream: Result<TcpStream>) -> Result<()> {
    let mut stream = setup_stream(stream?)?;
    process_client_data(&mut stream)?;
    cleanup_connection(&mut stream)
}

fn process_client_data(stream: &mut TcpStream) -> Result<()> {
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Info: recived `{}` from {}",
        String::from_utf8_lossy(&buffer[..bytes_read]),
        stream.peer_addr()?
    );
    stream.write_all(&buffer[..bytes_read])?;
    Ok(())
}

fn setup_stream(stream: TcpStream) -> Result<TcpStream> {
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    Ok(stream)
}

fn cleanup_connection(stream: &mut TcpStream) -> Result<()> {
    stream.shutdown(std::net::Shutdown::Both)
}
