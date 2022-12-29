#[derive(Debug, Clone, Copy)]
pub struct Square(pub u32);

impl Square {
    pub fn from_an(an: &str) -> Result<Self, ()> {
        if an.len() > 2 {
            return Err(());
        }
        let file = an
            .chars()
            .nth(0)
            .and_then(|c| "ABCDEFGH".find(c.to_ascii_uppercase()));
        let rank = an
            .chars()
            .nth(1)
            .and_then(|c| if c.is_ascii_digit() { Some(c) } else { None })
            .and_then(|c| {
                let r = c.to_digit(10).unwrap();
                if 1 <= r && r <= 8 {
                    Some(r - 1)
                } else {
                    None
                }
            });

        if let (Some(r), Some(f)) = (rank, file) {
            return Ok(Self(r * 8 + f as u32));
        }
        Err(())
    }
}
