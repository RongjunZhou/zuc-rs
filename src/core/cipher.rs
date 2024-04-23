use crate::core::constants::D;

struct Cipher {
    zuc: ZUC,
    algorithm: Algorithm,
}

pub struct ZUC {
    s: [u32; 16],
    r1: u32,
    r2: u32,
    x: [u32; 4],
}

impl ZUC {
    pub fn new(k: &[u8], iv: &[u8]) -> ZUC {
        let mut s = [0u32; 16];
        for i in 0..16 {
            s[i] = (k[i] as u32) << 23 | D[i] << 8 | iv[i] as u32
        }
        let mut zuc = ZUC {
            s,
            r1: 0,
            r2: 0,
            x: [0; 4],
        };
        for _ in 0..32 {}
        return zuc;
    }
}

pub enum Algorithm {
    EEA,
    EIA,
}

impl Algorithm {
    fn EEA() -> impl Fn(&str) -> str {

    }

    fn EIA() -> impl Fn(&str) -> str {

    }
}
