use candid::{CandidType, Int, Principal};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct User {
    pub id: Int,
    pub username: String,
    pub password_hash: String
}

