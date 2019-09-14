use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct StaticConfiguration {
    server_id: String;
}

#[derive(Deserialize)]
pub struct SecretConfiguration {
    token: String;
}