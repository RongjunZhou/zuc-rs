use crate::core::zuc::ZUC;

pub enum Algorithm {
    EEA,
    EIA,
}

impl Algorithm {
    pub fn encrypt(&self, zuc: &mut ZUC, origin: &[u32], len: u32) -> Vec<u32> {
        match self {
            Algorithm::EEA => {
                let mut rs: Vec<u32> = vec![];
                let key_length = (len + 31) / 32;
                let keys = zuc.generate_keystream(key_length as usize);
                let keys = keys.as_slice();
                for i in 0..key_length as usize {
                    rs.push(origin[i] ^ keys[i]);
                }
                if len % 32 != 0 {
                    rs[key_length as usize - 1] &= 0xffffffff << (32 - (len % 32));
                }
                rs
            }
            Algorithm::EIA => {
                #[inline]
                fn find_word(keys: &[u32], i: usize) -> u32 {
                    let j = i >> 5;
                    let m = i & 0x1f;
                    if m == 0 {
                        keys[j]
                    } else {
                        (keys[j] << m) | (keys[j + 1] >> (32 - m))
                    }
                }
                let key_length = (len + 31) / 32 + 2;
                let keys = zuc.generate_keystream(key_length as usize);
                let keys = keys.as_slice();
                let mut t = 0_u32;
                for i in 0..len as usize {
                    if origin[i >> 5] & (0x1 << (31 - (i & 0x1f))) > 0 {
                        t ^= find_word(keys, i);
                    }
                }

                t ^= find_word(keys, len as usize);
                let t = t ^ find_word(keys, 32 * (key_length - 1) as usize);
                vec![t]
            }
        }
    }
}
