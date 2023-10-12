pub mod identity;
pub mod security;
pub mod session;
pub mod storage;
pub mod user;

use std::time::Duration;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Debug)]
enum FlowType {
    Implicit,
    PKCE,
}

#[derive(Debug)]
struct ClientOptions {
    pub url: String,
    pub storage_key: String,
    pub auto_refresh_token: bool,
    pub persist_session: bool,
    pub detect_session_url: bool,
    pub headers: Vec<(String, String)>,
    pub flow_type: FlowType,
    pub debug: bool,
}

// Current session will be checked for refresh at this interval.
pub static AUTO_REFRESH_TICK_DURATION: Duration = Duration::from_secs(30 * 30000);
pub static AUTO_REFRESH_TICK_THRESHOLD: usize = 3;

pub struct GoTrueClient {
    instance_id: usize,

    storage_key: usize,
}
