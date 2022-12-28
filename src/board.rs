#[derive(Debug)]
pub struct Board {
    bitboards: [u64; 14],
}

impl Board {
    pub fn new() -> Self {
        Self { bitboards: [0; 14] }
    }
}
