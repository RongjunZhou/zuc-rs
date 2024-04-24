use crate::core::constant::{D, S0, S1};

pub struct ZUC {
    s: [u32; 16],
    r1: u32,
    r2: u32,
    x: [u32; 4],
}

impl ZUC {
    pub fn new(k: &[u8], iv: &[u8; 16]) -> ZUC {
        let mut s = [0u32; 16];
        for i in 0..16 {
            s[i] = make_u31(k[i] as u32, D[i], iv[i] as u32);
        }
        let mut zuc = ZUC {
            s,
            r1: 0,
            r2: 0,
            x: [0; 4],
        };
        for _ in 0..32 {
            zuc.bit_reconstruction();
            zuc.lfsr_with_initialization_mode(zuc.f() >> 1);
        }
        return zuc;
    }

    fn bit_reconstruction(&mut self) {
        self.x[0] = ((self.s[15] & 0x7FFF8000) << 1) | (self.s[14] & 0xFFFF);
        self.x[1] = ((self.s[11] & 0xFFFF) << 16) | (self.s[9] >> 15);
        self.x[2] = ((self.s[7] & 0xFFFF) << 16) | (self.s[5] >> 15);
        self.x[3] = ((self.s[2] & 0xFFFF) << 16) | (self.s[0] >> 15);
    }

    fn f(&mut self) -> u32 {
        let w = (self.x[0] ^ self.r1).wrapping_add(self.r2);
        let w1 = self.r1.wrapping_add(self.x[1]);
        let w2 = self.r2 ^ self.x[2];

        let u = l1((w1 << 16) | (w2 >> 16));
        let v = l2((w2 << 16) | (w1 >> 16));

        self.r1 = sbox(u);
        self.r2 = sbox(v);
        w
    }

    fn lfsr_with_initialization_mode(&mut self, u: u32) {
        let v = self.s[0];
        let v = add31(v, rot31(self.s[0], 8));
        let v = add31(v, rot31(self.s[4], 20));
        let v = add31(v, rot31(self.s[10], 21));
        let v = add31(v, rot31(self.s[13], 17));
        let v = add31(v, rot31(self.s[15], 15));

        let mut s16 = add31(v, u);

        if s16 == 0 {
            s16 = 2147483647;
        }
        for i in 0..15 {
            self.s[i] = self.s[i + 1];
        }
        self.s[15] = s16;
    }

    fn lfsr_with_work_mode(&mut self) {
        let v = self.s[0];
        let v = add31(v, rot31(self.s[0], 8));
        let v = add31(v, rot31(self.s[4], 20));
        let v = add31(v, rot31(self.s[10], 21));
        let v = add31(v, rot31(self.s[13], 17));
        let mut s16 = add31(v, rot31(self.s[15], 15));
        if s16 == 0 {
            s16 = 2147483647;
        }
        for i in 0..15 {
            self.s[i] = self.s[i + 1];
        }
        self.s[15] = s16;
    }

    fn generate_keystream(&mut self, n: usize) -> Vec<u32> {
        let mut keystream = vec![];
        for _ in 0..n {
            self.bit_reconstruction();
            let z = self.f() ^ self.x[3];
            self.lfsr_with_work_mode();
            keystream.push(z);
        }
        keystream
    }
}

#[inline]
fn make_u32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a << 24 | b << 16 | c << 8 | d
}

#[inline]
fn make_u31(k: u32, d: u32, iv: u32) -> u32 {
    k << 23 | d << 8 | iv
}

#[inline]
fn sbox(x: u32) -> u32 {
    make_u32(
        S0[(x >> 24) as usize] as u32,
        S1[((x >> 16) & 0xFF) as usize] as u32,
        S0[((x >> 8) & 0xFF) as usize] as u32,
        S1[(x & 0xFF) as usize] as u32,
    )
}

#[inline]
fn rot31(a: u32, k: u32) -> u32 {
    ((a << k) | (a >> (31 - k))) & 0x7FFFFFFF
}

#[inline]
fn add31(a: u32, b: u32) -> u32 {
    let c = a.wrapping_add(b);
    (c & 0x7FFFFFFF).wrapping_add(c >> 31)
}

fn l1(x: u32) -> u32 {
    x ^ x.rotate_left(2) ^ x.rotate_left(10) ^ x.rotate_left(18) ^ x.rotate_left(24)
}

fn l2(x: u32) -> u32 {
    x ^ x.rotate_left(8) ^ x.rotate_left(14) ^ x.rotate_left(22) ^ x.rotate_left(30)
}
