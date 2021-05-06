
use openssh::Session;
use super::{GPUPool, ResourceIndex, Job};

#[derive (Debug)]
pub struct Node {
    id: usize,
    session: Session,
    gpu_pool: GPUPool,
    n_gpus: Option<usize>,
    name: String,
    env: Option<String>
}

impl Node {

    pub fn new(id: usize, session: Session) -> Self {
        Node {
            id,
            session,
            gpu_pool: GPUPool::new(),
            n_gpus: None,
            name: String::new(),
            env: None
        }
    }

    pub fn get_index(&self) -> usize {
        self.id
    }

    pub fn add_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_env(&mut self, env: Option<String>) {
        self.env = env;
    }

    pub fn get_env(&self) -> &Option<String> {
        &self.env
    }

    #[allow (dead_code)]
    pub async fn ls(&self) {

        let ls = self.session
            .command("ls")
            .output()
            .await
            .expect("Error: Could not query ls");

        let ls_output = String::from_utf8(ls.stdout)
            .expect("Error: could not convert ls to UTF-8");

        eprintln!(
            "{}", ls_output
        );

    }

    pub async fn gpu_usage(&mut self) {

        let query_gpu_mem = self.session
            .command("bash")
            .arg("-c")
            .arg("nvidia-smi -q -d UTILIZATION")
            .output()
            .await
            .expect("Error: Could not query memory");

        let query_output = String::from_utf8(query_gpu_mem.stdout)
            .expect("Error: could not convert query to UTF-8");

        self.gpu_pool.from_smi(&query_output);
    }

    pub async fn available_gpus(&self,
            usage_free_threshold: usize,
            memory_free_threshold: usize) -> Vec<ResourceIndex> {

        self.gpu_pool.available_gpus(usage_free_threshold, memory_free_threshold)
            .iter()
            .map(|gpu_index|
                ResourceIndex::new(self.get_index(), *gpu_index)
            )
            .collect()


    }

    pub async fn run_job(&self, r: &ResourceIndex, j: Job) {
        println!("starting job on resource: {}", r);

        self.session
            .command("bash")
            .arg("-c")
            .arg(j.build_command(r, self.get_env()))
            .output()
            .await
            .expect("Error: Could not submit job");
    }
}
