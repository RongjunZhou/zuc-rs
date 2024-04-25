use crate::core::algorithm::Algorithm;
use crate::core::zuc::ZUC;

pub struct Cipher {
    zuc: ZUC,
    algorithm: Algorithm,
}

impl Cipher {
    pub fn new(ck: &[u8], count: u32, bearer: u32, direction: u32, algorithm: Algorithm) -> Cipher {
        let mut iv = [0u8; 16];

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

        let zuc = ZUC::new(ck, &iv);
        Cipher { zuc, algorithm }
    }

    pub fn encrypt(&mut self, origin: &[u32], len: u32) -> Vec<u32> {
        self.algorithm.encrypt(&mut self.zuc, origin, len)
    }

    pub fn decrypt(&mut self, origin: &[u32], len: u32) -> Vec<u32> {
        self.algorithm.encrypt(&mut self.zuc, origin, len)
    }
}
