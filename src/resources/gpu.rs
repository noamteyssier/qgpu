
#[derive (Debug)]
pub struct GPU {
    id: usize,
    mem_total: usize,
    mem_used: usize,
    mem_free: usize,
    fraction_free: f64,
    fraction_used: f64
}

impl GPU {

    pub fn new(
        id: usize,
        mem_total: usize,
        mem_used: usize,
        mem_free: usize) -> Self {

        let fraction_free = (mem_free as f64) / (mem_total as f64);
        let fraction_used = (mem_used as f64) / (mem_total as f64);

        GPU {
            id,
            mem_total,
            mem_used,
            mem_free,
            fraction_free,
            fraction_used
        }
    }

    pub fn get_fraction_free(&self) -> f64 {
        self.fraction_free
    }

    pub fn get_index(&self) -> usize {
        self.id
    }

}
