
use serde::Deserialize;



use super::ResourceIndex;#[derive (Debug, Clone, Deserialize)]
pub struct Job {
    command: String,
    args: Option<Vec<String>>,
    env: Option<String>,
    relative_path: Option<String>,
    n_submission: Option<usize>
}
impl Job {


    fn add_move_to_path(&self) -> String {
        match &self.relative_path {
            Some(p) => format!("cd {} ;", p),
            None => "".to_owned()
        }
    }

    fn set_env(&self, node_env: &Option<String>) -> String {
        match &self.env {

            // match on job environment with priority
            Some(e) => format!("conda activate {} ;", e),

            None => match &node_env {

                // otherwise match on node environment
                Some(ne) => format!("conda activate {} ;", ne),

                // no environment to set
                None => "".to_owned()
            }
        }
    }

    fn set_command(&self) -> String {
        format!("{} ", self.command)
    }

    fn set_args(&self) -> String {
        match &self.args {
            Some(args) => {
                let mut s = String::new();
                for a in args {
                    s.push_str(
                        &format!(" {}", a)
                    );
                }
                s.push_str("; ");
                s
            },
            None => "; ".to_owned()
        }
    }

    fn tmux_start(&self) -> String {
        format!("tmux new-session -d \"")
    }

    fn tmux_end(&self) -> String {
        "\"".to_owned()
    }

    fn node_id_var(&self, r: &ResourceIndex) -> String {
        format!("export QG_NODE_ID={}; ", r.get_node_id())
    }

    fn gpu_id_var(&self, r: &ResourceIndex) -> String {
        format!("export QG_GPU_ID={}; ", r.get_gpu_id())
    }

    fn log_path_var(&self, r: &ResourceIndex) -> String {
        format!("export QG_LOG_PATH=\"node{}_gpu{}.log.txt\"; ", r.get_node_id(), r.get_gpu_id())
    }

    pub fn build_command(&self, r: &ResourceIndex, node_env: &Option<String>) -> String {
        let mut command = String::new();
        command.push_str(&self.tmux_start());
        command.push_str("source ~/.bashrc; ");
        command.push_str("export CUDA_DEVICE_ORDER=PCI_BUS_ID; ");
        command.push_str(&self.node_id_var(r));
        command.push_str(&self.gpu_id_var(r));
        command.push_str(&self.log_path_var(r));
        command.push_str(&self.add_move_to_path());
        command.push_str(&self.set_env(node_env));
        command.push_str(&self.set_command());
        command.push_str(&self.set_args());
        command.push_str(&self.tmux_end());

        // println!("{:?}", command);

        command
    }
}
