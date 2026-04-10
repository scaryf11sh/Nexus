use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AlgorithmMetadata {
    pub name: String,
    pub category: String,
    pub complexity_time: String,
    pub complexity_space: String,
    pub description: String,
}