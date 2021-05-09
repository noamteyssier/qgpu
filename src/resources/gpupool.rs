
use super::{GPU, ResourceIndex};

#[derive (Debug)]
pub struct GPUPool {
    pool: Vec<GPU>
}
impl GPUPool {

    pub fn new() -> Self {
        GPUPool {
            pool: Vec::new()
        }
    }

    pub fn add_gpu(&mut self, gpu: GPU) {
        self.pool.push(gpu)
    }

    pub fn get_gpu(&self, r: &ResourceIndex) -> &GPU {
        &self.pool[r.get_gpu_id()]
    }

    pub fn get_gpus(&self) -> &Vec<GPU> {
        &self.pool
    }

    pub fn from_smi(&mut self, nvidia_smi: &str) {

        let mut lines = nvidia_smi.lines();
        while let Some(gpu_line) = lines.next() {

            let gpu_query: Vec<&str> = gpu_line.split(",")
                .map(|x| x.trim())
                .collect();

            let name = gpu_query[0].to_owned();
            let id = gpu_query[1].parse::<usize>()
                .expect("Error: Could not parse id to int");
            let util_gpu = gpu_query[2].parse::<usize>()
                .expect("Error: Could not parse gpu_usage to int");
            let util_mem = gpu_query[3].parse::<usize>()
                .expect("Error: Could not parse mem_usage to int");
            let mem_total = gpu_query[4].parse::<usize>()
                .expect("Error: Could not parse mem_total to int");
            let mem_free = gpu_query[5].parse::<usize>()
                .expect("Error: Could not parse mem_total to int");
            let mem_used = gpu_query[6].parse::<usize>()
                .expect("Error: Could not parse mem_total to int");

            let gpu = GPU::new(
                name, id, util_gpu, util_mem, mem_total, mem_free, mem_used
            );

            self.add_gpu(gpu);
        }

    }

    pub fn available_gpus(&self,
            usage_free_threshold: usize,
            memory_free_threshold: usize
            ) -> Vec<usize> {

        self.pool.iter()
            .filter(|g| g.is_available(usage_free_threshold, memory_free_threshold))
            .map(|g| g.get_index())
            .collect()
    }
}
