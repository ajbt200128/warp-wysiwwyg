use serde::Deserialize;
use std::process::Command;
use std::{fs, net::IpAddr, net::Ipv4Addr, net::SocketAddr};

#[derive(Deserialize)]
struct Config {
    server: Option<ServerConfig>,
}

#[derive(Deserialize)]
struct ServerConfig {
    ip_addr: Option<Ipv4Addr>,
    port: Option<u16>,
    tls_cert: Option<String>,
    tls_key: Option<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=Settings.toml");
    let config = get_config();
    set_server(config.server);
}

fn get_config() -> Config {
    let config_str = match fs::read_to_string("Settings.toml") {
        Err(_) => panic!("Error, Settings.toml is missing!"),
        Ok(config_str) => config_str,
    };
    match toml::from_str(&config_str) {
        Err(err) => panic!("Error in Settings.toml: {}", err),
        Ok(config) => config,
    }
}

fn set_server(server_settings: Option<ServerConfig>) {
    let server_settings = server_settings.unwrap_or(ServerConfig {
        ip_addr: None,
        port: None,
        tls_cert: None,
        tls_key: None,
    });
    let port = server_settings.port.unwrap_or(3030);
    let ip = server_settings
        .ip_addr
        .unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1));
    let socket_addr = SocketAddr::new(IpAddr::V4(ip), port);
    if server_settings.tls_key.is_some() && server_settings.tls_cert.is_some() {
        if cfg!(feature = "autoreload") {
            println!(
                "cargo:warning=TLS is enabled, but so is autoreload, autoreload cannot use tls"
            );
        } else {
            println!(
                "cargo:rustc-env=TLS_KEY={}",
                server_settings.tls_key.unwrap()
            );
            println!(
                "cargo:rustc-env=TLS_CERT={}",
                server_settings.tls_cert.unwrap()
            );
        }
    }
    println!("cargo:rustc-env=IP_ADDR={}", socket_addr);
}
