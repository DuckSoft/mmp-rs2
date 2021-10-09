use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum Password {
    Plain(String),
    Encoded(EncodedPassword),
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum EncodedPassword {
    #[serde(rename = "hex")]
    Hex(String),
    #[serde(rename = "base64")]
    Base64(String),
}

// TODO: data flow
