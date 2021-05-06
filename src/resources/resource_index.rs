
use std::fmt;

#[derive (Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct ResourceIndex {
    node_id: usize,
    gpu_id: usize
}

impl fmt::Display for ResourceIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n{}_g{}", self.node_id, self.gpu_id)
    }
}

impl ResourceIndex {
    pub fn new(node_id: usize, gpu_id: usize) -> Self {
        ResourceIndex {node_id, gpu_id}
    }

    pub fn get_node_id(&self) -> usize {
        self.node_id
    }

    pub fn get_gpu_id(&self) -> usize {
        self.gpu_id
    }
}
