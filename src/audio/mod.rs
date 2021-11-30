use std::convert::TryFrom;

pub use self::error::Error;

mod bitset;
mod engine;
pub use engine::*;
mod error;

#[derive(Debug)]
pub struct Controller {
    pub active_client: jack::AsyncClient<NotificationHandler, ProcessHandler>,
    pub midi_ui: rtrb::Consumer<wmidi::MidiMessage<'static>>,
    pub input: rtrb::Producer<Command>,
}

pub struct NotificationHandler {}

impl jack::NotificationHandler for NotificationHandler {}

pub struct ProcessHandler {
    midi_in: jack::Port<jack::MidiIn>,
    audio_out: jack::Port<jack::AudioOut>,
    engine: engine::Engine,

    midi_ui: rtrb::Producer<wmidi::MidiMessage<'static>>,
    input: rtrb::Consumer<Command>,
}

impl jack::ProcessHandler for ProcessHandler {
    fn process(
        &mut self,
        client: &jack::Client,
        process_scope: &jack::ProcessScope,
    ) -> jack::Control {
        while let Ok(command) = self.input.pop() {
            self.engine.run_command(command);
        }
        for data in self.midi_in.iter(process_scope) {
            use wmidi::MidiMessage;
            if let Ok(Some(midi_message)) =
                MidiMessage::try_from(data.bytes).map(|m| m.drop_unowned_sysex())
            {
                self.midi_ui.push(midi_message.clone()).ok();
                self.engine.midi_in(midi_message);
            }
        }
        if self.midi_in.connected_count() == Ok(0) {
            self.engine.midi_in(wmidi::MidiMessage::Reset);
        }
        if self.audio_out.connected_count() == Ok(0) {
            return jack::Control::Quit;
        }
        let buffer = self.audio_out.as_mut_slice(process_scope);
        self.engine.stack.process(buffer, client.sample_rate());
        for sample in buffer {
            *sample = sample.clamp(-1.0, 1.0);
        }
        jack::Control::Continue
    }
}

pub fn start() -> Result<Controller, Error> {
    let (client, _status) =
        jack::Client::new("musicprogram", jack::ClientOptions::NO_START_SERVER)?;
    let midi_in = client.register_port("capture_1", jack::MidiIn)?;
    let audio_out = client.register_port("playback_1", jack::AudioOut)?;
    let notification_handler = NotificationHandler {};
    let data = rtrb::RingBuffer::new(128);
    let input = rtrb::RingBuffer::new(1);
    let process_handler = ProcessHandler {
        midi_in,
        audio_out,
        engine: engine::Engine::new(),

        midi_ui: data.0,
        input: input.1,
    };

    let active_client = client.activate_async(notification_handler, process_handler)?;
    Ok(Controller {
        active_client,
        midi_ui: data.1,
        input: input.0,
    })
}
