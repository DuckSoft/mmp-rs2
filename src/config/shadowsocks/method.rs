use serde::{Deserialize, Serialize};

/// Shadowsocks Encryption Methods.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Method {
    #[serde(rename = "chacha20-ietf-poly1305")]
    ChaCha20IETFPoly1305,
    #[serde(rename = "aes-128-gcm")]
    AES128GCM,
    #[serde(rename = "aes-256-gcm")]
    AES256GCM,
}

#[cfg(test)]
mod test {
    use super::Method;

    #[test]
    fn test_deserialize() {
        let method = r#""aes-128-gcm""#;
        let deserialized_method: Method = serde_json::from_str(method).unwrap();
        assert_eq!(deserialized_method, Method::AES128GCM);

        let method = r#""aes-256-gcm""#;
        let deserialized_method: Method = serde_json::from_str(method).unwrap();
        assert_eq!(deserialized_method, Method::AES256GCM);

        let method = r#""chacha20-ietf-poly1305""#;
        let deserialized_method: Method = serde_json::from_str(method).unwrap();
        assert_eq!(deserialized_method, Method::ChaCha20IETFPoly1305);
    }

    #[test]
    fn test_serialize() {
        let method = Method::AES128GCM;
        let serialized_method = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized_method, r#""aes-128-gcm""#);

        let method = Method::AES256GCM;
        let serialized_method = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized_method, r#""aes-256-gcm""#);

        let method = Method::ChaCha20IETFPoly1305;
        let serialized_method = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized_method, r#""chacha20-ietf-poly1305""#);
    }
}
