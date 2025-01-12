use candid::{CandidType, Int};
use serde::{Deserialize, Serialize};




#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct Error {
    pub status: String,
    pub code: Int,
    pub message: String
}