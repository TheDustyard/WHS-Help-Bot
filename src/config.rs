use serde::[Deserialize];

#[derive(Deserialize)]
pub struct StaticConfiguration {
    token: String;
    server_id: String;
}