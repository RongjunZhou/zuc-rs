use std::{fs::File, io::{Read, Write}};
use crate::core::algorithm::Algorithm::EEA;
use crate::core::cipher::Cipher;

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
    let encrypted = zuc.encrypt(&content.clone().into_iter().map(|s|{s as u32}).collect::<Vec<u32>>(), content.len() as u32).into_iter().flat_map(|s| {s.to_be_bytes()}).collect::<Vec<u8>>();
    let mut file = File::create(file_path).unwrap();
    file.write_all(&encrypted).unwrap();
}