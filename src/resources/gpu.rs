
use std::fmt;
use colored::*;

#[derive (Debug)]
pub struct GPU {
    id: usize,
    gpu_usage: usize,
    gpu_memory: usize,
    fraction_usage_free: usize,
    fraction_memory_free: usize,
}

impl fmt::Display for GPU {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GPU_{}: usage_free: {}, memory_free: {}",
            self.id, self.fraction_usage_free, self.fraction_memory_free
        )
    }
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

    pub fn get_format_print(&self,
        usage_free_threshold: usize,
        memory_free_threshold: usize) -> ColoredString {

        let available = self.is_available(
            usage_free_threshold,
            memory_free_threshold
        );

        let s = format!(
            "GPU_{}: usage_free: {}, memory_free: {}, available: {}",
            self.id,
            self.fraction_usage_free,
            self.fraction_memory_free,
            available
        );

        match available {
            true => s.blue(),
            false => s.red()
        }

    }

    pub fn is_available(&self,
        usage_free_threshold: usize,
        memory_free_threshold: usize) -> bool {

        (self.get_usage_free() >= usage_free_threshold) &&
            (self.get_memory_free() >= memory_free_threshold)

    }

}
