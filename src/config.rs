use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct StaticConfiguration {
    pub token: String,
    pub server_id: String
}