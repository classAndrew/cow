use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub sql_server_ip: String,
    pub sql_server_port: u16,
    pub sql_server_username: String,
    pub sql_server_password: String,
    pub cmd_prefix: String,
    pub application_id: u64
}