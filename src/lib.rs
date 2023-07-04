use std::fmt::Debug;
use std::ops::Deref;

use dynarray::DynArray;

/// immutable owned String type
///
/// biggest advantage over std::String is that it saves the 8 capacity bytes of the Vec of String
#[derive(Default, Clone)]
pub struct IString(DynArray<u8>);

impl IString {
    pub fn new(s: &str) -> Self {
        // ImmutableString(s.as_bytes().into())
        Self::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        IString(bytes.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn is_valid_utf8(&self) -> bool {
        // it's fine to call this as std::str::from_utf8 will just do UTF-8 validation + a transmute
        std::str::from_utf8(&self.0).is_ok()
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
    }
}

impl Deref for IString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl Debug for IString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid_utf8() {
            Debug::fmt(self.as_str().unwrap(), f)
        } else {
            Debug::fmt(self.as_bytes(), f)
        }
    }
}

impl From<&str> for IString {
    fn from(s: &str) -> Self {
        IString::new(s)
    }
}

impl From<&[u8]> for IString {
    fn from(bytes: &[u8]) -> Self {
        IString::from_bytes(bytes)
    }
}

// could do this via &[char] -> Iterator<&char> -> String -> &str -> ImmutableString
// but this is more efficient, in particular avoiding the additional allocation of the String
impl From<&[char]> for IString {
    fn from(chars: &[char]) -> Self {
        // calculate total number of bytes for the UTF-8 representation of chars
        let byte_len: usize = chars.iter().map(|c| c.len_utf8()).sum();

        // NOTE: could potentially make this new_uninit
        let mut array = DynArray::new(byte_len);

        let mut idx = 0;
        // let mut buf: [u8; 4] = [0; 4];

        for c in chars.iter().cloned() {
            // for byte in c.encode_utf8(&mut buf).as_bytes().iter().cloned() {
            //     array[idx].write(byte);
            //     idx += 1;
            // }

            // encode directly into the array instead of going through a buffer first
            let bytes_written = c.encode_utf8(&mut array[idx..]).as_bytes().len();

            idx += bytes_written;
        }

        // we should have written exactly every element of the buffer
        assert_eq!(idx, byte_len);

        // ImmutableString(array.assume_init())
        IString(array)
    }
}
