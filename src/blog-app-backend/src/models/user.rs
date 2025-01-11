use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct User {
    pub id: Principal,
    pub username: String,
    pub password_hash: String
}
