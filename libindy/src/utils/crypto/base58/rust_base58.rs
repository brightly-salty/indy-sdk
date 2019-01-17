extern crate rust_base58;

use errors::prelude::*;
use failure::{err_msg, ResultExt};
use self::rust_base58::{FromBase58, ToBase58};

pub fn encode(doc: &[u8]) -> String {
    doc.to_base58()
}

pub fn decode(doc: &str) -> Result<Vec<u8>, IndyError> {
    doc.from_base58()
        .map_err(|err| err_msg(format!("Invalid base58 sequence: {:}", err)))
        .context(IndyErrorKind::InvalidStructure)
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let result = encode(&[1, 2, 3]);
        assert_eq!("Ldp", &result, "Got unexpected data");
    }

    #[test]
    fn decode_works() {
        let result = decode("Ldp");

        assert!(result.is_ok(), "Got error");
        assert_eq!(&[1, 2, 3], &result.unwrap()[..], "Get unexpected data");
    }
}