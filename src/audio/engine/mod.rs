pub mod nodes;
pub mod stack;

use basedrop::Owned;
pub use nodes::*;
pub use stack::*;
use wmidi::MidiMessage;

use super::bitset::BitSet;

pub struct Engine {
    pub channels: [Option<Owned<stack::Stack>>; 16],
    pitch: f64,
    notes: BitSet,
    notes_on: u8,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            channels: [(); 16].map(|_| None),
            pitch: 110.0,
            notes: BitSet::new(),
            notes_on: 0,
        }
    }

    pub fn midi_in(&mut self, midi_message: MidiMessage) {
        let stack = match self.channels[0].as_mut() {
            Some(stack) => stack,
            None => return,
        };
        match midi_message {
            MidiMessage::NoteOn(_channel, note, _velocity) => {
                self.pitch = note.to_freq_f64();
                self.notes_on += 1;
                self.notes.set(note as u8);
                stack.data.control[2] = 0.0; // Reset ADSR
            }
            MidiMessage::NoteOff(_channel, _note, _velocity) => {
                self.notes_on = self.notes_on.saturating_sub(1);
                self.notes.clear(_note as u8)
            }
            MidiMessage::Reset => {
                self.notes_on = 0;
                self.notes.clear_all();
            }
            _ => {}
        }
        if self.notes_on > 0 {
            stack.data.control[1] = 1.0;
        } else {
            stack.data.control[1] = 0.0;
        }
        stack.data.control[0] = self.pitch as f32;
    }

    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::AddNode(index, _) => todo!(),
            Command::SetChannel(index, stack) => {
                if let Some(channel) = self.channels.get_mut(index) {
                    *channel = Some(stack);
                }
            }
            Command::ReplaceNodes(index, nodes) => {
                if let Some(&mut Some(ref mut stack)) = self.channels.get_mut(index) {
                    stack.nodes = nodes;
                }
            }
            Command::RemoveChannel(index) => {
                if let Some(channel) = self.channels.get_mut(index) {
                    *channel = None;
                }
            }
            Command::ResetData => {
                for channel in &mut self.channels {
                    if let Some(stack) = channel {
                        stack.data = StackData::default();
                    }
                }
            }
        }
    }
}

pub enum Command {
    AddNode(usize, Node),
    SetChannel(usize, Owned<stack::Stack>),
    ReplaceNodes(usize, stack::NodeList),
    RemoveChannel(usize),
    ResetData,
}
