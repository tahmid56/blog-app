mod models;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, storage, update};
use models::User;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cell::RefCell;


#[derive(Clone, CandidType, Deserialize)]
struct Session {
    user_id: Principal,
    expires_at: u64,
}

type SessionStore = HashMap<String, Session>;

#[derive(CandidType, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(CandidType, Deserialize)]
struct LoginResponse {
    session_id: String,
}

// Initialize session store
thread_local! {
    static SESSION_STORE: RefCell<(Vec<Session>,)> = RefCell::new(storage::stable_restore().unwrap_or_default());
}

#[query]
fn login(request: LoginRequest) -> LoginResponse {
    let user = User {
        id: ic_cdk::caller(),
        username: request.username,
        password_hash: "hashed_password".to_string(),
    };

    let session_id = create_session(user.id);
    LoginResponse { session_id }
}

fn create_session(user_id: Principal) -> String {
    let session_id = ic_cdk::id().to_string(); 
    let session = Session {
        user_id,
        expires_at: ic_cdk::api::time() / 1_000_000_000 + 86400, 
    };

    SESSION_STORE.with(|store| {
        store.borrow_mut().0.push(session);
    });

    session_id
}

fn current_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn auth_middleware(session_id: Principal) -> Result<Principal, String> {
    SESSION_STORE.with(|store| {
        let store = store.borrow();
        
        store.0.iter().find(|session| session.user_id == session_id)
            .map_or(Err("Invalid session".to_string()), |session| {
                if session.expires_at > current_timestamp() {
                    Ok(session.user_id)
                } else {
                    Err("Session expired".to_string())
                }
            })
    })
}

#[query]
fn protected_route(session_id: String) -> String {
    let session_id = Principal::from_text(session_id).unwrap();
    match auth_middleware(session_id) {
        Ok(user_id) => format!("Hello, user {}!", user_id),
        Err(e) => format!("Authentication failed: {}", e),
    }
}

// Function to clear expired sessions (should be called periodically)
#[update]
fn clear_expired_sessions() {
    let current_time = current_timestamp();
    SESSION_STORE.with(|store| {
        let mut store = store.borrow_mut();
        
        store.0.retain(|session| session.expires_at > current_time);
    });
}