use std::env;
use std::error::Error;
use std::net::TcpStream;

use xmirror_event::Event;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::Macos as Platform;

#[cfg(not(target_os = "macos"))]
mod dummy;
#[cfg(not(target_os = "macos"))]
use dummy::Dummy as Platform;

fn main() -> Result<(), Box<dyn Error>> {
    let mut platform = Platform::new();

    let addr = env::args().nth(1).ok_or("usage: xmirror <host:port>")?;
    let mut stream = TcpStream::connect(addr)?;
    stream.set_nodelay(true)?;

    loop {
        let event = platform.capture();
        println!("{event:?}");

        // Exit if Shift+Alt+Ctrl+Super are all pressed at once
        if let Event::Mods(mods) = event {
            if mods.is_all() {
                drop(platform);
                std::process::exit(0);
            }
        }

        if let Err(e) = event.encode(&mut stream) {
            eprintln!("{e:#}");
            drop(platform);
            std::process::exit(1);
        }
    }
}
