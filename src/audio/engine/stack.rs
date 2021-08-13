use super::*;

type Audio = [f32; 256];
type Control = f32;

#[derive(Clone)]
pub struct Stack {
    pub nodes: Vec<Node>,
    pub data: StackData,
}

#[derive(Clone)]
pub struct StackData {
    pub audio: [Audio; 256],
    pub control: [Control; 256],
}

impl Stack {
    pub fn new() -> Stack {
        let mut chain = Vec::new();
        chain.push(Node::Oscillator {
            frequency: Port {
                name: "frequency",
                stack_index: Some(0),
                kind: PortKind::Control(0.0),
            },
            phase: Port {
                name: "phase",
                stack_index: None,
                kind: PortKind::Control(0.0),
            },
            waveform: Port {
                name: "control",
                stack_index: None,
                kind: PortKind::Control(1.0),
            },
            pulse_width: Port {
                name: "",
                stack_index: None,
                kind: PortKind::Control(0.125),
            },
            output: Port {
                name: "output",
                stack_index: Some(0),
                kind: PortKind::Audio([0.0; 256]),
            },
        });
        chain.push(Node::Mul {
            input_1: Port {
                name: "",
                stack_index: Some(0),
                kind: PortKind::Audio([0.0; 256]),
            },
            input_2: Port {
                name: "",
                stack_index: None,
                kind: PortKind::Control(0.125),
            },
            output: Port {
                name: "",
                stack_index: Some(0),
                kind: PortKind::Audio([0.0; 256]),
            },
        });
        Stack {
            nodes: chain,
            data: StackData {
                audio: [[0.0; 256]; 256],
                control: [0.0; 256],
            },
        }
    }

    pub fn process(&mut self, output_buffer: &mut [f32], sample_rate: usize) {
        for chunk in output_buffer.chunks_mut(256) {
            for node in &mut *self.nodes {
                node.process(chunk.len(), &mut self.data, sample_rate);
            }
            chunk.copy_from_slice(&self.data.audio[0][..chunk.len()]);
        }
    }
}
