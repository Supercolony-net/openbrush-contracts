// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use const_format;
pub use sha2_const;

use sha2_const::Sha256;

pub struct ConstHasher;

impl ConstHasher {
    pub const fn u8(str: &str) -> u8 {
        let hash = Sha256::new().update(str.as_bytes()).finalize();
        hash[0]
    }

    pub const fn u16(str: &str) -> u16 {
        let hash = Sha256::new().update(str.as_bytes()).finalize();
        u16::from_le_bytes([hash[0], hash[1]])
    }

    pub const fn u32(str: &str) -> u32 {
        let hash = Sha256::new().update(str.as_bytes()).finalize();
        u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
    }

    pub const fn u64(str: &str) -> u64 {
        let hash = Sha256::new().update(str.as_bytes()).finalize();
        u64::from_le_bytes([hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]])
    }

    pub const fn u128(str: &str) -> u128 {
        let hash = Sha256::new().update(str.as_bytes()).finalize();
        u128::from_le_bytes([
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7], hash[8], hash[9], hash[10],
            hash[11], hash[12], hash[13], hash[14], hash[15],
        ])
    }

    pub const fn hash(str: &str) -> [u8; 32] {
        Sha256::new().update(str.as_bytes()).finalize()
    }
}

pub struct StorageKeyConvertor;

impl StorageKeyConvertor {
    pub const fn old_key(new_key: u32) -> [u8; 32] {
        let bytes = new_key.to_le_bytes();
        [
            bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]
    }
}
