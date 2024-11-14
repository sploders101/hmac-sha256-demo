use std::io::Read;

use sha2::Digest;

const BLOCK_SIZE: usize = 64;
const IPAD: u8 = 0x36;
const OPAD: u8 = 0x5C;

fn main() {
    let stdin = std::io::stdin().lock();
    let user_key = b"Key";

    let ipad_buf = [IPAD; BLOCK_SIZE];
    let opad_buf = [OPAD; BLOCK_SIZE];

    let mut key = [0u8; BLOCK_SIZE];
    for (i, byte) in user_key.iter().enumerate() {
        key[i] = *byte;
    }

    let ipad_iter = key
        .iter()
        .zip(ipad_buf.iter())
        .map(|(left, right)| left ^ right)
        .chain(stdin.bytes().map(|i| i.unwrap()));

    let mut hasher = sha2::Sha256::new();
    for byte in ipad_iter {
        hasher.update([byte]);
    }
    let hash = hasher.finalize();

    let opad_iter = key
        .iter()
        .zip(opad_buf.iter())
        .map(|(left, right)| left ^ right)
        .chain(hash.bytes().map(|i| i.unwrap()));

    let mut hasher = sha2::Sha256::new();
    for byte in opad_iter {
        hasher.update([byte]);
    }
    let hash = hasher.finalize();
    let hash = hash.as_slice();
    println!("{:02X?}", hash);
}
