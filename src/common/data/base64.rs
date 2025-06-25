use base64::engine::{general_purpose, Engine};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, BinaryDecodeError>;

#[derive(Error, Debug)]
pub enum BinaryDecodeError {
    #[error("Failed to decode data")]
    FailToB64uDecode,
}

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(|_| BinaryDecodeError::FailToB64uDecode)
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    b64u_decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(BinaryDecodeError::FailToB64uDecode)
}
