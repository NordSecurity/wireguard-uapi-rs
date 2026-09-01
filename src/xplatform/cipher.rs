use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The set of AEAD cipher suites supported by NepTUN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cipher {
    Chacha20Poly1305,
    Aegis256,
    Aegis256x2,
    Aegis256x4,
}

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
        match s.trim().to_ascii_lowercase().as_str() {
            "chacha20poly1305" => Ok(Self::Chacha20Poly1305),
            "aegis256" => Ok(Self::Aegis256),
            "aegis256x2" => Ok(Self::Aegis256x2),
            "aegis256x4" => Ok(Self::Aegis256x4),
            other => Err(UnknownCipher(other.to_string())),
        }
    }
}

impl Display for Cipher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cipher::Chacha20Poly1305 => f.write_str("chacha20poly1305"),
            Cipher::Aegis256 => f.write_str("aegis256"),
            Cipher::Aegis256x2 => f.write_str("aegis256x2"),
            Cipher::Aegis256x4 => f.write_str("aegis256x4"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_supported_variants() {
        let variants = [
            (Cipher::Chacha20Poly1305, "chacha20poly1305"),
            (Cipher::Aegis256, "aegis256"),
            (Cipher::Aegis256x2, "aegis256x2"),
            (Cipher::Aegis256x4, "aegis256x4"),
        ];
        for (cipher, wire) in &variants {
            assert_eq!(cipher.to_string(), *wire);
            assert_eq!(wire.parse::<Cipher>().unwrap(), *cipher);
        }
    }

    #[test]
    fn case_insensitive_parsing() {
        assert_eq!(
            "CHACHA20POLY1305".parse::<Cipher>().unwrap(),
            Cipher::Chacha20Poly1305
        );
        assert_eq!("Aegis256".parse::<Cipher>().unwrap(), Cipher::Aegis256);
        assert_eq!("AEGIS256X2".parse::<Cipher>().unwrap(), Cipher::Aegis256x2);
        assert_eq!("AeGiS256X4".parse::<Cipher>().unwrap(), Cipher::Aegis256x4);
    }

    #[test]
    fn unknown_cipher_returns_err() {
        assert!("aes128".parse::<Cipher>().is_err());
        assert!("".parse::<Cipher>().is_err());
    }
}
