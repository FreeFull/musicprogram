use arrayvec::ArrayVec;

use super::*;

pub type Audio = [f32; 256];
pub type Control = f32;
pub type NodeList = Owned<ArrayVec<Node, 16>>;

pub struct Stack {
    pub nodes: NodeList,
    pub data: StackData,
}

#[derive(Clone, Debug)]
pub struct StackData {
    pub audio: Vec<Audio>,
    pub control: Vec<Control>,
}

impl Stack {
    pub fn new(nodes: NodeList) -> Stack {
        Stack {
            nodes,
            data: StackData::default(),
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

impl Default for StackData {
    fn default() -> Self {
        StackData {
            audio: vec![[0.0; 256]; 256],
            control: vec![0.0; 256],
        }
    }
}
