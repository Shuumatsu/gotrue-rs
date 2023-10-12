use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
struct Identity {
    id: String,
    user_id: u128,
    identity_data: Option<Value>,
    provider: String,
    last_sign_in_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
