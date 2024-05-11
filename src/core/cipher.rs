use crate::core::algorithm::Algorithm;
use crate::core::zuc::ZUC;

pub struct Cipher {
    zuc: ZUC,
    algorithm: Algorithm,
}

impl Cipher {
    pub fn new(ck: &[u8], count: u32, bearer: u32, direction: u32, algorithm: Algorithm) -> Cipher {
        let mut iv = [0u8; 16];

        match algorithm {
            Algorithm::EEA => {
                iv[0] = (count >> 24) as u8;
                iv[1] = (count >> 16) as u8;
                iv[2] = (count >> 8) as u8;
                iv[3] = count as u8;
                iv[4] = (((bearer << 1) | (direction & 1)) << 2) as u8;

                iv[8] = iv[0];
                iv[9] = iv[1];
                iv[10] = iv[2];
                iv[11] = iv[3];
                iv[12] = iv[4];
            }
            Algorithm::EIA => {
                iv[0] = (count >> 24) as u8;
                iv[1] = (count >> 16) as u8;
                iv[2] = (count >> 8) as u8;
                iv[3] = count as u8;
                iv[4] = (bearer << 3) as u8;

                iv[8] = iv[0] ^ ((direction << 7) as u8);
                iv[9] = iv[1];
                iv[10] = iv[2];
                iv[11] = iv[3];
                iv[12] = iv[4];
                iv[14] = iv[6] ^ ((direction << 7) as u8);
            }
        }

        let zuc = ZUC::new(ck, &iv);
        Cipher { zuc, algorithm }
    }

    pub fn encrypt(&mut self, origin: &[u32], len: u32) -> Vec<u32> {
        self.algorithm.encrypt(&mut self.zuc, origin, len)
    }
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, Write},
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::core::{
        algorithm::Algorithm::{self, EEA},
        cipher::Cipher,
    };

    #[test]
    fn test_speed() {
        let mut str = String::new();
        for _ in 0..128 * 8 {
            str.push('a');
        }
        str.push('\n');
        let mut output = File::create("src/test.txt").unwrap();
        for _ in 0..1024 {
            output.write_all(str.as_bytes()).unwrap();
        }
        let ck: [u8; 16] = [
            0x17, 0x3d, 0x14, 0xba, 0x50, 0x03, 0x73, 0x1d, 0x7a, 0x60, 0x04, 0x94, 0x70, 0xf0,
            0x0a, 0x29,
        ];
        let count = 0x66035492_u32;
        let bearer = 0xf_u32;
        let direction = 0_u32;
        let mut eea = Cipher::new(&ck, count, bearer, direction, EEA);
        let mut times = vec![];
        for _ in 0..20 {
            let input = File::open("src/test.txt").unwrap();
            let bufferd = std::io::BufReader::new(input);
            let mut data = Vec::new();
            for line in bufferd.lines() {
                let line = line.unwrap();
                for c in line.chars() {
                    data.push(c as u32);
                }
            }
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            eea.encrypt(&data, data.len() as u32);
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                - start;
            times.push(time);
            println!("time:\t{}", time);
        }
        let mut sum = 0u128;
        let len = times.len() as u32;
        for ele in times {
            sum += ele;
        }
        println!("averge\t{}", sum / len as u128);
    }

    #[test]
    fn test_eea() {
        let ck: [u8; 16] = [
            0x17, 0x3d, 0x14, 0xba, 0x50, 0x03, 0x73, 0x1d, 0x7a, 0x60, 0x04, 0x94, 0x70, 0xf0,
            0x0a, 0x29,
        ];

        let count = 0x66035492_u32;
        let bearer = 0xf_u32;
        let direction = 0_u32;
        let length = 0xc1_u32;

        let ibs: [u32; 7] = [
            0x6cf65340, 0x735552ab, 0x0c9752fa, 0x6f9025fe, 0x0bd675d9, 0x005875b2, 0x00000000,
        ];

        let obs: [u32; 7] = [
            0xa6c85fc6, 0x6afb8533, 0xaafc2518, 0xdfe78494, 0x0ee1e4b0, 0x30238cc8, 0x00000000,
        ];

        // encrypt
        let mut eea = Cipher::new(&ck, count, bearer, direction, EEA);
        let rs = eea.encrypt(&ibs, length);
        assert_eq!(obs, rs.as_slice());

        // decrypt
        let mut eea = Cipher::new(&ck, count, bearer, direction, EEA);
        let rs = eea.encrypt(&rs, length);
        assert_eq!(ibs, rs.as_slice());
    }

    #[test]
    pub fn test_eia() {
        let ik: [u8; 16] = [
            0xc9, 0xe6, 0xce, 0xc4, 0x60, 0x7c, 0x72, 0xdb, 0x00, 0x0a, 0xef, 0xa8, 0x83, 0x85,
            0xab, 0x0a,
        ];

        let count = 0xa94059da_u32;
        let bearer = 0x0a_u32;
        let direction = 0x01_u32;
        let length = 0x0241_u32;

        let m: [u32; 19] = [
            0x983b41d4, 0x7d780c9e, 0x1ad11d7e, 0xb70391b1, 0xde0b35da, 0x2dc62f83, 0xe7b78d63,
            0x06ca0ea0, 0x7e941b7b, 0xe91348f9, 0xfcb170e2, 0x217fecd9, 0x7f9f68ad, 0xb16e5d7d,
            0x21e569d2, 0x80ed775c, 0xebde3f40, 0x93c53881, 0x00000000,
        ];

        let mac = 0xfae8ff0b_u32;

        let mut eia = Cipher::new(&ik, count, bearer, direction, Algorithm::EIA);
        let rs = eia.encrypt(&m, length);
        assert_eq!(mac, rs.as_slice()[0]);
    }
}
