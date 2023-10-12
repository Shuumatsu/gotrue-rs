use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Factor {
    id: u128,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    status: String,
    friendly_name: Option<String>,
    factor_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    // Define Identity struct if needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u128,

    aud: String,
    role: String,
    email: String,
    email_confirmed_at: Option<NaiveDateTime>,
    invited_at: Option<NaiveDateTime>,

    phone: String,
    phone_confirmed_at: Option<NaiveDateTime>,

    confirmation_sent_at: Option<NaiveDateTime>,

    recovery_sent_at: Option<NaiveDateTime>,

    email_change: Option<String>,
    email_change_sent_at: Option<NaiveDateTime>,

    phone_change: Option<String>,
    phone_change_sent_at: Option<NaiveDateTime>,

    reauthentication_sent_at: Option<NaiveDateTime>,

    last_sign_in_at: Option<NaiveDateTime>,

    app_metadata: Value, // Assuming you want to use serde_json::Value for dynamic data
    user_metadata: Value, // Assuming you want to use serde_json::Value for dynamic data

    factors: Vec<Factor>, // Use Option<Vec<Factor>> if factors can be null
    identities: Vec<Identity>,

    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    banned_until: Option<NaiveDateTime>,

    // ConfirmedAt is deprecated. Use EmailConfirmedAt or PhoneConfirmedAt instead.
    confirmed_at: NaiveDateTime,
}
