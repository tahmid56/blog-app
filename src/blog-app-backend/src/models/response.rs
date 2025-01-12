use candid::{CandidType, Int};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct Response <T>{
    pub status: String,
    pub code: Int,
    pub data: T
}