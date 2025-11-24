use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub version: String,
    pub game_dir: String,
    pub java_path: String,
    pub max_ram: u32,
}

pub fn load_instances() -> Vec<Instance> {
    let data = fs::read_to_string("instances.json")
        .expect("Unable to read file");

    serde_json::from_str(&data)
        .expect("Invalid JSON in instances.json")
}

pub fn save_instances(instances: &Vec<Instance>) {
    
}