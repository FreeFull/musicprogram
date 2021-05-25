use std::convert::TryFrom;

pub use self::error::Error;

mod error;
mod stack;
mod bitset;

#[derive(Debug)]
pub struct Controller {
    pub active_client: jack::AsyncClient<NotificationHandler, ProcessHandler>,
    pub data_in: rtrb::Consumer<wmidi::Note>,
}

pub struct NotificationHandler {}

impl jack::NotificationHandler for NotificationHandler {}

pub struct ProcessHandler {
    midi_in: jack::Port<jack::MidiIn>,
    audio_out: jack::Port<jack::AudioOut>,
    stack: stack::Stack,
    pitch: f64,
    notes_on: u8,
    data_out: rtrb::Producer<wmidi::Note>,
}

impl jack::ProcessHandler for ProcessHandler {
    fn process(
        &mut self,
        client: &jack::Client,
        process_scope: &jack::ProcessScope,
    ) -> jack::Control {
        for data in self.midi_in.iter(process_scope) {
            use wmidi::MidiMessage;
            match MidiMessage::try_from(data.bytes) {
                Ok(MidiMessage::NoteOn(_channel, note, _velocity)) => {
                    self.pitch = note.to_freq_f64();
                    self.notes_on += 1;
                    self.stack.data.control[2] = 0.0; // Reset ADSR
                    self.data_out.push(note).ok();
                },
                Ok(MidiMessage::NoteOff(_channel, _note, _velocity)) => {
                    self.notes_on = self.notes_on.saturating_sub(1);
                },
                Ok(MidiMessage::Reset) => {
                    self.notes_on = 0;
                }
                _ => {}
            }
        }
        if self.notes_on > 0 {
            self.stack.data.control[1] = 1.0;
        } else {
            self.stack.data.control[1] = 0.0;
        }
        if self.midi_in.connected_count() == Ok(0) {
            self.notes_on = 0;
        }
        if self.audio_out.connected_count() == Ok(0) {
            return jack::Control::Quit;
        }
        let buffer = self.audio_out.as_mut_slice(process_scope);
        self.stack.data.control[0] = self.pitch as f32;
        self.stack.process(buffer, client.sample_rate());
        jack::Control::Continue
    }
}

pub fn start() -> Result<Controller, Error> {
    let (client, _status) =
        jack::Client::new("musicprogram", jack::ClientOptions::NO_START_SERVER)?;
    let midi_in = client.register_port("capture_1", jack::MidiIn)?;
    let audio_out = client.register_port("playback_1", jack::AudioOut)?;
    let notification_handler = NotificationHandler {};
    let data = rtrb::RingBuffer::new(128).split();
    let process_handler = ProcessHandler {
        midi_in,
        audio_out,
        stack: stack::Stack::new(),
        pitch: 110.0,
        notes_on: 0,
        data_out: data.0,
    };
    let active_client = client.activate_async(notification_handler, process_handler)?;
    Ok(Controller { active_client, data_in: data.1 })
}
