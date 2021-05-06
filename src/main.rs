
use std::path::Path;

mod resources;
mod utils;

use resources::NodePool;
use utils::{
    open_sessions,
    read_node_config,
    read_job_config,
    get_args
};

async fn stat(
        node_config_path: &str,
        usage_free_threshold: usize,
        memory_free_threshold: usize) {


    let node_config = Path::new(node_config_path);
    let sernodes = read_node_config(node_config);
    let nodes = open_sessions(sernodes).await;
    let mut node_pool = NodePool::new(nodes);

    node_pool.query_gpus().await;
    for n in node_pool.get_nodes() {
        println!(
            "{}", n.get_format_print(
                usage_free_threshold,
                memory_free_threshold
            )
        )
    }

    // let resources = node_pool.available_gpus(
    //     usage_free_threshold, memory_free_threshold
    // ).await;

    // for r in resources {
    //     println!("{}", node_pool.get_node(&r));
    // }

}

async fn sub(
        node_config_path: &str,
        job_config_path: &str,
        usage_free_threshold: usize,
        memory_free_threshold: usize,
        dry_run: bool) {

    let node_config = Path::new(node_config_path);
    let job_config = Path::new(job_config_path);


    let sernodes = read_node_config(node_config);
    let mut jobs = read_job_config(job_config);

    let nodes = open_sessions(sernodes).await;
    let mut node_pool = NodePool::new(nodes);

    node_pool.query_gpus().await;
    let resources = node_pool.available_gpus(
        usage_free_threshold, memory_free_threshold
    ).await;

    for r in resources {
        match jobs.pop_front() {
            Some(j) => {

                if !dry_run {
                    println!(
                        "Submitting:\n>>>Job:\n    {:?}\n>>>Resource:\n    {}",
                        j, node_pool.print_resource(&r)
                    );
                    node_pool.run_job(r, j).await;
                }
                else {
                    println!(
                        "Will Submit:\n>>>Job:\n    {:?}\n>>>Resource:\n    {}",
                        j, node_pool.print_resource(&r)
                    );
                    println!();
                }
            },
            None => {
                break;
            }
        };
    }


}


#[tokio::main]
async fn main() {

    let matches = get_args();

    match matches.subcommand() {

        ("stat", Some(sub_m)) => {
            stat(
                sub_m.value_of("nodes")
                    .unwrap(),
                sub_m.value_of("usage_free_threshold")
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error: Unable to parse free usage sto int"),
                sub_m.value_of("memory_free_threshold")
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error: Unable to parse free memory to int")

            ).await;
        },

        ("sub", Some(sub_m)) => {
            sub(
                sub_m.value_of("nodes")
                    .unwrap(),
                sub_m.value_of("jobs")
                    .unwrap(),
                sub_m.value_of("usage_free_threshold")
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error: Unable to parse free usage sto int"),
                sub_m.value_of("memory_free_threshold")
                    .unwrap()
                    .parse::<usize>()
                    .expect("Error: Unable to parse free memory to int"),
                sub_m.is_present("dry")
            ).await;
        }

        _ => unreachable!()

    };

}
