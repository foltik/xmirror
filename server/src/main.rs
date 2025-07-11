use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::{env, thread};

use xmirror_event::Event;

#[cfg(target_os = "linux")]
mod x11;
#[cfg(target_os = "linux")]
use x11::X11 as Platform;

#[cfg(not(target_os = "linux"))]
mod dummy;
#[cfg(not(target_os = "linux"))]
use dummy::Dummy as Platform;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let platform = Platform::new();

    let addr = env::args().nth(1).ok_or("usage: xmirror-server <ip:port>")?;
    let listener = TcpListener::bind(&addr)?;
    println!("Listening at {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let peer = stream.peer_addr().unwrap();
                    println!("Connected: {peer}");

                    match accept(&platform, stream) {
                        Err(e) => println!("Error from {peer}: {e:#}"),
                        Ok(()) => println!("Disconnected: {peer}"),
                    }
                });
            }
            Err(e) => println!("Error accepting: {e:#}"),
        }
    }
    Ok(())
}

fn accept(platform: &Platform, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    stream.set_nodelay(true)?;

    let mut packet = [0u8; Event::SIZE];
    loop {
        match stream.read_exact(&mut packet) {
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(e.into()),
            Ok(()) => {}
        }

        let event = Event::decode(&packet)?;
        platform.emulate(event)?;
    }
}
