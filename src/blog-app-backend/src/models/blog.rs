use candid::{CandidType, Int};
use serde::{Deserialize};



#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Blog {
    pub id: Int,
    pub title: String,
    pub content: String,
    pub author_id: Int,
    pub created_at: u64,
}