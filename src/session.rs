use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    user: User,
}
