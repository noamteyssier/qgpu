
use serde::Deserialize;

#[derive (Debug, Deserialize)]
pub struct SerNode {
    name: String,
    env: Option<String>
}
impl SerNode {

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_env(&self) -> Option<String> {
        self.env.clone()
    }
}
