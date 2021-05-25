mod nodes;
use self::nodes::*;

type Audio = [f32; 256];
type Control = f32;

pub struct Stack {
    pub nodes: Vec<NodeEnum>,
    pub data: StackData,
}

pub struct StackData {
    pub audio: [Audio; 256],
    pub control: [Control; 256],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            nodes: vec![
                NodeEnum::Saw {
                    frequency: Port::Control(None, 0.1),
                    phase: Port::Control(None, 0.0),
                    output: Port::Audio(Some(0), [0.0; 256]),
                },
                NodeEnum::Abs {
                    output: Port::Audio(Some(0), [0.0; 256]),
                },
                NodeEnum::Pwm {
                    frequency: Port::Control(Some(0), 0.0),
                    phase: Port::Control(None, 0.0),
                    pulse_width: Port::Audio(Some(0), [0.0; 256]),
                    output: Port::Audio(Some(0), [0.0; 256]),
                },
                NodeEnum::Mul {
                    volume: Port::Control(None, 0.25),
                    audio: Port::Audio(Some(0), [0.0; 256]),
                },
                NodeEnum::Adsr {
                    attack: Port::Control(None, 0.1),
                    decay: Port::Control(None, 1.0),
                    sustain: Port::Control(None, 0.25),
                    release: Port::Control(None, 1.0),
                    output: Port::Audio(Some(1), [0.0; 256]),
                    previous_gate: 0.0,
                    gate: Port::Control(Some(1), 0.0),
                    time: Port::Control(Some(2), 0.0),
                    voltage: 0.0,
                },
                NodeEnum::Mul {
                    volume: Port::Audio(Some(1), [0.0; 256]),
                    audio: Port::Audio(Some(0), [0.0; 256]),
                },
            ],
            data: StackData {
                audio: [[0.0; 256]; 256],
                control: [0.0; 256],
            },
        }
    }

    pub fn process(&mut self, output_buffer: &mut [f32], sample_rate: usize) {
        for chunk in output_buffer.chunks_mut(256) {
            for node in &mut self.nodes {
                node.process(chunk.len(), &mut self.data, sample_rate);
            }
            chunk.copy_from_slice(&self.data.audio[0][..chunk.len()]);
        }
    }
}

