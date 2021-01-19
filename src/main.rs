use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let addr = stream.peer_addr()?;
    println!("Connection established with {}", addr);
    loop {
        let mut buf = vec![0; 16];
        match stream.read_exact(&mut buf) {
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buf);
                println!("{}: {}", addr, msg);
            }
            Err(_) => {
                println!("Closing connection with {}", addr);
                break;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666")?;

    for stream in listener.incoming() {
        thread::spawn(move || handle_client(stream?));
    }

    Ok(())
}
