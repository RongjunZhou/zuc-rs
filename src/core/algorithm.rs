pub enum Algorithm {
    EEA,
    EIA,
}

impl Algorithm {
    pub(crate) fn get_algorithm(&self) -> impl Fn([u32], u32) -> Vec<u32> {
        match self {
            Algorithm::EEA => |origin, len| {
                return vec![];
            },
            Algorithm::EIA => |origin, len| {
                return vec![];
            },
        }
    }
}
