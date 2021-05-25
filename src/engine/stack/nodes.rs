use super::*;

pub trait Node {
    fn process(&mut self, samples: usize, data: &mut StackData, sample_rate: usize);
}

impl Node for NodeEnum {
    fn process(&mut self, samples: usize, data: &mut StackData, sample_rate: usize) {
        self.process(samples, data, sample_rate);
    }
}

pub enum NodeEnum {
    Add {
        input: Port,
        output: Port,
    },
    Abs {
        output: Port,
    },
    Adsr {
        attack: Port,
        decay: Port,
        sustain: Port,
        release: Port,
        output: Port,
        previous_gate: f32,
        gate: Port,
        time: Port,
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
    Pwm {
        frequency: Port,
        phase: Port,
        pulse_width: Port,
        output: Port,
    },
}

impl NodeEnum {
    pub fn process(&mut self, samples: usize, data: &mut StackData, sample_rate: usize) {
        assert!(samples <= 256);
        match self {
            NodeEnum::Add { input, output } => {
                input.get(data);
                output.get(data);
                for i in 0..samples {
                    output[i] += input[i];
                }
                output.set(data);
            }
            NodeEnum::Abs { output } => {
                output.get(data);
                for i in 0..samples {
                    output[i] = output[i].abs();
                }
                output.set(data);
            }
            NodeEnum::Adsr {
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
                time.get(data);
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
                            *voltage = (decay + attack - time[i]) * (1.0 - sustain) / decay + sustain;
                        }
                    } else {
                        *voltage -= sustain / (release * sample_rate as f32);
                        *voltage = (*voltage).max(0.0);
                    }
                    output[i] = *voltage;
                    *previous_gate = gate[i];
                    time[i] += 1.0 / sample_rate as f32;
                }
                time.set(data);
                output.set(data);
            }
            NodeEnum::Mul { volume, audio } => {
                volume.get(data);
                audio.get(data);
                for i in 0..samples {
                    audio[i] *= volume[i];
                }
                audio.set(data);
            }
            NodeEnum::Saw {
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
            NodeEnum::Pwm {
                frequency,
                phase,
                pulse_width,
                output,
            } => {
                frequency.get(data);
                phase.get(data);
                pulse_width.get(data);
                for i in 0..samples {
                    phase[i] %= 1.0;
                    output[i] = if phase[i] > pulse_width[i] { 1.0 } else { -1.0 };
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
    pub fn get(&mut self, data: &StackData) {
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

    pub fn set(&self, data: &mut StackData) {
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
