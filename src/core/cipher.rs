use crate::core::algorithm::Algorithm;
use crate::core::zuc::ZUC;

struct Cipher {
    zuc: ZUC,
    algorithm: Box<dyn Fn(&[u32], u32) -> Vec<u32>>,
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
        let algorithm = algorithm.get_algorithm();
        Cipher { zuc, algorithm }
    }

    pub fn encrypt(&self, origin: &[u32], len: u32) -> Vec<u32> {
        self.algorithm.as_ref()(origin, len)
    }

    pub fn decrypt(&self, origin: &[u32], len: u32) -> Vec<u32> {
        self.algorithm.as_ref()(origin, len)
    }
}
