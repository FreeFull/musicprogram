use crossbeam::Sender;

mod error;
pub use self::error::Error;

#[derive(Debug)]
pub struct Controller {
    active_client: jack::AsyncClient<NotificationHandler, ProcessHandler>,
}

#[derive(Debug)]
pub enum Command {
    Noop,
}

struct NotificationHandler {}

impl jack::NotificationHandler for NotificationHandler {
}

struct ProcessHandler {
    midi_in_port: jack::Port<jack::MidiIn>,
}

impl jack::ProcessHandler for ProcessHandler {
    fn process(
        &mut self,
        client: &jack::Client,
        process_scope: &jack::ProcessScope,
    ) -> jack::Control {
        for data in self.midi_in_port.iter(process_scope) {
            println!("{:?}", data);
        }
        jack::Control::Continue
    }
}

pub fn start() -> Result<Controller, Error> {
    let (client, _status) = jack::Client::new("musicprogram", jack::ClientOptions::NO_START_SERVER)?;
    let port = client.register_port("capture_1", jack::MidiIn)?;
    let notification_handler = NotificationHandler {};
    let process_handler = ProcessHandler { midi_in_port: port, };
    let active_client = client.activate_async(notification_handler, process_handler)?;
    Ok(Controller { active_client, })
}
