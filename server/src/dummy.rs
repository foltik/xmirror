use xmirror_event::Event;

#[derive(Clone, Copy)]
pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        eprintln!("This platform is not supported. Please send a PR or switch to: [Linux+X11]");
        std::process::exit(1);
    }
    pub fn emulate(&self, _: Event) -> Result<(), String> {
        unimplemented!()
    }
}
