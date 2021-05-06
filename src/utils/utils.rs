
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;

use crate::resources::{Node, SerNode, Job};
use openssh::Session;
use futures::future::join_all;

async fn _open_session(sernodes: &Vec<SerNode>) -> Vec<Result<Session, openssh::Error>>{

    let mut sessions = Vec::new();

    for sn in sernodes {

        let session_name = sn.get_name();
        println!("opening : {}", session_name);

        let session = openssh::Session::connect(
            session_name,
            openssh::KnownHosts::Strict
        );

        sessions.push(session);
    }

    join_all(sessions).await
}

pub async fn open_sessions(sernodes: Vec<SerNode>) -> Vec<Node> {

    let mut nodes = Vec::new();
    let sessions = _open_session(&sernodes).await;

    let mut node_index = 0;
    for (i, s) in sessions.into_iter().enumerate() {

        let mut n = Node::new(
            node_index,
            s.expect("Error: Failed to build session")
        );

        n.add_env(sernodes[i].get_env());
        n.add_name(sernodes[i].get_name());

        nodes.push(n);
        node_index += 1;
    }

    nodes
}

pub fn read_node_config(path: &Path) -> Vec<SerNode> {
    let file = File::open(path).expect("Error: Could not open Node Config File");
    let reader = BufReader::new(file);

    let deserializer = serde_json::Deserializer::from_reader(reader);

    deserializer.into_iter::<SerNode>()
        .map(|x| x.expect("Error: Could not parase Node"))
        .collect()

}

pub fn read_job_config(path: &Path) -> VecDeque<Job> {

    let file = File::open(path).expect("Error: Could not open Node Config File");
    let reader = BufReader::new(file);

    let deserializer = serde_json::Deserializer::from_reader(reader);

    deserializer.into_iter::<Job>()
        .map(|x| x.expect("Error: Could not parase Node"))
        .collect()

}
