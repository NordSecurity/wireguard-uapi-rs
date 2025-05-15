use std::ops::{Deref, DerefMut};

use zeroize::{Zeroize, ZeroizeOnDrop};

/// Key material, public, private or preshared.
#[derive(Clone, Debug, Default, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Key([u8; 32]);

impl Deref for Key {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Key {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Key {
    fn from(mut value: [u8; 32]) -> Self {
        let ret = Self(value);
        value.zeroize();
        ret
    }
}
