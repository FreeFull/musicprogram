type Audio = [f32; 256];
type Control = f32;

pub struct Stack {
    pub nodes: Vec<Node>,
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
                Node::Saw {
                    frequency: Port::Control(Some(0), 0.0),
                    phase: Port::Control(None, 0.0),
                    output: Port::Audio(Some(0), [0.0; 256]),
                },
                Node::Mul {
                    volume: Port::Control(None, 0.25),
                    audio: Port::Audio(Some(0), [0.0; 256]),
                },
                Node::Adsr {
                    attack: Port::Control(None, 0.1),
                    decay: Port::Control(None, 1.0),
                    sustain: Port::Control(None, 0.25),
                    release: Port::Control(None, 1.0),
                    output: Port::Audio(Some(1), [0.0; 256]),
                    previous_gate: 0.0,
                    gate: Port::Control(Some(1), 0.0),
                    time: 0.0,
                    voltage: 0.0,
                },
                Node::Mul {
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

pub enum Node {
    Adsr {
        attack: Port,
        decay: Port,
        sustain: Port,
        release: Port,
        output: Port,
        previous_gate: f32,
        gate: Port,
        time: f32,
        voltage: f32,
    },
    Mul {
        volume: Port,
        audio: Port,
    },
    Saw {
        frequency: Port,
        phase: Port,
        output: Port,
    },
}

impl Node {
    fn process(&mut self, samples: usize, data: &mut StackData, sample_rate: usize) {
        assert!(samples <= 256);
        match self {
            Node::Adsr {
                attack,
                decay,
                sustain,
                release,
                output,
                gate,
                time,
                voltage,
                previous_gate,
            } => {
                attack.get(data);
                decay.get(data);
                sustain.get(data);
                release.get(data);
                output.get(data);
                gate.get(data);
                for i in 0..samples {
                    let attack = attack[i];
                    let decay = decay[i];
                    let sustain = sustain[i];
                    let release = release[i];
                    if gate[i] != 0.0 {
                        if *previous_gate == 0.0 {
                            *time = 0.0;
                        }
                        if *time < attack {
                            *voltage = *time / attack;
                        } else if *time < attack + decay {
                            *voltage = (decay + attack - *time) * (1.0 - sustain) / decay + sustain;
                        }
                    } else {
                        *voltage -= sustain/(release * sample_rate as f32);
                        *voltage = (*voltage).max(0.0);
                    }
                    output[i] = *voltage;
                    *previous_gate = gate[i];
                    *time += 1.0 / sample_rate as f32;
                }
                output.set(data);
            }
            Node::Mul { volume, audio } => {
                volume.get(data);
                audio.get(data);
                for i in 0..samples {
                    audio[i] *= volume[i];
                }
                audio.set(data);
            }
            Node::Saw {
                frequency,
                phase,
                output,
            } => {
                frequency.get(data);
                phase.get(data);

                for i in 0..samples {
                    phase[i] %= 1.0;
                    output[i] = phase[i] * 2.0 - 1.0;
                    phase[i] += frequency[i] / sample_rate as f32;
                    phase[i] %= 1.0;
                }
                phase.set(data);
                output.set(data);
            }
        }
    }
}

#[derive(Clone)]
pub enum Port {
    Audio(Option<u8>, [f32; 256]),
    Control(Option<u8>, f32),
}

impl Port {
    fn get(&mut self, data: &StackData) {
        match *self {
            Port::Audio(Some(index), _) => {
                *self = Port::Audio(Some(index), data.audio[index as usize])
            }
            Port::Control(Some(index), _) => {
                *self = Port::Control(Some(index), data.control[index as usize])
            }
            _ => {}
        };
    }

    fn set(&self, data: &mut StackData) {
        match *self {
            Port::Audio(Some(index), audio) => {
                data.audio[index as usize].copy_from_slice(&audio);
            }
            Port::Control(Some(index), control) => {
                data.control[index as usize] = control;
            }
            _ => {}
        }
    }
}

impl std::ops::Index<usize> for Port {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match self {
            Port::Audio(_, audio) => &audio[index],
            Port::Control(_, control) => control,
        }
    }
}

impl std::ops::IndexMut<usize> for Port {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            Port::Audio(_, audio) => &mut audio[index],
            Port::Control(_, control) => control,
        }
    }
}
