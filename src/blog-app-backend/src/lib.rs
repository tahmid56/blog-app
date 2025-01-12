mod models;
use std::cell::RefCell;
use std::collections::HashMap;

use candid::{Int, Principal};
use ic_cdk::{println, query, update};

use models::{Error, Response, User, Blog};

#[derive(Debug, Clone)]
struct Session {
    user_id: Int,
    expires_at: u64,
}

thread_local! {
    static USERS: RefCell<Vec<User>> = RefCell::default();
    static SESSIONS: RefCell<HashMap<Int, Session>> = RefCell::default();
    static BLOGS: RefCell<Vec<Blog>> = RefCell::default();
}

#[update]
pub fn register(username: String, password_hash: String) -> Result<Response<String>, Error> {
    let user_exists = USERS.with(|users| {
        users.borrow().iter().any(|user| user.username == username)
    });
    if user_exists {
        return Err(Error {
            status: "error".to_string(),
            code: Int::from(400),
            message: "User already exists".to_string(),
        });
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        let user = User {
            id: Int::from(users.len() as u32),
            username,
            password_hash,
        };
        users.push(user.clone());
        user
    });

    Ok(Response {
        status: "success".to_string(),
        code: Int::from(201),
        data: "User registered successfully".to_string(),
    })
}

#[update]
pub fn login(username: String, password_hash: String) -> Result<Response<String>, Error> {
    let user = USERS.with(|users| {
        users.borrow().iter().find(|u| u.username == username && u.password_hash == password_hash).cloned()
    });

    match user {
        Some(user) => {
            
            let session = Session {
                user_id: user.id.clone(),
                expires_at: ic_cdk::api::time() + 3600 * 24 * 7, // Session expires in 7 days
            };

            SESSIONS.with(|sessions| {
                sessions.borrow_mut().insert(user.id, session);
            });

            Ok(Response {
                status: "success".to_string(),
                code: Int::from(200),
                data: "Login successful".to_string(),
            })
        }
        None => Err(Error {
            status: "error".to_string(),
            code: Int::from(401),
            message: "Invalid username or password".to_string(),
        })
    }
}

#[query]
pub fn is_logged_in(user_id: Int) -> bool {
    SESSIONS.with(|sessions| {
        sessions.borrow().get(&user_id).map_or(false, |session| session.user_id == user_id)
    })
}

#[update]
pub fn logout(user_id: Int) -> Response<String> {
    let removed = SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        sessions.remove(&user_id)
    });

    match removed {
        Some(session) if session.user_id == user_id => {
            Response {
                status: "success".to_string(),
                code: Int::from(200),
                data: "Logged out successfully".to_string(),
            }
        },
        Some(_) => {
            Response {
                status: "error".to_string(),
                code: Int::from(403),
                data: "User ID does not match the session".to_string(),
            }
        },
        None => {
            Response {
                status: "error".to_string(),
                code: Int::from(404),
                data: "No active session found".to_string(),
            }
        }
    }
}


#[update]
pub fn create_blog(title: String, content: String, user_id: Int) -> Result<Response<String>, Error> {
    
    // Check if the user is logged in
    let user_id = SESSIONS.with(|sessions| {
        sessions.borrow().iter().find(|(_, session)| session.user_id == user_id).map(|(_, session)| session.user_id.clone())
    });
    

    match user_id {
        Some(author_id) => {
            let blog = BLOGS.with(|blogs| {
                let mut blogs = blogs.borrow_mut();
                let blog = Blog {
                    id: Int::from((blogs.len()+1) as u32),
                    title,
                    content,
                    author_id,
                    created_at: ic_cdk::api::time(),
                };
                blogs.push(blog.clone());
                blog
            });

            Ok(Response {
                status: "success".to_string(),
                code: Int::from(201),
                data: format!("Blog created successfully with ID: {}", blog.id),
            })
        },
        None => Err(Error {
            status: "error".to_string(),
            code: Int::from(401),
            message: "User not logged in".to_string(),
        }),
    }
}


#[query]
pub fn get_all_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| {
        blogs.borrow().clone()
    })
}

#[update]
pub fn delete_blog(blog_id: Int, user_id: Int) -> Result<Response<String>, Error> {
    let blog = BLOGS.with(|blogs| {
        blogs.borrow().iter().find(|blog| blog.id == blog_id).cloned()
    });
    match blog {
        Some(blog) => {
            if blog.author_id == user_id {
                BLOGS.with(|blogs| {
                    let mut blogs = blogs.borrow_mut();
                    blogs.retain(|b| b.id != blog_id);
                });
                Ok(Response {
                    status: "success".to_string(),
                    code: Int::from(200),
                    data: format!("Blog with ID {} deleted successfully", blog_id),
                })
            }else{
                Err(Error {
                    status: "error".to_string(),
                    code: Int::from(403),
                    message: "User not authorized to delete this blog".to_string(),
                })
            }
        },
        None => Err(Error {
            status: "error".to_string(),
            code: Int::from(404),
            message: "Blog not found".to_string(),
        }),
    }
}

