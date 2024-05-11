use std::{fs::File, io::{Read, Write}};

use crate::core::algorithm::Algorithm::EEA;
use crate::core::cipher::Cipher;

trait VecU8Ext {
    fn to_block(&self) -> Vec<u32>;
}

trait VecU32Ext {
    fn to_byte(&self) -> Vec<u8>;
}

impl VecU8Ext for Vec<u8> {
    fn to_block(&self) -> Vec<u32> {
        let mut numbers = Vec::new();
        for chunk in self.chunks(4) {
            let mut num = 0;
                for &byte in chunk {
                    num = (num << 8) | byte as u32;
                }
            numbers.push(num);
        }
        return numbers;
    }
}

impl VecU32Ext for Vec<u32> {
    fn to_byte(&self) -> Vec<u8> {
        self.iter().flat_map(|&x| x.to_be_bytes().to_vec()).collect()
    }
}

pub(crate) fn encrypt_file(file_path: &str) {
    let ck: [u8; 16] = [
        0x17, 0x3d, 0x14, 0xba, 0x50, 0x03, 0x73, 0x1d, 0x7a, 0x60, 0x04, 0x94, 0x70, 0xf0, 0x0a,
        0x29,
    ];

    let count = 0x66035492_u32;
    let bearer = 0xf_u32;
    let direction = 0_u32;

    let mut file = File::open(file_path).unwrap();
    let mut content = vec![];
    file.read_to_end(&mut content).unwrap();
    let mut zuc = Cipher::new(&ck, count, bearer, direction, EEA);
    let content = content.to_block();
    let encrypted = zuc.encrypt(content.as_slice(), content.len() as u32).to_byte();
    let mut file = File::create(file_path).unwrap();
    file.write_all(&encrypted).unwrap();
}

#[cfg(test)]
mod test {
    use super::encrypt_file;

    #[test]
    fn file_test() {
        encrypt_file("src/test.txt")
    }
}