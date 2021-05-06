
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

    pub fn parse_memory(line: &str) -> usize {

        line.split_whitespace().nth(2)
            .expect("Error: Could not index to expected memory usage")
            .parse::<usize>()
            .expect("Error: Could not parse memory usage to usize")

    }

    pub fn from_smi(&mut self, nvidia_smi: &str) {

        let mut lines = nvidia_smi.lines();
        let mut gpu_index = 0;

        while let Some(line) = lines.next() {

            if line.contains("FB Memory Usage") {

                let mem_total = GPUPool::parse_memory(
                    lines.next().expect("Error: Unexpected End of SMI (TOTAL)")
                );
                let mem_used = GPUPool::parse_memory(
                    lines.next().expect("Error: Unexpected End of SMI (USED)")
                );
                let mem_free = GPUPool::parse_memory(
                    lines.next().expect("Error: Unexpected End of SMI (FREE)")
                );

                let gpu = GPU::new(gpu_index, mem_total, mem_used, mem_free);

                self.add_gpu(gpu);
                gpu_index += 1;
            }
        }

    }

    pub fn available_gpus(&self, free_threshold: f64) -> Vec<usize> {
        self.pool.iter()
            .filter(|g| g.get_fraction_free() >= free_threshold)
            .map(|g| g.get_index())
            .collect()
    }
}
