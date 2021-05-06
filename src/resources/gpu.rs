
#[derive (Debug)]
pub struct GPU {
    id: usize,
    gpu_usage: usize,
    gpu_memory: usize,
    fraction_usage_free: usize,
    fraction_memory_free: usize,
}

impl GPU {

    pub fn new(id: usize, gpu_usage: usize, gpu_memory: usize) -> Self {

        let fraction_usage_free = 100 - gpu_usage;
        let fraction_memory_free = 100 - gpu_memory;

        GPU {
            id,
            gpu_usage,
            gpu_memory,
            fraction_usage_free,
            fraction_memory_free
        }
    }

    pub fn get_usage_free(&self) -> usize {
        self.fraction_usage_free
    }

    pub fn get_memory_free(&self) -> usize {
        self.fraction_memory_free
    }

    pub fn get_index(&self) -> usize {
        self.id
    }

    pub fn is_available(&self,
        usage_free_threshold: usize,
        memory_free_threshold: usize) -> bool {

        (self.get_usage_free() >= usage_free_threshold) &
            (self.get_memory_free() >= memory_free_threshold)

    }

}
