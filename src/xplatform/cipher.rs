use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Wire-format names follow IANA AEAD cipher names.
const WIRE_CHACHA20_POLY1305: &str = "AEAD_CHACHA20_POLY1305";
const WIRE_AEGIS256: &str = "AEAD_AEGIS256";
const WIRE_AEGIS256X2: &str = "AEAD_AEGIS256X2";
const WIRE_AEGIS256X4: &str = "AEAD_AEGIS256X4";

/// The set of AEAD cipher suites supported by NepTUN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cipher {
    Chacha20Poly1305,
    Aegis256,
    Aegis256x2,
    Aegis256x4,
}

/// Error returned when a string does not match any known cipher name.
#[derive(Debug)]
pub struct UnknownCipher(pub String);

impl std::fmt::Display for UnknownCipher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown cipher: `{}`", self.0)?;
        Ok(())
    }
}

impl std::error::Error for UnknownCipher {}

impl FromStr for Cipher {
    type Err = UnknownCipher;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            WIRE_CHACHA20_POLY1305 => Ok(Self::Chacha20Poly1305),
            WIRE_AEGIS256 => Ok(Self::Aegis256),
            WIRE_AEGIS256X2 => Ok(Self::Aegis256x2),
            WIRE_AEGIS256X4 => Ok(Self::Aegis256x4),
            other => Err(UnknownCipher(other.to_string())),
        }
    }
}

impl Display for Cipher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cipher::Chacha20Poly1305 => f.write_str(WIRE_CHACHA20_POLY1305),
            Cipher::Aegis256 => f.write_str(WIRE_AEGIS256),
            Cipher::Aegis256x2 => f.write_str(WIRE_AEGIS256X2),
            Cipher::Aegis256x4 => f.write_str(WIRE_AEGIS256X4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_supported_variants() {
        let variants = [
            (Cipher::Chacha20Poly1305, "AEAD_CHACHA20_POLY1305"),
            (Cipher::Aegis256, "AEAD_AEGIS256"),
            (Cipher::Aegis256x2, "AEAD_AEGIS256X2"),
            (Cipher::Aegis256x4, "AEAD_AEGIS256X4"),
        ];
        for (cipher, wire) in &variants {
            assert_eq!(cipher.to_string(), *wire);
            assert_eq!(wire.parse::<Cipher>().unwrap(), *cipher);
        }
    }

    #[test]
    fn unknown_cipher_returns_err() {
        assert!("aes128".parse::<Cipher>().is_err());
        assert!("".parse::<Cipher>().is_err());
    }

    #[test]
    fn ciphers_are_case_sensitive() {
        assert!("aead_aegis256".parse::<Cipher>().is_err());
        assert_eq!("AEAD_AEGIS256".parse::<Cipher>().unwrap(), Cipher::Aegis256);
    }
}
