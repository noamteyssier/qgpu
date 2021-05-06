
use futures::future::join_all;

use super::{Node, ResourceIndex, Job};

pub struct NodePool {
    nodes: Vec<Node>
}
impl NodePool {
    pub fn new(nodes: Vec<Node>) -> Self {
        NodePool {
            nodes
        }
    }

    pub fn get_node(&self, r: &ResourceIndex) -> &Node {
        &self.nodes[r.get_node_id()]
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    #[allow (dead_code)]
    pub async fn query_ls(&self) {
        let mut v = Vec::new();

        for i in 0..self.nodes.len() {
            v.push(
                self.nodes[i].ls()
            )
        }

        join_all(v).await;
    }

    pub async fn query_gpus(&mut self) {
        let mut v = Vec::new();

        for n in self.nodes.iter_mut() {
            v.push(n.gpu_usage())
        }

        join_all(v).await;
    }

    pub async fn available_gpus(&self,
        usage_free_threshold: usize,
        memory_free_threshold: usize) -> Vec<ResourceIndex> {

        let mut v = Vec::new();

        // prepare query available gpus passing free usage and memory threshold
        for n in self.nodes.iter() {
            v.push(n.available_gpus(usage_free_threshold, memory_free_threshold))
        }

        // perform parallel query
        let resources = join_all(v).await;

        // return all available resources as vector
        let mut all_resources = resources
            .into_iter()
            .flatten()
            .collect::<Vec<ResourceIndex>>();

        all_resources.sort_by_key(|k| k.get_gpu_id());

        all_resources

    }

    pub async fn run_job(&self, resource: ResourceIndex, job: Job) {
        let n = self.get_node(&resource);
        let job = n.run_job(&resource, job);
        job.await;
    }
}
