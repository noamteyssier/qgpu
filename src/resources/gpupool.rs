
use super::GPU;

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

    pub fn parse_value(line: &str) -> usize {

        line.split_whitespace().nth(2)
            .expect("Error: Could not index to expected memory usage")
            .parse::<usize>()
            .expect("Error: Could not parse memory usage to usize")

    }

    pub fn from_smi(&mut self, nvidia_smi: &str) {

        let mut lines = nvidia_smi.lines();
        let mut gpu_index = 0;

        while let Some(line) = lines.next() {

            if line.trim() == "Utilization" {

                let gpu_usage = GPUPool::parse_value(
                    lines.next().expect("Error: Unexpected End of SMI (GPU USAGE)")
                );
                let gpu_memory = GPUPool::parse_value(
                    lines.next().expect("Error: Unexpected End of SMI (GPU USAGE)")
                );


                let gpu = GPU::new(gpu_index, gpu_usage, gpu_memory);

                self.add_gpu(gpu);
                gpu_index += 1;
            }
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
