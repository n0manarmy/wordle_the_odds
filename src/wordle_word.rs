use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct WordleWord {
    pub word: &'static str,
    pub zipf_dist: f32,
}