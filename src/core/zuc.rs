pub struct ZUC{
    s: [u32; 16],
    r1: u32,
    r2: u32,
    x: [u32; 4],
}

impl ZUC {
    pub fn new(k: &[u8], iv:[u8]) -> ZUC {
        let mut s = [0 as u32; 16];
        for i in 0..16 {

        }
    }
}