
use std::fmt;
use colored::*;

#[derive (Debug)]
pub struct GPU {
    name: String,
    id: usize,
    util_gpu: usize,
    util_mem: usize,
    mem_total: usize,
    mem_free: usize,
    mem_used:usize,
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

    pub fn new(
            name: String,
            id: usize,
            util_gpu: usize,
            util_mem: usize,
            mem_total: usize,
            mem_free: usize,
            mem_used: usize
            ) -> Self {


        let fraction_usage_free = 100 - util_gpu;
        let fraction_memory_free = (
            100.0 * (mem_free as f64 / mem_total as f64)
        ).round() as usize;


        GPU {
            name,
            id,
            util_gpu,
            util_mem,
            mem_total,
            mem_free,
            mem_used,
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
            "GPU_{}: name: {}, usage_free: {}, memory_free: {}, available: {}",
            self.id,
            self.name,
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
