use candid::CandidType;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}