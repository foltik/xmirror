use xmirror_event::Event;

pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        eprintln!("This platform is not supported. Please send a PR or switch to: [MacOS]");
        std::process::exit(1);
    }
    pub fn capture(&mut self) -> Event {
        unimplemented!()
    }
}
