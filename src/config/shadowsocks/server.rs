use crate::config::shadowsocks::{Method, Password};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Server {
    address: SocketAddr,
    method: Method,
    password: Password,
}

#[cfg(test)]
mod test {
    use super::{Method, Password, Server};
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[test]
    fn test_serialize() {
        let server = Server {
            address: SocketAddr::from_str("127.0.0.1:23333").unwrap(),
            method: Method::ChaCha20IETFPoly1305,
            password: Password::Plain(String::from("114514")),
        };
        let serialized_server = serde_json::to_string(&server).unwrap();
        assert_eq!(
            serialized_server,
            r#"{"address":"127.0.0.1:23333","method":"chacha20-ietf-poly1305","password":"114514"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let server = r#"{"address":"127.0.0.1:23333","method":"chacha20-ietf-poly1305","password":"114514"}"#;
        let deserialized_server: Server = serde_json::from_str(server).unwrap();
        assert_eq!(
            deserialized_server,
            Server {
                address: SocketAddr::from_str("127.0.0.1:23333").unwrap(),
                method: Method::ChaCha20IETFPoly1305,
                password: Password::Plain(String::from("114514")),
            }
        )
    }
}
