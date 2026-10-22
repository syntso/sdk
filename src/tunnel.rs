use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Tunnel {
    pub id: u64,
    pub target: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

#[derive(Serialize)]
struct Forward<'a> {
    remote_host: &'a str,
    remote_port: u16,
}

pub fn open(tunnel: &str) -> Result<Tunnel, Error> {
    call("tunnel.open", Some(tunnel), &json!({}))
}

pub fn forward(host: &str, remote_host: &str, remote_port: u16) -> Result<Tunnel, Error> {
    let params = Forward {
        remote_host,
        remote_port,
    };

    call("tunnel.open", Some(host), &params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_to_an_address_seen_from_the_host() {
        let params = Forward {
            remote_host: "db.internal",
            remote_port: 5432,
        };

        assert_eq!(
            serde_json::to_value(params).unwrap(),
            json!({ "remote_host": "db.internal", "remote_port": 5432 })
        );
    }

    #[test]
    fn reads_what_tunnel_open_returns() {
        let tunnel: Tunnel = serde_json::from_value(json!({
            "id": 3, "target": "@orders/api-1", "local_port": 40123, "remote_host": "db.internal",
            "remote_port": 5432, "state": "connected", "active": 0, "connections": 0,
            "sent_bytes": 0, "received_bytes": 0, "last_error": null
        }))
        .unwrap();

        assert_eq!(tunnel.local_port, 40123);
    }
}
