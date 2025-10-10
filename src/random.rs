pub struct XorRand {
    x: u32,
}

impl XorRand {
    pub fn new(seed: u32) -> XorRand {
        XorRand {
            x: seed ^ 123456789,
        }
    }

    pub fn next01(&mut self) -> f64 {
        let mut x = self.x;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.x = x;
        (x as f64) / (std::u32::MAX as f64)
    }

    pub fn nexti(&mut self) -> u32 {
        let mut x = self.x;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.x = x;
        x
    }
}
