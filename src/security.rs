
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
struct GoTrueMetaSecurity {
    captcha_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecurityEmbed {
    gotrue_meta_security: GoTrueMetaSecurity,
}
