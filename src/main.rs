
use std::path::Path;

mod resources;
mod utils;

use resources::NodePool;
use utils::{open_sessions, read_node_config, read_job_config};

#[tokio::main]
async fn main() {

    let usage_free_threshold = 90;
    let memory_free_threshold = 85;

    let fn_node_config = "node_pool.json";
    let fn_job_config = "jobs.json";

    let node_config_path = Path::new(fn_node_config);
    let job_config_path = Path::new(fn_job_config);

    let sernodes = read_node_config(node_config_path);
    let mut jobs = read_job_config(job_config_path);

    let nodes = open_sessions(sernodes).await;
    let mut node_pool = NodePool::new(nodes);

    node_pool.query_gpus().await;
    let resources = node_pool.available_gpus(usage_free_threshold, memory_free_threshold).await;

    for r in resources {
        match jobs.pop_front() {
            Some(j) => {
                node_pool.run_job(r, j).await;
            },
            None => {
                break;
            }
        };
    }


}
