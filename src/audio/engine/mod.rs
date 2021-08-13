pub mod nodes;
pub mod stack;

pub use nodes::*;
pub use stack::*;
use wmidi::MidiMessage;

use super::bitset::BitSet;

pub struct Engine {
    pub midi_in: Option<Box<dyn FnMut(&mut Engine, MidiMessage) + Send>>,
    pub stack: stack::Stack,
    pitch: f64,
    notes: BitSet,
    notes_on: u8,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            midi_in: None,
            stack: stack::Stack::new(),
            pitch: 110.0,
            notes: BitSet::new(),
            notes_on: 0,
        }
    }

    pub fn midi_in(&mut self, midi_message: MidiMessage) {
        match midi_message {
            MidiMessage::NoteOn(_channel, note, _velocity) => {
                self.pitch = note.to_freq_f64();
                self.notes_on += 1;
                self.notes.set(note as u8);
                self.stack.data.control[2] = 0.0; // Reset ADSR
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
            self.stack.data.control[1] = 1.0;
        } else {
            self.stack.data.control[1] = 0.0;
        }
        self.stack.data.control[0] = self.pitch as f32;
        /*
        let mut midi_closure = self.midi_in.take();
        if let Some(ref mut midi_closure) = midi_closure {
            midi_closure(self, message);
        }
        self.midi_in = midi_closure;
        */
    }
}
