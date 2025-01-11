use candid::CandidType;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub author_id: u32,
    pub blog_id: u32
}