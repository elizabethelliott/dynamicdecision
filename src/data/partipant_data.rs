extern crate yaml_rust;

use yaml_rust::Yaml;

pub struct ParticipantData {
    pub id: usize,
    pub data: Yaml,
}