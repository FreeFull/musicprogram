use std::iter::FusedIterator;

use arrayvec::ArrayVec;
use enum_iterator::IntoEnumIterator;
use enum_kinds::EnumKind;

use super::*;

mod port;
pub use port::*;

#[derive(Copy, Clone, Debug, EnumKind, PartialEq)]
#[enum_kind(NodeKind, derive(IntoEnumIterator))]
pub enum Node {
    Abs {
        input: Port,
        output: Port,
    },
    Add {
        input_1: Port,
        input_2: Port,
        output: Port,
    },
    Adsr {
        attack: Port,
        decay: Port,
        sustain: Port,
        release: Port,
        output: Port,
        time: Port,
        gate: Port,
        previous_gate: f32,
        voltage: f32,
    },
    Mul {
        input_1: Port,
        input_2: Port,
        output: Port,
    },
    Oscillator {
        frequency: Port,
        phase: Port,
        waveform: Port,
        pulse_width: Port,
        output: Port,
    },
}

impl Node {
    pub fn new(variant: NodeKind) -> Node {
        match variant {
            NodeKind::Abs => Node::Abs {
                input: Port::audio("input"),
                output: Port::audio("output"),
            },
            NodeKind::Add => Node::Add {
                input_1: Port::audio("input 1"),
                input_2: Port::audio("input 2"),
                output: Port::audio("output"),
            },
            NodeKind::Adsr => Node::Adsr {
                attack: Port::audio("attack"),
                decay: Port::audio("decay"),
                sustain: Port::audio("sustain"),
                release: Port::audio("release"),
                gate: Port::audio("gate"),
                time: Port::audio("time"),
                output: Port::audio("output"),
                previous_gate: 0.0,
                voltage: 0.0,
            },
            NodeKind::Mul => Node::Mul {
                input_1: Port::default(),
                input_2: Port::default(),
                output: Port::audio("output"),
            },
            NodeKind::Oscillator => Node::Oscillator {
                frequency: Port::audio("frequency"),
                phase: Port::new("phase", PortKind::Control(0.0)),
                waveform: Port::new("waveform", PortKind::Control(0.0)),
                pulse_width: Port::audio("pulse width"),
                output: Port::audio("output"),
            },
        }
    }

    pub fn process(&mut self, samples: usize, data: &mut StackData, sample_rate: usize) {
        assert!(0 < samples && samples <= 256);
        use Node::*;
        for input in self.inputs() {
            input.read(data);
        }
        match self {
            Add {
                input_1,
                input_2,
                output,
            } => {
                for i in 0..samples {
                    output[i] = input_1[i] + input_2[i];
                }
            }
            Abs { input, output } => {
                for i in 0..samples {
                    output[i] = input[i].abs();
                }
            }
            Adsr {
                attack,
                decay,
                sustain,
                release,
                output,
                time,
                gate,
                previous_gate,
                voltage,
            } => {
                for i in 0..samples {
                    let attack = attack[i];
                    let decay = decay[i];
                    let sustain = sustain[i];
                    let release = release[i];
                    if gate[i] != 0.0 {
                        if *previous_gate == 0.0 {
                            time[i] = 0.0;
                        }
                        if time[i] < attack {
                            *voltage = time[i] / attack;
                        } else if time[i] < attack + decay {
                            *voltage =
                                (decay + attack - time[i]) * (1.0 - sustain) / decay + sustain;
                        }
                    } else {
                        *voltage -= sustain / (release * sample_rate as f32);
                        *voltage = (*voltage).max(0.0);
                    }
                    output[i] = *voltage;
                    *previous_gate = gate[i];
                    time[i] += 1.0 / sample_rate as f32;
                }
            }
            Mul {
                input_1,
                input_2,
                output,
            } => {
                for i in 0..samples {
                    output[i] = input_1[i] * input_2[i];
                }
            }
            Oscillator {
                frequency,
                phase,
                waveform,
                pulse_width,
                output,
            } => {
                for i in 0..samples {
                    let waveform = waveform[i].clamp(0.0, 1.0).floor() as u8;
                    phase[i] %= 1.0;
                    output[i] = match waveform {
                        0 => phase[i] * 2.0 - 1.0, // Sawtooth
                        1 => {
                            // PWM
                            if phase[i] > pulse_width[i] {
                                1.0
                            } else {
                                -1.0
                            }
                        }
                        _ => 0.0,
                    };
                    phase[i] += frequency[i] / sample_rate as f32;
                }
            }
        }
        for output in self.outputs() {
            output.write(data);
        }
    }

    pub fn inputs(&mut self) -> impl Iterator<Item = &mut Port> {
        let mut inputs: ArrayVec<&mut Port, 7> = ArrayVec::new();
        match self {
            Node::Abs { input, output: _ } => {
                inputs.push(input);
            }
            Node::Add {
                input_1,
                input_2,
                output: _,
            } => {
                inputs.push(input_1);
                inputs.push(input_2);
            }
            Node::Adsr {
                attack,
                decay,
                sustain,
                release,
                output,
                time,
                gate,
                previous_gate: _,
                voltage: _,
            } => {
                inputs.push(attack);
                inputs.push(decay);
                inputs.push(sustain);
                inputs.push(release);
                inputs.push(output);
                inputs.push(time);
                inputs.push(gate);
            }
            Node::Mul {
                input_1,
                input_2,
                output: _,
            } => {
                inputs.push(input_1);
                inputs.push(input_2);
            }
            Node::Oscillator {
                frequency,
                phase,
                waveform,
                pulse_width,
                output: _,
            } => {
                inputs.push(frequency);
                inputs.push(phase);
                inputs.push(waveform);
                inputs.push(pulse_width);
            }
        }
        inputs.into_iter()
    }

    pub fn outputs(&mut self) -> impl Iterator<Item = &mut Port> {
        let mut outputs: ArrayVec<&mut Port, 1> = ArrayVec::new();
        match self {
            Node::Abs { output, .. } => {
                outputs.push(output);
            }
            Node::Add { output, .. } => {
                outputs.push(output);
            }
            Node::Adsr { output, .. } => {
                outputs.push(output);
            }
            Node::Mul { output, .. } => {
                outputs.push(output);
            }
            Node::Oscillator { output, .. } => {
                outputs.push(output);
            }
        }
        outputs.into_iter()
    }

    pub fn name(&self) -> &'static str {
        NodeKind::from(self).name()
    }
}

impl NodeKind {
    pub fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator + FusedIterator + Copy {
        Self::into_enum_iter()
    }

    pub fn name(&self) -> &'static str {
        match self {
            NodeKind::Abs => "Abs",
            NodeKind::Add => "Add",
            NodeKind::Adsr => "ADSR",
            NodeKind::Mul => "Mul",
            NodeKind::Oscillator => "Oscillator",
        }
    }
}
