use arrayvec::ArrayVec;

use crate::audio::engine::StackData;

#[derive(Clone, Debug, Default)]
pub struct Ports(ArrayVec<Port, 32>);

impl std::ops::Deref for Ports {
    type Target = ArrayVec<Port, 32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Ports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Index<&'static str> for Ports {
    type Output = Port;

    fn index(&self, index: &'static str) -> &Self::Output {
        self.0.iter().find(|elem| elem.name == index).unwrap()
    }
}

impl std::ops::Index<usize> for Ports {
    type Output = Port;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<&'static str> for Ports {
    fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
        self.0.iter_mut().find(|elem| elem.name == index).unwrap()
    }
}

impl std::ops::IndexMut<usize> for Ports {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Port {
    pub name: &'static str,
    pub stack_index: Option<u8>,
    pub kind: PortKind,
}

impl Port {
    pub fn audio(name: &'static str) -> Self {
        Port {
            name,
            stack_index: None,
            kind: PortKind::Audio([0.0; 256]),
        }
    }

    pub fn control(name: &'static str, value: f32) -> Self {
        Port {
            name,
            stack_index: None,
            kind: PortKind::Control(value),
        }
    }

    pub fn new(name: &'static str, kind: PortKind) -> Self {
        Port {
            name,
            stack_index: None,
            kind,
        }
    }

    pub fn read(&mut self, data: &StackData) {
        if let Some(index) = self.stack_index {
            match &mut self.kind {
                PortKind::Audio(buf) => {
                    *buf = data.audio[index as usize];
                }
                PortKind::Control(buf) => {
                    *buf = data.control[index as usize];
                }
            }
        }
    }

    pub fn write(&self, data: &mut StackData) {
        if let Some(index) = self.stack_index {
            match self.kind {
                PortKind::Audio(audio) => {
                    data.audio[index as usize].copy_from_slice(&audio);
                }
                PortKind::Control(control) => {
                    data.control[index as usize] = control;
                }
            }
        }
    }
}

impl std::ops::Index<usize> for Port {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.kind[index]
    }
}

impl std::ops::IndexMut<usize> for Port {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.kind[index]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PortKind {
    Audio([f32; 256]),
    Control(f32),
}

impl std::ops::Index<usize> for PortKind {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match self {
            PortKind::Audio(audio) => &audio[index],
            PortKind::Control(control) => control,
        }
    }
}

impl std::ops::IndexMut<usize> for PortKind {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            PortKind::Audio(audio) => &mut audio[index],
            PortKind::Control(control) => control,
        }
    }
}

impl Default for PortKind {
    fn default() -> Self {
        PortKind::Audio([0.0; 256])
    }
}
